//! Flint CLI — create, run and build Flint projects.

use std::{net::SocketAddr, path::Path, process};

mod build;
mod bytecode;
mod config;
mod out;
mod site;
mod templates;
mod util;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("new") => cmd_new(&args[2..]),
        Some("serve") => cmd_serve(&args[2..]).await,
        Some("run") => cmd_run(&args[2..]).await,
        Some("build") => cmd_build(&args[2..]),
        Some("update") | Some("upgrade") => cmd_update(),
        Some("version") | Some("--version") | Some("-V") => {
            println!("flint {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            print_help();
            if matches!(
                args.get(1).map(String::as_str),
                Some("help") | Some("--help") | Some("-h") | None
            ) {
                // normal exit for help
            } else {
                process::exit(1);
            }
        }
    }
}

fn print_help() {
    println!(
        "{bold}flint{reset} {ver}

{bold}Usage:{reset}
  flint new <name>                   scaffold a new project
  flint new <name> --template tasks  scaffold with the tasks API example
  flint serve [dir]                  start the development server with hot reload
  flint run <file.flintbc>           serve a compiled bytecode file
  flint build [dir]                  compile portable bytecode into dist/
  flint build --static [dir]         export pages/*.flint.ui to static HTML
  flint update                       update the CLI to the latest release
  flint version                      print the version

{bold}Templates:{reset}
  minimal   a single GET /hello route  (default)
  tasks     GET /tasks, GET /tasks/:id, POST /tasks, showing controllers,
            services and repositories
  static    UI-only project for static HTML export",
        bold = out::BOLD,
        reset = out::RESET,
        ver = env!("CARGO_PKG_VERSION"),
    );
}

// ---------------------------------------------------------------------------
// new
// ---------------------------------------------------------------------------

fn cmd_new(args: &[String]) {
    let name = match args.iter().find(|a| !a.starts_with('-')) {
        Some(n) => n.clone(),
        None => {
            out::error("missing project name");
            eprintln!("  usage: flint new <name> [--template minimal|tasks]");
            process::exit(1);
        }
    };
    config::validate_project_name(&name).unwrap_or_else(|e| {
        out::error(e);
        process::exit(1);
    });

    let template = args
        .windows(2)
        .find(|w| w[0] == "--template" || w[0] == "-t")
        .map(|w| w[1].as_str())
        .unwrap_or("minimal");

    let root = Path::new(&name);
    if root.exists() {
        out::error(format!("directory '{name}' already exists"));
        process::exit(1);
    }

    let files = match template {
        "minimal" => templates::minimal(&name),
        "tasks" => templates::tasks(&name),
        "static" => templates::site(&name),
        other => {
            out::error(format!("unknown template '{other}'"));
            eprintln!("  available: minimal, tasks, static");
            process::exit(1);
        }
    };

    for (rel, content) in &files {
        let full = root.join(rel);
        if let Some(p) = full.parent() {
            std::fs::create_dir_all(p).unwrap_or_else(|e| {
                out::error(format!("could not create '{}': {e}", p.display()));
                process::exit(1);
            });
        }
        std::fs::write(&full, content).unwrap_or_else(|e| {
            out::error(format!("could not write '{}': {e}", full.display()));
            process::exit(1);
        });
        out::created(format!("{}/{}", name, rel.display()));
    }

    println!();
    println!("  get started:");
    println!("    {}cd {name}{}", out::BOLD, out::RESET);
    if template == "static" {
        println!("    {}flint build --static{}", out::BOLD, out::RESET);
    } else {
        println!("    {}flint serve{}", out::BOLD, out::RESET);
    }
}

// ---------------------------------------------------------------------------
// serve (source + hot reload)
// ---------------------------------------------------------------------------

async fn cmd_serve(args: &[String]) {
    let dir = resolve_project_dir(args);
    if bytecode::is_bytecode_path(&dir) {
        out::error("'flint serve' only works with source projects; use 'flint run <file.flintbc>' for bytecode");
        process::exit(1);
    }

    let config = load_config(&dir);
    let routes_dir = dir.join(&config.server.routes);
    let pages_dir = dir.join(&config.server.pages);

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .unwrap_or_else(|_| {
            out::error(format!(
                "invalid address {}:{}",
                config.server.host, config.server.port
            ));
            process::exit(1);
        });

    flint::log::set(
        config
            .server
            .log
            .parse()
            .unwrap_or(flint::log::LogLevel::Info),
    );

    // Directories and files that trigger a reload when modified.
    let watch_paths = vec![
        dir.join(&config.server.routes),
        dir.join(&config.server.pages),
        dir.join(&config.server.services),
        dir.join(&config.server.repositories),
        dir.join(&config.server.components),
        dir.join("flint.toml"),
    ];

    println!(
        "  {bold}{}{reset} v{}  →  {bold}http://{addr}{reset}",
        config.project.name,
        config.project.version,
        bold = out::BOLD,
        reset = out::RESET,
    );

    loop {
        let modules = match load_source_modules(&routes_dir, &pages_dir, &dir) {
            Ok(m) => m,
            Err(e) => {
                out::error(e);
                wait_for_change(&watch_paths).await;
                out::step("watch", "change detected, reloading");
                continue;
            }
        };

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let watch_clone = watch_paths.clone();
        let watcher = tokio::spawn(async move {
            wait_for_change(&watch_clone).await;
            let _ = shutdown_tx.send(());
        });

        let result = flint::http::serve_with_shutdown(modules, addr, async move {
            shutdown_rx.await.ok();
        })
        .await;

        watcher.abort();

        match result {
            Ok(()) => {
                out::step("watch", "change detected, reloading");
                // Brief pause so the OS releases the port before we rebind.
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                // Port still held; retry after a short delay.
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) if e.kind() == std::io::ErrorKind::InvalidInput => {
                // Router build failure (duplicate routes, etc.).
                out::error(format!("{e}"));
                wait_for_change(&watch_paths).await;
                out::step("watch", "change detected, reloading");
            }
            Err(e) => {
                out::error(format!("server: {e}"));
                process::exit(1);
            }
        }
    }
}

fn load_source_modules(
    routes_dir: &Path,
    pages_dir: &Path,
    project_root: &Path,
) -> Result<Vec<flint::lang::AppModule>, String> {
    let mut modules = if routes_dir.exists() {
        flint::lang::load_app_dir(routes_dir, project_root).map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    let mut pages =
        flint::lang::load_pages_dir(pages_dir, project_root).map_err(|e| e.to_string())?;
    modules.append(&mut pages);
    Ok(modules)
}

/// Polls the mtime of every `.fl` and `.flint.ui` file (and `flint.toml`)
/// under the given paths, returning when any of them changes.
async fn wait_for_change(watch_paths: &[std::path::PathBuf]) {
    let initial = source_mtime(watch_paths);
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        if source_mtime(watch_paths) != initial {
            return;
        }
    }
}

fn source_mtime(paths: &[std::path::PathBuf]) -> u64 {
    let mut max = 0u64;
    let mut stack: Vec<std::path::PathBuf> = paths.to_vec();
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&p) {
                for entry in entries.flatten() {
                    stack.push(entry.path());
                }
            }
        } else if is_watched_file(&p) {
            if let Ok(meta) = p.metadata() {
                if let Ok(mtime) = meta.modified() {
                    let ms = mtime
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;
                    max = max.max(ms);
                }
            }
        }
    }
    max
}

fn is_watched_file(path: &std::path::Path) -> bool {
    if path.file_name().and_then(|n| n.to_str()) == Some("flint.toml") {
        return true;
    }
    let s = path.to_str().unwrap_or("");
    s.ends_with(".fl") || s.ends_with(".flint.ui")
}

// ---------------------------------------------------------------------------
// run (bytecode only, no hot reload)
// ---------------------------------------------------------------------------

async fn cmd_run(args: &[String]) {
    let path = resolve_project_dir(args);
    if !bytecode::is_bytecode_path(&path) {
        out::error("'flint run' only accepts .flintbc files; use 'flint serve' to run a source project");
        process::exit(1);
    }
    serve_bytecode(&path).await;
}

async fn serve_bytecode(path: &Path) {
    let project = bytecode::read_project(path).unwrap_or_else(|e| {
        out::error(e);
        process::exit(1);
    });

    let addr: SocketAddr = std::env::var("FLINT_ADDR")
        .or_else(|_| std::env::var("ASMB_ADDR"))
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .unwrap_or_else(|_| {
            out::error("invalid FLINT_ADDR");
            process::exit(1);
        });

    let log_level = std::env::var("FLINT_LOG").unwrap_or_else(|_| "info".to_string());
    flint::log::set(log_level.parse().unwrap_or(flint::log::LogLevel::Info));
    if let Err(e) = flint::http::serve_with_ready(project.modules, addr, |addr| {
        println!(
            "  {bold}{}{reset} v{}  ->  {bold}http://{addr}{reset}",
            project.name,
            project.version,
            bold = out::BOLD,
            reset = out::RESET,
        );
    })
    .await
    {
        out::error(format!("server: {e}"));
        process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// build
// ---------------------------------------------------------------------------

fn cmd_build(args: &[String]) {
    let dir = resolve_project_dir(args);
    let config = load_config(&dir);
    let routes_dir = dir.join(&config.server.routes);
    let pages_dir = dir.join(&config.server.pages);

    if args.iter().any(|arg| arg == "--static") {
        site::run(&dir, &pages_dir, &config.project.name).unwrap_or_else(|e| {
            out::error(e);
            process::exit(1);
        });
        return;
    }

    build::run(
        &dir,
        &routes_dir,
        &pages_dir,
        &config.project.name,
        &config.project.version,
    )
    .unwrap_or_else(|e| {
        out::error(e);
        process::exit(1);
    });
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

fn cmd_update() {
    let current = env!("CARGO_PKG_VERSION");

    let output = std::process::Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--location",
            "--header",
            "Accept: application/vnd.github+json",
            "--header",
            "X-GitHub-Api-Version: 2022-11-28",
            "https://api.github.com/repos/MateusGX/flint/releases/latest",
        ])
        .output()
        .unwrap_or_else(|_| {
            out::error("curl is required for flint update");
            process::exit(1);
        });

    if !output.status.success() {
        out::error("could not reach GitHub releases — check your connection");
        process::exit(1);
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        out::error("unexpected response from GitHub API");
        process::exit(1);
    });

    let latest = json["tag_name"]
        .as_str()
        .unwrap_or("")
        .trim_start_matches('v');

    if latest.is_empty() {
        out::error("could not determine latest version");
        process::exit(1);
    }

    println!(
        "  {DIM}current{RESET}  {current}",
        DIM = out::DIM,
        RESET = out::RESET
    );
    println!(
        "  {DIM}latest{RESET}   {latest}",
        DIM = out::DIM,
        RESET = out::RESET
    );
    println!();

    if current == latest {
        println!("  Already up to date.");
        return;
    }

    println!(
        "  Updating to {bold}{latest}{reset}...",
        bold = out::BOLD,
        reset = out::RESET
    );
    println!();

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://flint.devlayer.app/install.sh | sh")
        .status()
        .unwrap_or_else(|_| {
            out::error("failed to launch the update script");
            process::exit(1);
        });

    if !status.success() {
        out::error("update failed");
        process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// shared helpers
// ---------------------------------------------------------------------------

fn resolve_project_dir(args: &[String]) -> std::path::PathBuf {
    let dir = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(String::as_str)
        .unwrap_or(".");
    Path::new(dir).to_path_buf()
}

fn load_config(dir: &Path) -> config::Config {
    let path = dir.join("flint.toml");
    config::load(&path).unwrap_or_else(|e| {
        out::error(e);
        process::exit(1);
    })
}

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
        Some("serve") | Some("run") => cmd_serve(&args[2..]).await,
        Some("build") => cmd_build(&args[2..]),
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
  flint serve [dir|file.flintbc]     start the development server
  flint build [dir]                  compile portable bytecode into dist/
  flint build --static [dir]         export app/*.flint.ui to static HTML
  flint version                      print the version

{bold}Templates:{reset}
  minimal   a single GET /hello route  (default)
  tasks     GET /tasks, GET /tasks/:id, POST /tasks, showing controllers,
            services and repositories
  static    UI-only project for static HTML export

{bold}Project file (flint.toml):{reset}
  [project]
  name    = \"my-app\"
  version = \"0.1.0\"

  [server]
  host   = \"127.0.0.1\"
  port   = 3000
  routes = \"api\"
  pages  = \"app\"",
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
// serve
// ---------------------------------------------------------------------------

async fn cmd_serve(args: &[String]) {
    let dir = resolve_project_dir(args);
    if bytecode::is_bytecode_path(&dir) {
        serve_bytecode(&dir).await;
        return;
    }

    let config = load_config(&dir);
    let routes_dir = dir.join(&config.server.routes);
    let pages_dir = dir.join(&config.server.pages);

    let mut modules = if routes_dir.exists() {
        flint::lang::load_app_dir(&routes_dir, &dir).unwrap_or_else(|e| {
            out::error(format!(
                "failed to load routes from '{}': {e}",
                routes_dir.display()
            ));
            process::exit(1);
        })
    } else {
        Vec::new()
    };
    let mut page_modules = flint::lang::load_pages_dir(&pages_dir, &dir).unwrap_or_else(|e| {
        out::error(format!(
            "failed to load pages from '{}': {e}",
            pages_dir.display()
        ));
        process::exit(1);
    });
    modules.append(&mut page_modules);

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .unwrap_or_else(|_| {
            out::error(format!(
                "invalid address {}:{}",
                config.server.host, config.server.port
            ));
            process::exit(1);
        });

    flint::log::set(flint::log::LogLevel::from_str(&config.server.log));
    if let Err(e) = flint::http::serve_with_ready(modules, addr, |addr| {
        println!(
            "  {bold}{}{reset} v{}  →  {bold}http://{addr}{reset}",
            config.project.name,
            config.project.version,
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
    flint::log::set(flint::log::LogLevel::from_str(&log_level));
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

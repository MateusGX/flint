//! `flint build` — compiles a project's `.fl` routes into a standalone
//! release binary by generating a small Rust shim, then invoking `cargo
//! build --release` on it.
//!
//! The shim embeds all route sources as string literals so the resulting
//! binary is fully self-contained and needs neither the Flint CLI nor the route
//! files at runtime.
//!
//! Layout produced under `.flint-build/`:
//!
//! ```text
//! .flint-build/
//! ├── Cargo.toml   — generated; depends on the Flint library
//! └── src/
//!     └── main.rs  — generated; embeds every .fl file
//! ```

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::out;

/// Path to the Flint library crate, derived from the CLI crate's location.
fn flint_lib_path() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|crates_dir| crates_dir.join("flint"))
        .ok_or_else(|| "cannot resolve Flint library crate path".to_string())
}

pub fn run(
    project_dir: &Path,
    routes_dir: &Path,
    pages_dir: &Path,
    name: &str,
    version: &str,
) -> Result<(), String> {
    // Collect route files and generated page modules.
    let mut routes: Vec<(String, String)> = std::fs::read_dir(routes_dir)
        .map_err(|e| format!("cannot read routes dir '{}': {e}", routes_dir.display()))?
        .map(|entry| {
            entry.map_err(|e| format!("cannot read route entry in '{}': {e}", routes_dir.display()))
        })
        .filter_map(|entry| match entry {
            Ok(entry) if entry.path().extension().is_some_and(|x| x == "fl") => Some(Ok(entry)),
            Ok(_) => None,
            Err(err) => Some(Err(err)),
        })
        .map(|e| {
            let path = e?.path();
            let fname = path.file_name().unwrap().to_string_lossy().into_owned();
            let source = std::fs::read_to_string(&path)
                .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;
            // Expand `use` directives so the embedded source is self-contained.
            let content = flint::lang::expand_source(&source, project_dir)
                .map_err(|e| format!("preprocessing '{fname}': {e}"))?;
            Ok((fname, content))
        })
        .collect::<Result<Vec<_>, String>>()?;

    for path in collect_page_paths(pages_dir)? {
        let fname = path
            .strip_prefix(pages_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let source = std::fs::read_to_string(&path)
            .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;
        let generated = flint::lang::compile_page_source(&source, &path, pages_dir)
            .map_err(|e| format!("page '{fname}': {e}"))?;
        let content = flint::lang::expand_source(&generated.source, project_dir)
            .map_err(|e| format!("preprocessing page '{fname}': {e}"))?;
        routes.push((format!("page:{fname}"), content));
    }

    routes.sort_by(|a, b| a.0.cmp(&b.0));

    if routes.is_empty() {
        return Err(format!("no .fl files found in '{}'", routes_dir.display()));
    }

    validate_embedded_routes(&routes)?;

    for (fname, _) in &routes {
        out::step("embedding", fname);
    }

    // Generate the build directory.
    let build_dir = project_dir.join(".flint-build");
    std::fs::create_dir_all(build_dir.join("src"))
        .map_err(|e| format!("cannot create build dir: {e}"))?;

    // Write Cargo.toml.
    let cargo_toml = generate_cargo_toml(name, version);
    std::fs::write(build_dir.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("cannot write Cargo.toml: {e}"))?;

    // Write main.rs.
    let main_rs = generate_main(&routes);
    std::fs::write(build_dir.join("src/main.rs"), main_rs)
        .map_err(|e| format!("cannot write main.rs: {e}"))?;

    // Run cargo build --release.
    out::step("compiling", format!("{name} v{version}"));
    let status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&build_dir)
        .status()
        .map_err(|e| format!("cargo not found: {e}"))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    // Copy binary to dist/.
    let dist_dir = project_dir.join("dist");
    std::fs::create_dir_all(&dist_dir).map_err(|e| format!("cannot create dist/: {e}"))?;

    let bin_name = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };

    let src_bin = build_dir.join("target/release").join(&bin_name);
    let dst_bin = dist_dir.join(&bin_name);

    std::fs::copy(&src_bin, &dst_bin).map_err(|e| format!("cannot copy binary to dist/: {e}"))?;

    out::done("ready", format!("dist/{bin_name}"));
    Ok(())
}

fn collect_page_paths(pages_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    if !pages_dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    collect_page_paths_rec(pages_dir, &mut paths)
        .map_err(|e| format!("cannot read pages dir '{}': {e}", pages_dir.display()))?;
    paths.sort();
    Ok(paths)
}

fn collect_page_paths_rec(dir: &Path, paths: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_page_paths_rec(&path, paths)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".flint.html") || name.ends_with(".flint.ui"))
        {
            paths.push(path);
        }
    }
    Ok(())
}

fn generate_cargo_toml(name: &str, version: &str) -> String {
    let flint_dependency = flint_dependency();
    let name = toml_string(name);
    let version = toml_string(version);
    format!(
        r#"[package]
name = {name}
version = {version}
edition = "2021"

[[bin]]
name = {name}
path = "src/main.rs"

[dependencies]
flint     = {{ {flint_dependency} }}
tokio    = {{ version = "1", features = ["rt-multi-thread", "macros"] }}
tracing-subscriber = "0.3"
"#
    )
}

fn validate_embedded_routes(routes: &[(String, String)]) -> Result<(), String> {
    let mut modules = Vec::with_capacity(routes.len());
    for (name, source) in routes {
        let app = flint::lang::compile_app_source(source)
            .map_err(|e| format!("compiling embedded route '{name}': {e}"))?;
        modules.push(flint::lang::AppModule {
            program: Arc::new(app.program),
            routes: app.routes,
            source_path: PathBuf::from(name),
        });
    }
    let _ = flint::http::try_router(modules).map_err(|e| format!("validating routes: {e}"))?;
    Ok(())
}

fn flint_dependency() -> String {
    if let Ok(path) = std::env::var("FLINT_LIB_PATH") {
        return format!("path = {}", toml_string(&path.replace('\\', "/")));
    }
    if let Ok(path) = flint_lib_path() {
        if path.join("Cargo.toml").exists() {
            let path = path.to_string_lossy().replace('\\', "/");
            return format!("path = {}", toml_string(&path));
        }
    }
    format!("version = {}", toml_string(env!("CARGO_PKG_VERSION")))
}

fn generate_main(routes: &[(String, String)]) -> String {
    // Build the &[(&str, &str)] literal — each route is (file_name, source).
    let routes_literal: String = routes
        .iter()
        .map(|(name, src)| {
            // Escape the source content so it can live in a raw string.
            // Use the largest number of # that doesn't appear in the source.
            let hashes = pick_raw_hashes(src);
            let open = format!("r{hashes}\"");
            let close = format!("\"{hashes}");
            format!("    ({}, {open}{src}{close}),\n", rust_string_literal(name))
        })
        .collect();

    format!(
        r#"use std::net::SocketAddr;
use flint::{{lang, http}};

const ROUTES: &[(&str, &str)] = &[
{routes_literal}];

#[tokio::main]
async fn main() {{
    if let Err(error) = run().await {{
        eprintln!("{{error}}");
        std::process::exit(1);
    }}
}}

async fn run() -> Result<(), String> {{
    tracing_subscriber::fmt::init();

    let modules: Vec<_> = ROUTES
        .iter()
        .map(|(name, src)| {{
            let app = lang::compile_app_source(src)
                .map_err(|e| format!("{{name}}: {{e}}"))?;
            Ok(flint::lang::AppModule {{
                program: std::sync::Arc::new(app.program),
                routes: app.routes,
                source_path: std::path::PathBuf::from(name),
            }})
        }})
        .collect::<Result<_, String>>()?;

    let addr: SocketAddr = std::env::var("FLINT_ADDR")
        .or_else(|_| std::env::var("ASMB_ADDR"))
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .map_err(|e| format!("invalid FLINT_ADDR: {{e}}"))?;

    http::serve_with_ready(modules, addr, |addr| println!("listening on http://{{addr}}"))
        .await
        .map_err(|e| format!("server error: {{e}}"))?;
    Ok(())
}}
"#
    )
}

/// Returns the raw-string delimiter (`###...`) that safely wraps `s`,
/// choosing the minimum number of `#` characters needed.
fn pick_raw_hashes(s: &str) -> String {
    let mut n = 0usize;
    loop {
        let close = format!("\"{}", "#".repeat(n));
        if !s.contains(&close) {
            return "#".repeat(n);
        }
        n += 1;
    }
}

fn toml_string(value: &str) -> String {
    format!("\"{}\"", escape_common_string(value))
}

fn rust_string_literal(value: &str) -> String {
    format!("\"{}\"", escape_common_string(value))
}

fn escape_common_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::validate_embedded_routes;

    #[test]
    fn validates_embedded_routes_before_building_binary() {
        let routes = vec![
            (
                "a.fl".to_string(),
                "a:\n  ret\nroute GET \"/dupe\" -> a\n".to_string(),
            ),
            (
                "b.fl".to_string(),
                "b:\n  ret\nroute GET \"/dupe\" -> b\n".to_string(),
            ),
        ];

        let err = validate_embedded_routes(&routes).unwrap_err();
        assert!(err.contains("duplicate route GET /dupe"), "{err}");
    }
}

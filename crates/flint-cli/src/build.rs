//! `flint build` — compiles a project to portable Flint bytecode.

use std::path::Path;

use crate::bytecode;
use crate::out;

pub fn run(
    project_dir: &Path,
    routes_dir: &Path,
    pages_dir: &Path,
    name: &str,
    version: &str,
) -> Result<(), String> {
    let mut modules = if routes_dir.exists() {
        flint::lang::load_app_dir(routes_dir, project_dir)
            .map_err(|e| format!("failed to load routes from '{}': {e}", routes_dir.display()))?
    } else {
        Vec::new()
    };
    let mut page_modules = flint::lang::load_pages_dir(pages_dir, project_dir)
        .map_err(|e| format!("failed to load pages from '{}': {e}", pages_dir.display()))?;
    modules.append(&mut page_modules);

    if modules.is_empty() {
        return Err(format!(
            "no .fl route files found in '{}' and no .flint.ui pages found in '{}'",
            routes_dir.display(),
            pages_dir.display()
        ));
    }

    validate_modules(&modules)?;

    for module in &modules {
        out::step("bytecode", module.source_path.display());
    }

    let dist_dir = project_dir.join("dist");
    std::fs::create_dir_all(&dist_dir).map_err(|e| format!("cannot create dist/: {e}"))?;
    let output_name = bytecode::file_name(name);
    let output_path = dist_dir.join(&output_name);

    out::step("writing", format!("{name} v{version}"));
    bytecode::write_project(&output_path, name, version, &modules)?;

    out::done("ready", format!("dist/{output_name}"));
    Ok(())
}

fn validate_modules(modules: &[flint::lang::AppModule]) -> Result<(), String> {
    let _ = flint::http::router(modules.to_vec()).map_err(|e| format!("validating routes: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;

    use super::validate_modules;

    #[test]
    fn validates_routes_before_writing_bytecode() {
        let modules = ["a", "b"]
            .into_iter()
            .map(|name| {
                let source =
                    format!("section .route\n    GET \"/dupe\" -> {name}\n\nsection .text\n{name}:\n    ret\n");
                let app = flint::lang::compile_app_source(&source).unwrap();
                flint::lang::AppModule {
                    program: Arc::new(app.program),
                    routes: app.routes,
                    source_path: PathBuf::from(format!("{name}.fl")),
                }
            })
            .collect::<Vec<_>>();

        let err = validate_modules(&modules).unwrap_err();
        assert!(err.contains("duplicate route GET /dupe"), "{err}");
    }
}

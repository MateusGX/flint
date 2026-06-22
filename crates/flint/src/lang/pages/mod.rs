//! Page templates for Flint apps.
//!
//! Pages live under `pages/` as `.flint.ui` files. Each file must use the
//! section-based format:
//!
//! ```text
//! section .route
//!     GET "/path"
//!
//! section .data
//!     label  db "string value"
//!
//! section .render
//!     window "Title"
//!       card "Card"
//!         field "Nome", r1
//!       end
//!     end
//! ```
//!
//! The compiler produces an ordinary `.fl` route module, keeping the VM and
//! bytecode format unchanged.

use std::fmt;
use std::fs;
use std::path::Path;

use crate::lang::{app, compiler::CompileError, AppModule, LoadError};

mod blocks;
mod compiler;
mod emit;
mod parser;
mod paths;
mod render;
mod source;

#[derive(Debug, Clone, PartialEq)]
pub struct PageCompileError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for PageCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for PageCompileError {}

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledPageSource {
    pub route_path: String,
    pub method: String,
    pub source: String,
}

impl CompiledPageSource {
    /// Compiles the generated Flint source into a [`crate::lang::CompiledApp`].
    ///
    /// `project_root` is needed to resolve any `use "..."` directives that
    /// were included via `@use` in the original `.flint.ui` file.
    pub fn compile(
        &self,
        project_root: &Path,
    ) -> Result<crate::lang::CompiledApp, crate::lang::Error> {
        use crate::lang::preprocessor;

        preprocessor::validate_sections(&self.source).map_err(|msg| {
            crate::lang::Error::Compile(CompileError {
                line: 1,
                message: msg,
            })
        })?;
        let normalized = preprocessor::normalize_sections(&self.source);
        let expanded = preprocessor::expand(&normalized, project_root).map_err(|e| {
            crate::lang::Error::Compile(CompileError {
                line: 1,
                message: e.to_string(),
            })
        })?;
        crate::lang::compile_app_source_raw(&expanded)
    }
}

/// Compiles one `.flint.ui` page into ordinary Flint source.
///
/// The source must use the section-based format (`section .route`, …,
/// `section .render`).
pub fn compile_page_source(
    source: &str,
    page_path: impl AsRef<Path>,
    pages_dir: impl AsRef<Path>,
) -> Result<CompiledPageSource, PageCompileError> {
    compiler::compile(source, page_path.as_ref(), pages_dir.as_ref())
}

/// Compiles every `.flint.ui` page under `pages_dir` into independent app
/// modules, matching how route `.fl` files are loaded.
pub fn load_pages_dir(
    pages_dir: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
) -> Result<Vec<AppModule>, LoadError> {
    let dir = pages_dir.as_ref();
    let root = project_root.as_ref();

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    paths::collect_page_paths(dir, &mut paths).map_err(|source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    paths.sort();

    let mut modules = Vec::with_capacity(paths.len());
    for path in paths {
        let source = fs::read_to_string(&path).map_err(|source| LoadError::Io {
            path: path.clone(),
            source,
        })?;
        let generated = compile_page_source(&source, &path, dir).map_err(|e| LoadError::Page {
            path: path.clone(),
            message: e.to_string(),
        })?;
        modules.push(app::compile_module_source(path, &generated.source, root)?);
    }

    Ok(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section_source(route: &str, render: &str) -> String {
        format!("section .route\n    {route}\n\nsection .render\n{render}\n")
    }

    #[test]
    fn compiles_section_page_to_flint_source() {
        let source = section_source(
            "GET \"/hello\"",
            "    window \"Hello\"\n        text \"World\"\n    end\n",
        );
        let page = compile_page_source(&source, "pages/hello.flint.ui", "pages").unwrap();
        assert_eq!(page.method, "GET");
        assert_eq!(page.route_path, "/hello");
        assert!(page.source.contains("section .route"), "{}", page.source);
        assert!(
            page.source.contains("GET \"/hello\" -> __page_hello"),
            "{}",
            page.source
        );
        assert!(page.source.contains("section .text"), "{}", page.source);
        assert!(page.source.contains("mov r14, \"\""), "{}", page.source);
        assert!(
            page.source.contains("ncallr r14, ui.window,"),
            "{}",
            page.source
        );
        assert!(
            page.source.contains("ncall http.html, r14"),
            "{}",
            page.source
        );
    }

    #[test]
    fn rejects_legacy_page_format() {
        let source = "@page \"/hello\"\n<h1>Hello</h1>\n";
        let err = compile_page_source(source, "pages/hello.flint.ui", "pages").unwrap_err();
        assert!(err.message.contains("section"), "{}", err.message);
    }

    #[test]
    fn infers_routes_from_page_paths() {
        let empty = "section .route\n\nsection .render\n";

        let page = compile_page_source(empty, "pages/users/[id].flint.ui", "pages").unwrap();
        assert_eq!(page.route_path, "/users/:id");

        let page = compile_page_source(empty, "pages/blog/index.flint.ui", "pages").unwrap();
        assert_eq!(page.route_path, "/blog");

        let page = compile_page_source(empty, "pages/index.flint.ui", "pages").unwrap();
        assert_eq!(page.route_path, "/");

        let page = compile_page_source(empty, "pages/admin/index.flint.ui", "pages").unwrap();
        assert_eq!(page.route_path, "/admin");
    }

    #[test]
    fn data_section_inlines_string_values() {
        let source = "section .route\n    GET \"/\"\n\nsection .data\n    greeting  db \"Hello, World!\"\n\nsection .render\n    text greeting\n";
        let page = compile_page_source(source, "pages/index.flint.ui", "pages").unwrap();
        assert!(
            page.source.contains("mov r15, \"Hello, World!\""),
            "{}",
            page.source
        );
        assert!(
            page.source.contains("ncallr r14, ui.text,"),
            "{}",
            page.source
        );
    }

    #[test]
    fn use_directives_are_emitted_before_handler() {
        let source =
            "@use \"services/site.fl\"\n\nsection .route\n    GET \"/\"\n\nsection .render\n";
        let page = compile_page_source(source, "pages/index.flint.ui", "pages").unwrap();
        assert!(
            page.source.contains("use \"services/site.fl\""),
            "{}",
            page.source
        );
    }
}

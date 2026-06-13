//! Page templates for Flint apps.
//!
//! Pages live under `pages/` as `.flint.html` or `.flint.ui` files. Both
//! suffixes are compiled identically — preamble directives, `<% %>` code
//! blocks, `<%= %>` expressions, and raw HTML are all handled by
//! [`html::compile`] — into ordinary `.fl` route modules before the existing
//! lexer/parser/compiler pipeline runs, which keeps the VM and bytecode
//! format unchanged.
//!
//! By convention, `.flint.ui` pages build their body entirely from `<% %>`
//! blocks that call the `ui.*` stdlib natives (e.g. `ncallr r14, ui.window,
//! r14, "Title"`) to append Flint's default styled HTML to the `r14`
//! accumulator, instead of writing raw HTML/CSS.

use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::lang::{AppModule, LoadError};

mod html;
mod shared;

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

/// Compiles one page template into ordinary Flint source.
pub fn compile_page_source(
    source: &str,
    page_path: impl AsRef<Path>,
    pages_dir: impl AsRef<Path>,
) -> Result<CompiledPageSource, PageCompileError> {
    html::compile(source, page_path.as_ref(), pages_dir)
}

/// Compiles every supported page under `pages_dir` into independent app
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
    shared::collect_page_paths(dir, &mut paths).map_err(|source| LoadError::Io {
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
        let expanded = crate::lang::preprocessor::expand(&generated.source, root).map_err(|e| {
            LoadError::Include {
                path: path.clone(),
                message: e.to_string(),
            }
        })?;
        let app =
            crate::lang::compile_app_source(&expanded).map_err(|source| LoadError::Compile {
                path: path.clone(),
                source,
            })?;
        modules.push(AppModule {
            program: Arc::new(app.program),
            routes: app.routes,
            source_path: path,
        });
    }

    Ok(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_html_and_expressions_to_flint_source() {
        let source = r#"@page "/hello"
@use "services/site.fl"
<h1>Hello <%= r2 %></h1>
<%
mov r0, "name"
ncallr r2, http.query, r0
%>
"#;
        let page = compile_page_source(source, "pages/index.flint.html", "pages").unwrap();
        assert_eq!(page.method, "GET");
        assert_eq!(page.route_path, "/hello");
        assert!(page.source.contains("use \"services/site.fl\""));
        assert!(page.source.contains("mov r15, r2"));
        assert!(page.source.contains("ncallr r15, string.from, r15"));
        assert!(page.source.contains("ncall http.html, r14"));
        assert!(page.source.contains("route GET \"/hello\" -> __page_index"));
    }

    #[test]
    fn infers_routes_from_page_paths() {
        let page = compile_page_source("", "pages/users/[id].flint.html", "pages").unwrap();
        assert_eq!(page.route_path, "/users/:id");

        let page = compile_page_source("", "pages/blog/index.flint.html", "pages").unwrap();
        assert_eq!(page.route_path, "/blog");

        let page = compile_page_source("", "pages/index.flint.html", "pages").unwrap();
        assert_eq!(page.route_path, "/");

        let page = compile_page_source("", "pages/admin/index.flint.ui", "pages").unwrap();
        assert_eq!(page.route_path, "/admin");
    }

    #[test]
    fn compiles_ui_pages_through_the_html_pipeline() {
        let source = r#"@page "/dashboard"
<%
mov r1, "Ada"
mov r15, "Dashboard"
ncallr r14, ui.window, r14, r15
mov r15, "Welcome"
ncallr r14, ui.text, r14, r15
mov r15, "Profile"
ncallr r14, ui.card, r14, r15
mov r15, "Name"
ncallr r14, ui.field, r14, r15, r1
mov r15, "API"
mov r2, "/hello"
ncallr r14, ui.button, r14, r15, r2
ncallr r14, ui.card_end, r14
ncallr r14, ui.window_end, r14
%>
"#;
        let page = compile_page_source(source, "pages/dashboard.flint.ui", "pages").unwrap();

        assert_eq!(page.method, "GET");
        assert_eq!(page.route_path, "/dashboard");
        assert!(page.source.contains("mov r14, \"\""));
        assert!(page.source.contains("mov r1, \"Ada\""));
        assert!(page.source.contains("ncallr r14, ui.window, r14, r15"));
        assert!(page.source.contains("ncallr r14, ui.window_end, r14"));
        assert!(page.source.contains("ncall http.html, r14"));
        assert!(page
            .source
            .contains("route GET \"/dashboard\" -> __page_dashboard"));
    }
}

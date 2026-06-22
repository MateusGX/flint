use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::out;

const CSS_ASSET: &str = "flint.css";
const JS_ASSET: &str = "flint.js";

struct StaticPage {
    output: PathBuf,
    html: String,
}

pub fn run(project_dir: &Path, pages_dir: &Path, name: &str) -> Result<(), String> {
    if !pages_dir.exists() {
        return Err(format!(
            "cannot export static site: pages dir '{}' does not exist",
            pages_dir.display()
        ));
    }

    let modules = flint::lang::load_pages_dir(pages_dir, project_dir)
        .map_err(|e| format!("failed to load pages from '{}': {e}", pages_dir.display()))?;
    if modules.is_empty() {
        return Err(format!(
            "no .flint.ui pages found in '{}'",
            pages_dir.display()
        ));
    }

    let dist_dir = project_dir.join("dist");
    if dist_dir.exists() {
        std::fs::remove_dir_all(&dist_dir).map_err(|e| format!("cannot clean dist/: {e}"))?;
    }
    std::fs::create_dir_all(&dist_dir).map_err(|e| format!("cannot create dist/: {e}"))?;

    // Write shared asset files directly from the built-in constants.
    let css_path = dist_dir.join(CSS_ASSET);
    std::fs::write(&css_path, flint::UI_CSS)
        .map_err(|e| format!("cannot write '{}': {e}", css_path.display()))?;
    out::step("asset", "dist/flint.css");

    let js_path = dist_dir.join(JS_ASSET);
    std::fs::write(&js_path, js_content_from_constant())
        .map_err(|e| format!("cannot write '{}': {e}", js_path.display()))?;
    out::step("asset", "dist/flint.js");

    let mut written = HashSet::new();
    let mut pages = Vec::new();

    for module in modules {
        reject_request_natives(&module)?;
        for route in &module.routes {
            if route.method != "GET" {
                return Err(format!(
                    "cannot export {} {} from '{}': static export supports GET pages only",
                    route.method,
                    route.path,
                    module.source_path.display()
                ));
            }
            if route
                .path
                .split('/')
                .any(|segment| segment.starts_with(':'))
            {
                return Err(format!(
                    "cannot export dynamic route {} from '{}': provide a concrete static page path",
                    route.path,
                    module.source_path.display()
                ));
            }

            let output = static_output_path(&dist_dir, &route.path)?;
            if !written.insert(output.clone()) {
                return Err(format!(
                    "multiple pages export to '{}'",
                    output
                        .strip_prefix(project_dir)
                        .unwrap_or(&output)
                        .display()
                ));
            }

            let html = flint::http::render_static_html(
                Arc::clone(&module.program),
                route.handler_address,
                &route.path,
            )
            .map_err(|e| {
                format!(
                    "rendering static page {} from '{}': {e}",
                    route.path,
                    module.source_path.display()
                )
            })?;

            let css_href = relative_asset_path(&output, &dist_dir, CSS_ASSET)?;
            let js_src = relative_asset_path(&output, &dist_dir, JS_ASSET)?;
            let html = strip_and_inject(&html, &css_href, &js_src);
            pages.push(StaticPage { output, html });
        }
    }

    for page in pages {
        if let Some(parent) = page.output.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create '{}': {e}", parent.display()))?;
        }
        std::fs::write(&page.output, &page.html)
            .map_err(|e| format!("cannot write '{}': {e}", page.output.display()))?;
        out::step(
            "html",
            page.output
                .strip_prefix(project_dir)
                .unwrap_or(&page.output)
                .display(),
        );
    }

    out::done("ready", format!("{name} static site in dist/"));
    Ok(())
}

/// Strips the inline `<style>` and `<script>` blocks that `ui.window` embeds
/// and injects external `<link>` and `<script src>` references in their place.
fn strip_and_inject(html: &str, css_href: &str, js_src: &str) -> String {
    let style_block = format!("<style>\n{}</style>\n", flint::UI_CSS);
    let html = html.replace(&style_block, "");
    let html = html.replace(flint::UI_JS, "");

    let assets = format!(
        "<link rel=\"stylesheet\" href=\"{css_href}\">\n\
         <script src=\"{js_src}\" defer></script>\n"
    );

    if let Some(head_end) = html.find("</head>") {
        let mut out = String::with_capacity(html.len() + assets.len());
        out.push_str(&html[..head_end]);
        out.push_str(&assets);
        out.push_str(&html[head_end..]);
        out
    } else {
        assets + &html
    }
}

/// Extracts raw JS content from the `<script>…</script>` constant.
fn js_content_from_constant() -> &'static str {
    flint::UI_JS
        .strip_prefix("<script>\n")
        .and_then(|s| s.strip_suffix("</script>\n"))
        .unwrap_or(flint::UI_JS)
}

fn reject_request_natives(module: &flint::lang::AppModule) -> Result<(), String> {
    for instr in &module.program.instructions {
        if let flint::vm::Instr::NCall { name_idx, .. } = instr {
            let name = module.program.strings[*name_idx as usize].as_ref();
            if name.starts_with("http.") && name != "http.html" {
                return Err(format!(
                    "cannot export '{}' statically: page uses request/response native '{name}'",
                    module.source_path.display()
                ));
            }
        }
    }
    Ok(())
}

fn static_output_path(dist_dir: &Path, route_path: &str) -> Result<PathBuf, String> {
    if !route_path.starts_with('/') {
        return Err(format!(
            "static route path '{route_path}' must start with '/'"
        ));
    }
    if route_path == "/" {
        return Ok(dist_dir.join("index.html"));
    }

    let mut output = dist_dir.to_path_buf();
    for segment in route_path.trim_start_matches('/').split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(format!(
                "static route path '{route_path}' is not exportable"
            ));
        }
        output.push(segment);
    }
    output.push("index.html");
    Ok(output)
}

fn relative_asset_path(page_path: &Path, dist_dir: &Path, asset: &str) -> Result<String, String> {
    let parent = page_path.parent().unwrap_or(dist_dir);
    let relative_parent = parent.strip_prefix(dist_dir).unwrap_or(parent);
    let depth = relative_parent.components().count();

    if depth == 0 {
        return Ok(asset.to_string());
    }

    let mut path = String::new();
    for _ in 0..depth {
        path.push_str("../");
    }
    path.push_str(asset);
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn maps_routes_to_directory_index_files() {
        assert_eq!(
            static_output_path(Path::new("dist"), "/").unwrap(),
            Path::new("dist/index.html")
        );
        assert_eq!(
            static_output_path(Path::new("dist"), "/about").unwrap(),
            Path::new("dist/about/index.html")
        );
        assert_eq!(
            static_output_path(Path::new("dist"), "/docs/start").unwrap(),
            Path::new("dist/docs/start/index.html")
        );
    }

    #[test]
    fn strips_inline_style_and_script_and_injects_external_refs() {
        let html = format!(
            "<head><style>\n{}</style>\n{}</head><body></body>",
            flint::UI_CSS,
            flint::UI_JS
        );
        let result = strip_and_inject(&html, "flint.css", "flint.js");
        assert!(result.contains("<link rel=\"stylesheet\" href=\"flint.css\">"));
        assert!(result.contains("<script src=\"flint.js\" defer></script>"));
        assert!(!result.contains("<style>"));
        assert!(!result.contains("<script>"));
    }

    #[test]
    fn js_content_strips_script_tags() {
        let content = js_content_from_constant();
        assert!(!content.starts_with("<script>"));
        assert!(!content.ends_with("</script>\n"));
        assert!(content.contains("flintDialog"));
    }

    #[test]
    fn injects_relative_assets_before_head_close() {
        let html = "<head><title>x</title></head>";
        let result = strip_and_inject(html, "../flint.css", "../flint.js");
        assert!(result.contains("<link rel=\"stylesheet\" href=\"../flint.css\">"));
        assert!(result.contains("<script src=\"../flint.js\" defer></script>"));
        assert!(result.find("<link").unwrap() < result.find("</head>").unwrap());
    }

    #[test]
    fn computes_relative_asset_paths() {
        assert_eq!(
            relative_asset_path(Path::new("dist/index.html"), Path::new("dist"), "flint.css")
                .unwrap(),
            "flint.css"
        );
        assert_eq!(
            relative_asset_path(
                Path::new("dist/about/index.html"),
                Path::new("dist"),
                "flint.css"
            )
            .unwrap(),
            "../flint.css"
        );
        assert_eq!(
            relative_asset_path(
                Path::new("dist/docs/start/index.html"),
                Path::new("dist"),
                "flint.js"
            )
            .unwrap(),
            "../../flint.js"
        );
    }
}

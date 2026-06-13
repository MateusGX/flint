use std::path::Path;

use super::shared::{
    append_code, append_expr, append_handler_end, append_handler_start, append_html, append_uses,
    handler_name, infer_route_path, line_number, parse_directives,
};
use super::{CompiledPageSource, PageCompileError};

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Html(String),
    Code(String),
    Expr(String),
}

pub(super) fn compile(
    source: &str,
    page_path: &Path,
    pages_dir: impl AsRef<Path>,
) -> Result<CompiledPageSource, PageCompileError> {
    let pages_dir = pages_dir.as_ref();
    let (directives, body) = parse_directives(source)?;
    let route_path = directives
        .path
        .unwrap_or_else(|| infer_route_path(page_path, pages_dir));
    let method = directives.method.unwrap_or_else(|| "GET".to_string());
    let handler = handler_name(page_path, pages_dir);
    let segments = parse_segments(&body)?;

    let mut out = String::new();
    append_uses(&mut out, directives.uses);
    append_handler_start(&mut out, &handler);

    for segment in segments {
        match segment {
            Segment::Html(html) => append_html(&mut out, &html),
            Segment::Code(code) => append_code(&mut out, &code),
            Segment::Expr(expr) => append_expr(&mut out, &expr),
        }
    }

    append_handler_end(&mut out, &method, &route_path, &handler);

    Ok(CompiledPageSource {
        route_path,
        method,
        source: out,
    })
}

fn parse_segments(source: &str) -> Result<Vec<Segment>, PageCompileError> {
    let mut segments = Vec::new();
    let mut rest = source;

    loop {
        let Some(start) = rest.find("<%") else {
            if !rest.is_empty() {
                segments.push(Segment::Html(rest.to_string()));
            }
            break;
        };
        if start > 0 {
            segments.push(Segment::Html(rest[..start].to_string()));
        }

        let tag_start_line = line_number(source, source.len() - rest.len() + start);
        let after_open = &rest[start + 2..];
        let Some(end) = after_open.find("%>") else {
            return Err(PageCompileError {
                line: tag_start_line,
                message: "unterminated '<%' block".to_string(),
            });
        };

        let inner = &after_open[..end];
        if let Some(expr) = inner.strip_prefix('=') {
            let expr = expr.trim();
            if expr.is_empty() {
                return Err(PageCompileError {
                    line: tag_start_line,
                    message: "expected an expression after '<%='".to_string(),
                });
            }
            segments.push(Segment::Expr(expr.to_string()));
        } else {
            segments.push(Segment::Code(inner.to_string()));
        }

        rest = &after_open[end + 2..];
    }

    Ok(segments)
}

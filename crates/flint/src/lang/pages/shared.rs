use std::path::{Path, PathBuf};

use super::PageCompileError;

pub(super) const HTML_PAGE_SUFFIX: &str = ".flint.html";
pub(super) const UI_PAGE_SUFFIX: &str = ".flint.ui";
pub(super) const HTML_REG: &str = "r14";
pub(super) const SCRATCH_REG: &str = "r15";

#[derive(Debug, Default)]
pub(super) struct PageDirectives {
    pub(super) method: Option<String>,
    pub(super) path: Option<String>,
    pub(super) uses: Vec<String>,
}

pub(super) fn parse_directives(source: &str) -> Result<(PageDirectives, String), PageCompileError> {
    let mut directives = PageDirectives::default();
    let mut body = String::new();
    let mut in_preamble = true;

    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_preamble {
            if is_directive(trimmed, "@page") {
                parse_page_directive(trimmed, line_no, &mut directives)?;
                continue;
            }
            if is_directive(trimmed, "@route") {
                parse_route_directive(trimmed, line_no, &mut directives)?;
                continue;
            }
            if is_directive(trimmed, "@use") {
                parse_use_directive(trimmed, line_no, &mut directives)?;
                continue;
            }
            if !trimmed.is_empty() {
                in_preamble = false;
            }
        }

        body.push_str(line);
        body.push('\n');
    }

    Ok((directives, body))
}

fn is_directive(line: &str, name: &str) -> bool {
    line == name
        || line
            .strip_prefix(name)
            .is_some_and(|rest| rest.starts_with(char::is_whitespace))
}

fn parse_page_directive(
    line: &str,
    line_no: usize,
    directives: &mut PageDirectives,
) -> Result<(), PageCompileError> {
    let rest = line.strip_prefix("@page").unwrap().trim();
    if rest.is_empty() {
        directives.method.get_or_insert_with(|| "GET".to_string());
        return Ok(());
    }
    if rest.starts_with('"') {
        directives.method = Some("GET".to_string());
        directives.path = Some(parse_quoted(rest, line_no, "@page")?);
        return Ok(());
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let method = parts.next().unwrap();
    let path = parts.next().unwrap_or("").trim();
    directives.method = Some(method.to_uppercase());
    directives.path = Some(parse_quoted(path, line_no, "@page")?);
    Ok(())
}

fn parse_route_directive(
    line: &str,
    line_no: usize,
    directives: &mut PageDirectives,
) -> Result<(), PageCompileError> {
    let rest = line.strip_prefix("@route").unwrap().trim();
    let mut parts = rest.splitn(2, char::is_whitespace);
    let method = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PageCompileError {
            line: line_no,
            message: "expected '@route METHOD \"/path\"'".to_string(),
        })?;
    let path = parts.next().unwrap_or("").trim();
    directives.method = Some(method.to_uppercase());
    directives.path = Some(parse_quoted(path, line_no, "@route")?);
    Ok(())
}

fn parse_use_directive(
    line: &str,
    line_no: usize,
    directives: &mut PageDirectives,
) -> Result<(), PageCompileError> {
    let rest = line.strip_prefix("@use").unwrap().trim();
    directives.uses.push(parse_quoted(rest, line_no, "@use")?);
    Ok(())
}

fn parse_quoted(text: &str, line: usize, directive: &str) -> Result<String, PageCompileError> {
    let Some(rest) = text.strip_prefix('"') else {
        return Err(PageCompileError {
            line,
            message: format!("expected {directive} to contain a quoted string"),
        });
    };

    let mut value = String::new();
    let mut chars = rest.char_indices();
    while let Some((idx, ch)) = chars.next() {
        match ch {
            '"' => {
                if !rest[idx + 1..].trim().is_empty() {
                    return Err(PageCompileError {
                        line,
                        message: format!("unexpected text after quoted string in {directive}"),
                    });
                }
                return Ok(value);
            }
            '\\' => {
                let Some((_, escaped)) = chars.next() else {
                    return Err(PageCompileError {
                        line,
                        message: format!("unterminated escape sequence in {directive}"),
                    });
                };
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    other => {
                        return Err(PageCompileError {
                            line,
                            message: format!("unknown escape sequence '\\{other}' in {directive}"),
                        })
                    }
                }
            }
            other => value.push(other),
        }
    }

    Err(PageCompileError {
        line,
        message: format!("unterminated quoted string in {directive}"),
    })
}

pub(super) fn append_uses(out: &mut String, uses: Vec<String>) {
    for use_path in uses {
        out.push_str("use \"");
        out.push_str(&flint_string_literal(&use_path));
        out.push_str("\"\n");
    }
    if !out.is_empty() {
        out.push('\n');
    }
}

pub(super) fn append_handler_start(out: &mut String, handler: &str) {
    out.push_str(handler);
    out.push_str(":\n");
    out.push_str("    mov ");
    out.push_str(HTML_REG);
    out.push_str(", \"\"\n");
}

pub(super) fn append_handler_end(out: &mut String, method: &str, route_path: &str, handler: &str) {
    out.push_str("    ncall http.html, ");
    out.push_str(HTML_REG);
    out.push_str("\n    ret\n\n");
    out.push_str("route ");
    out.push_str(method);
    out.push_str(" \"");
    out.push_str(&flint_string_literal(route_path));
    out.push_str("\" -> ");
    out.push_str(handler);
    out.push('\n');
}

pub(super) fn append_html(out: &mut String, html: &str) {
    if html.is_empty() {
        return;
    }
    out.push_str("    mov ");
    out.push_str(SCRATCH_REG);
    out.push_str(", \"");
    out.push_str(&flint_string_literal(html));
    out.push_str("\"\n    ncallr ");
    out.push_str(HTML_REG);
    out.push_str(", string.concat, ");
    out.push_str(HTML_REG);
    out.push_str(", ");
    out.push_str(SCRATCH_REG);
    out.push('\n');
}

pub(super) fn append_code(out: &mut String, code: &str) {
    for line in code.lines() {
        if line.trim().is_empty() {
            continue;
        }
        out.push_str("    ");
        out.push_str(line.trim());
        out.push('\n');
    }
}

pub(super) fn append_expr(out: &mut String, expr: &str) {
    out.push_str("    mov ");
    out.push_str(SCRATCH_REG);
    out.push_str(", ");
    out.push_str(expr);
    out.push_str("\n    ncallr ");
    out.push_str(SCRATCH_REG);
    out.push_str(", string.from, ");
    out.push_str(SCRATCH_REG);
    out.push_str("\n    ncallr ");
    out.push_str(SCRATCH_REG);
    out.push_str(", string.escape_html, ");
    out.push_str(SCRATCH_REG);
    out.push_str("\n    ncallr ");
    out.push_str(HTML_REG);
    out.push_str(", string.concat, ");
    out.push_str(HTML_REG);
    out.push_str(", ");
    out.push_str(SCRATCH_REG);
    out.push('\n');
}

fn flint_string_literal(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => {}
            other => escaped.push(other),
        }
    }
    escaped
}

pub(super) fn collect_page_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_page_paths(&path, paths)?;
        } else if is_page_path(&path) {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_page_path(path: &Path) -> bool {
    is_html_page_path(path) || is_ui_page_path(path)
}

fn is_html_page_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(HTML_PAGE_SUFFIX))
}

pub(super) fn is_ui_page_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(UI_PAGE_SUFFIX))
}

pub(super) fn infer_route_path(page_path: &Path, pages_dir: &Path) -> String {
    let relative = page_path.strip_prefix(pages_dir).unwrap_or(page_path);
    let mut segments = page_segments(relative);
    if segments.last().is_some_and(|segment| segment == "index") {
        segments.pop();
    }
    if segments.is_empty() {
        return "/".to_string();
    }
    format!("/{}", segments.join("/"))
}

pub(super) fn handler_name(page_path: &Path, pages_dir: &Path) -> String {
    let relative = page_path.strip_prefix(pages_dir).unwrap_or(page_path);
    let mut name = String::from("__page");
    for segment in page_segments(relative) {
        name.push('_');
        for ch in segment.chars() {
            if ch.is_ascii_alphanumeric() {
                name.push(ch.to_ascii_lowercase());
            } else {
                name.push('_');
            }
        }
    }
    name
}

fn page_segments(relative: &Path) -> Vec<String> {
    relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(|segment| {
            segment
                .strip_suffix(HTML_PAGE_SUFFIX)
                .or_else(|| segment.strip_suffix(UI_PAGE_SUFFIX))
                .unwrap_or(segment)
        })
        .map(|segment| {
            if segment.starts_with('[') && segment.ends_with(']') && segment.len() > 2 {
                format!(":{}", &segment[1..segment.len() - 1])
            } else {
                segment.to_string()
            }
        })
        .collect()
}

pub(super) fn line_number(source: &str, byte_idx: usize) -> usize {
    source[..byte_idx].bytes().filter(|b| *b == b'\n').count() + 1
}

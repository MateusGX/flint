use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::sections;

/// Validates that a `.fl` source file uses section-based syntax.
///
/// `use` directives, blank lines and comments may appear outside sections, but
/// all labels, instructions, data declarations and route declarations must be
/// inside a `section ...` block.
///
/// Returns an error message if any such line is found.
pub fn validate_sections(source: &str) -> Result<(), String> {
    let mut current_section: Option<&str> = None;
    let mut saw_section = false;

    for line in source.lines() {
        let t = line.trim();
        if let Some(name) = sections::section_name(t) {
            current_section = Some(name);
            saw_section = true;
            continue;
        }

        if t.is_empty() || t.starts_with(';') || parse_use(line).is_some() {
            continue;
        }

        if !saw_section {
            return Err(
                ".fl files must use section blocks; add 'section .text', 'section .route', 'section .data' or 'section .bss'"
                    .to_string(),
            );
        }

        if current_section != Some(sections::ROUTE) && (t.starts_with("route ") || t == "route") {
            return Err("route declarations must be inside 'section .route'".to_string());
        }
    }

    if !saw_section {
        return Err(
            ".fl files must contain at least one section block: 'section .text', 'section .route', 'section .data' or 'section .bss'"
                .to_string(),
        );
    }

    Ok(())
}

/// Lowers `section .route` declarations into the compiler's internal `route`
/// directive form. This is a pre-processing step only; `.fl` files must use
/// `section .route` — bare `route` directives are rejected by
/// [`validate_sections`] before this function is called.
///
/// Other sections (`.data`, `.bss`, `.text`) are passed through unchanged.
pub fn normalize_sections(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut in_route = false;

    for line in source.lines() {
        let t = line.trim();

        if let Some(name) = sections::section_name(t) {
            in_route = name == sections::ROUTE;
            if !in_route {
                out.push_str(line);
                out.push('\n');
            }
            continue;
        }

        if in_route {
            if parse_use(line).is_some() {
                // use directives pass through unchanged; expand() handles them.
                out.push_str(line);
                out.push('\n');
            } else if !t.is_empty() && !t.starts_with(';') {
                out.push_str("route ");
                out.push_str(t);
                out.push('\n');
            }
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }

    out
}

/// Expands every `use "path"` line in `source`, replacing it with the
/// (recursively expanded) content of that file. Paths are resolved relative
/// to `project_root`.
///
/// A file included more than once is inlined only on its first appearance —
/// identical to how `%include` guards work in NASM.
pub fn expand(source: &str, project_root: &Path) -> Result<String, ExpandError> {
    let root = project_root.canonicalize().map_err(|e| ExpandError {
        path: project_root.to_path_buf(),
        reason: e.to_string(),
    })?;
    expand_rec(source, &root, &mut HashSet::new())
}

fn expand_rec(
    source: &str,
    root: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<String, ExpandError> {
    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        match parse_use(line) {
            Some(rel) => {
                let path = root.join(rel);
                let canonical = path.canonicalize().map_err(|e| ExpandError {
                    path: path.clone(),
                    reason: e.to_string(),
                })?;
                if !canonical.starts_with(root) {
                    return Err(ExpandError {
                        path,
                        reason: "include path escapes the project root".to_string(),
                    });
                }
                if !visited.insert(canonical.clone()) {
                    continue;
                }
                let content = std::fs::read_to_string(&canonical).map_err(|e| ExpandError {
                    path: canonical.clone(),
                    reason: e.to_string(),
                })?;
                validate_sections(&content).map_err(|reason| ExpandError {
                    path: canonical.clone(),
                    reason,
                })?;
                let normalized = normalize_sections(&content);
                let expanded = expand_rec(&normalized, root, visited)?;
                out.push_str(&expanded);
                if !out.ends_with('\n') {
                    out.push('\n');
                }
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    Ok(out)
}

fn parse_use(line: &str) -> Option<String> {
    let rest = line.trim_start().strip_prefix("use")?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }

    let rest = rest.trim_start().strip_prefix('"')?;
    parse_quoted_include(rest)
}

fn parse_quoted_include(rest: &str) -> Option<String> {
    let mut value = String::new();
    let mut chars = rest.char_indices();
    while let Some((idx, ch)) = chars.next() {
        match ch {
            '"' => {
                let tail = rest[idx + 1..].trim_start();
                if tail.is_empty() || tail.starts_with(';') {
                    return Some(value);
                }
                return None;
            }
            '\\' => {
                let (_, escaped) = chars.next()?;
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    _ => return None,
                }
            }
            other => value.push(other),
        }
    }
    None
}

#[derive(Debug)]
pub struct ExpandError {
    pub path: PathBuf,
    pub reason: String,
}

impl std::fmt::Display for ExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.reason)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{expand, parse_use, validate_sections};

    fn temp_project() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("flint-preprocessor-test-{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn rejects_includes_that_escape_project_root() {
        let root = temp_project();
        let outside = root.parent().unwrap().join("flint-outside-include.fl");
        fs::write(&outside, "mov r0, 1\n").unwrap();

        let source = format!(
            "use \"../{}\"\n",
            outside.file_name().unwrap().to_string_lossy()
        );
        let err = expand(&source, &root).unwrap_err();
        assert!(
            err.to_string().contains("escapes the project root"),
            "{err}"
        );

        let _ = fs::remove_file(outside);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn expands_use_lines_with_trailing_comments() {
        let root = temp_project();
        fs::write(root.join("lib.fl"), "section .text\nmov r0, 7\n").unwrap();

        let expanded = expand("use \"lib.fl\" ; shared helpers\nhlt\n", &root).unwrap();

        assert!(expanded.contains("mov r0, 7"), "{expanded}");
        assert!(!expanded.contains("use \"lib.fl\""), "{expanded}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_source_without_sections() {
        let err = validate_sections("helper:\n  ret\n").unwrap_err();
        assert!(err.contains("must use section blocks"), "{err}");
    }

    #[test]
    fn rejects_included_files_without_sections() {
        let root = temp_project();
        fs::write(root.join("lib.fl"), "mov r0, 7\n").unwrap();

        let err = expand("use \"lib.fl\"\nsection .text\nhlt\n", &root).unwrap_err();
        assert!(err.to_string().contains("must use section blocks"), "{err}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn parses_escaped_include_paths() {
        assert_eq!(
            parse_use(r#"use "lib\"quoted.fl" ; comment"#).as_deref(),
            Some("lib\"quoted.fl")
        );
        assert_eq!(
            parse_use(r#"use "lib\\path.fl""#).as_deref(),
            Some("lib\\path.fl")
        );
    }
}

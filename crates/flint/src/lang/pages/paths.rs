use std::path::{Path, PathBuf};

pub(super) const UI_PAGE_SUFFIX: &str = ".flint.ui";

pub(super) fn collect_page_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_page_paths(&path, paths)?;
        } else if is_ui_page_path(&path) {
            paths.push(path);
        }
    }
    Ok(())
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
        .map(|segment| segment.strip_suffix(UI_PAGE_SUFFIX).unwrap_or(segment))
        .map(|segment| {
            if segment.starts_with('[') && segment.ends_with(']') && segment.len() > 2 {
                format!(":{}", &segment[1..segment.len() - 1])
            } else {
                segment.to_string()
            }
        })
        .collect()
}

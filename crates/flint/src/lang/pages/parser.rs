use crate::lang::sections::{self, HTTP_METHODS};

use super::compiler::Parsed;
use super::PageCompileError;

#[derive(PartialEq)]
pub(super) enum PageSection {
    None,
    Route,
    Data,
    Bss,
    Text,
    Render,
}

pub(super) fn parse_sections(source: &str) -> Result<Parsed, PageCompileError> {
    let first_line = source
        .lines()
        .map(|l| l.trim())
        .find(|t| !t.is_empty() && !t.starts_with(';') && !t.starts_with("@use "));

    if !first_line.is_some_and(|t| t.starts_with("section ")) {
        return Err(PageCompileError {
            line: 1,
            message: ".flint.ui files must use the section format — start with 'section .route'"
                .to_string(),
        });
    }

    let mut p = Parsed::default();
    let mut cur = PageSection::None;

    for (idx, raw) in source.lines().enumerate() {
        let line_no = idx + 1;
        let t = raw.trim();

        if t.starts_with(';') || t.is_empty() {
            match cur {
                PageSection::Text => {
                    p.text.push_str(raw);
                    p.text.push('\n');
                }
                PageSection::Render => {
                    p.render.push_str(raw);
                    p.render.push('\n');
                }
                _ => {}
            }
            continue;
        }

        if t.starts_with("@use ") && cur == PageSection::None {
            let rest = t.strip_prefix("@use ").unwrap().trim();
            p.uses.push(parse_quoted_str(rest, line_no, "@use")?);
            continue;
        }

        if let Some(name) = sections::section_name(t) {
            cur = page_section(name, line_no)?;
            continue;
        }

        match cur {
            PageSection::None => {}
            PageSection::Route => parse_route_line(t, line_no, &mut p)?,
            PageSection::Data => parse_data_line(t, line_no, &mut p)?,
            PageSection::Bss => parse_bss_line(t, line_no, &mut p)?,
            PageSection::Text => {
                p.text.push_str(raw);
                p.text.push('\n');
            }
            PageSection::Render => {
                p.render.push_str(raw);
                p.render.push('\n');
            }
        }
    }

    Ok(p)
}

fn page_section(name: &str, line_no: usize) -> Result<PageSection, PageCompileError> {
    if !sections::PAGE_SECTIONS.contains(&name) {
        return Err(PageCompileError {
            line: line_no,
            message: format!(
                "unknown section '{name}' — expected {}",
                sections::PAGE_SECTIONS.join(", ")
            ),
        });
    }

    match name {
        s if s == sections::ROUTE => Ok(PageSection::Route),
        s if s == sections::DATA => Ok(PageSection::Data),
        s if s == sections::BSS => Ok(PageSection::Bss),
        s if s == sections::TEXT => Ok(PageSection::Text),
        s if s == sections::RENDER => Ok(PageSection::Render),
        _ => unreachable!("PAGE_SECTIONS and PageSection mapping are out of sync"),
    }
}

fn parse_route_line(line: &str, line_no: usize, p: &mut Parsed) -> Result<(), PageCompileError> {
    let (method_raw, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let method = method_raw.trim().to_uppercase();

    if !HTTP_METHODS.contains(&method.as_str()) {
        return Err(PageCompileError {
            line: line_no,
            message: format!("unknown HTTP method '{method}' in section .route"),
        });
    }

    let rest = rest.trim();
    if rest.is_empty() {
        return Err(PageCompileError {
            line: line_no,
            message: format!("expected path after '{method}' in section .route"),
        });
    }

    p.method = Some(method);
    p.path = Some(parse_quoted_str(rest, line_no, "section .route")?);
    Ok(())
}

fn parse_data_line(line: &str, line_no: usize, p: &mut Parsed) -> Result<(), PageCompileError> {
    let mut words = line.split_whitespace();
    let label = words.next().unwrap_or("").to_string();
    let kw = words.next().unwrap_or("");

    if label.is_empty() {
        return Ok(());
    }
    if kw != "db" {
        return Err(PageCompileError {
            line: line_no,
            message: format!("expected 'db' after label '{label}' in section .data, got '{kw}'"),
        });
    }

    let quote_pos = line.find('"').ok_or_else(|| PageCompileError {
        line: line_no,
        message: format!("expected quoted string after 'db' for label '{label}'"),
    })?;
    let value = parse_quoted_str(&line[quote_pos..], line_no, "section .data")?;
    p.data.insert(label, value);
    Ok(())
}

fn parse_bss_line(line: &str, line_no: usize, p: &mut Parsed) -> Result<(), PageCompileError> {
    let mut words = line.split_whitespace();
    let label = words.next().unwrap_or("").to_string();
    let kw = words.next().unwrap_or("");
    let count_str = words.next().unwrap_or("");

    if label.is_empty() {
        return Ok(());
    }
    if kw != "res" {
        return Err(PageCompileError {
            line: line_no,
            message: format!("expected 'res' after label '{label}' in section .bss, got '{kw}'"),
        });
    }

    let count: usize = count_str.parse().map_err(|_| PageCompileError {
        line: line_no,
        message: format!("expected integer count after 'res', got '{count_str}'"),
    })?;

    p.bss.push((label, count));
    Ok(())
}

pub(super) fn parse_quoted_str(
    s: &str,
    line_no: usize,
    ctx: &str,
) -> Result<String, PageCompileError> {
    let inner = s.trim().strip_prefix('"').ok_or_else(|| PageCompileError {
        line: line_no,
        message: format!("expected a quoted string in {ctx}, got: {s}"),
    })?;

    let mut value = String::new();
    let mut chars = inner.chars();

    loop {
        match chars.next() {
            None => {
                return Err(PageCompileError {
                    line: line_no,
                    message: format!("unterminated string in {ctx}"),
                })
            }
            Some('"') => {
                let trailing = chars.as_str();
                if !trailing.trim().is_empty() {
                    return Err(PageCompileError {
                        line: line_no,
                        message: format!(
                            "unexpected text after closing quote in {ctx}: {trailing}"
                        ),
                    });
                }
                return Ok(value);
            }
            Some('\\') => match chars.next() {
                Some('n') => value.push('\n'),
                Some('t') => value.push('\t'),
                Some('"') => value.push('"'),
                Some('\\') => value.push('\\'),
                Some(other) => {
                    return Err(PageCompileError {
                        line: line_no,
                        message: format!("unknown escape '\\{other}' in {ctx}"),
                    })
                }
                None => {
                    return Err(PageCompileError {
                        line: line_no,
                        message: format!("unterminated escape in {ctx}"),
                    })
                }
            },
            Some(c) => value.push(c),
        }
    }
}

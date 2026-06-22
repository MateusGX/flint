use std::collections::HashMap;

use super::blocks::{block_close_native, Block};
use super::compiler::FLINT_MNEMONICS;
use super::emit::{emit0, emit1, emit2, emit3, emit4, emit5, err_missing_arg};
use super::PageCompileError;

pub(super) fn compile_render(
    render: &str,
    data: &HashMap<String, String>,
    out: &mut String,
) -> Result<(), PageCompileError> {
    let mut stack: Vec<Block> = Vec::new();

    for (idx, raw) in render.lines().enumerate() {
        let line_no = idx + 1;
        let t = raw.trim();
        if t.is_empty() || t.starts_with(';') {
            continue;
        }
        compile_render_line(t, line_no, &mut stack, data, out)?;
    }

    Ok(())
}

fn compile_render_line(
    line: &str,
    line_no: usize,
    stack: &mut Vec<Block>,
    data: &HashMap<String, String>,
    out: &mut String,
) -> Result<(), PageCompileError> {
    if is_label_def(line) {
        out.push_str(line);
        out.push('\n');
        return Ok(());
    }

    let (kw, args) = tokenize_render_line(line);
    let ctx = stack.last().cloned();

    if FLINT_MNEMONICS.contains(&kw.as_str()) {
        out.push_str("    ");
        out.push_str(line);
        out.push('\n');
        return Ok(());
    }

    macro_rules! arg {
        ($idx:expr) => {
            args.get($idx)
                .map(|s| s.as_str())
                .ok_or_else(|| err_missing_arg(&kw, $idx, line_no))?
        };
    }

    match kw.as_str() {
        // ── Block closer ──────────────────────────────────────────────────────
        "end" => {
            let Some(block) = stack.pop() else {
                return Err(PageCompileError {
                    line: line_no,
                    message: "unexpected 'end' — no open block to close".to_string(),
                });
            };
            emit0(out, block_close_native(&block));
        }

        // ── Block openers ─────────────────────────────────────────────────────
        "window" => {
            emit1(out, "ui.window", arg!(0), data);
            stack.push(Block::Window);
        }
        "navbar" => {
            emit0(out, "ui.navbar");
            stack.push(Block::Navbar);
        }
        "layout" => {
            emit0(out, "ui.layout");
            stack.push(Block::Layout);
        }
        "sidebar" => {
            emit0(out, "ui.sidebar");
            stack.push(Block::Sidebar);
        }
        "main" => {
            emit0(out, "ui.main");
            stack.push(Block::Main);
        }
        "card" => {
            emit1(out, "ui.card", arg!(0), data);
            stack.push(Block::Card);
        }
        "block" => {
            if args.len() >= 2 {
                emit2(out, "ui.section", arg!(0), arg!(1), data);
            } else {
                emit1(out, "ui.section", arg!(0), data);
            }
            stack.push(Block::UiSection);
        }
        "table" => {
            emit0(out, "ui.table");
            stack.push(Block::Table);
        }
        "tfoot" => {
            emit0(out, "ui.tfoot");
            stack.push(Block::Tfoot);
        }
        "menu" => {
            emit1(out, "ui.menu", arg!(0), data);
            stack.push(Block::Menu);
        }
        "list" => {
            emit0(out, "ui.list");
            stack.push(Block::List);
        }
        "ol" => {
            emit0(out, "ui.ol");
            stack.push(Block::Ol);
        }
        "row" if !matches!(ctx, Some(Block::Table) | Some(Block::Tfoot)) => {
            emit0(out, "ui.row");
            stack.push(Block::LayoutRow);
        }
        "col" => {
            emit0(out, "ui.column");
            stack.push(Block::Column);
        }
        "toolbar" => {
            emit0(out, "ui.toolbar");
            stack.push(Block::Toolbar);
        }
        "action_bar" => {
            emit0(out, "ui.action_bar");
            stack.push(Block::ActionBar);
        }
        "accordion" => {
            emit0(out, "ui.accordion");
            stack.push(Block::Accordion);
        }
        "item" if matches!(ctx, Some(Block::Accordion)) => {
            emit1(out, "ui.accordion_item", arg!(0), data);
            stack.push(Block::AccordionItem);
        }
        "breadcrumb" => {
            emit0(out, "ui.breadcrumb");
            stack.push(Block::Breadcrumb);
        }
        "pagination" => {
            emit0(out, "ui.pagination");
            stack.push(Block::Pagination);
        }
        "steps" => {
            emit0(out, "ui.steps");
            stack.push(Block::Steps);
        }
        "tree" => {
            emit0(out, "ui.tree");
            stack.push(Block::Tree);
        }
        "group" if matches!(ctx, Some(Block::Tree) | Some(Block::TreeGroup)) => {
            emit1(out, "ui.tree_group", arg!(0), data);
            stack.push(Block::TreeGroup);
        }
        "form" => {
            emit2(out, "ui.form", arg!(0), arg!(1), data);
            stack.push(Block::Form);
        }
        "fieldset" => {
            emit1(out, "ui.fieldset", arg!(0), data);
            stack.push(Block::Fieldset);
        }
        "footer" if args.is_empty() => {
            emit0(out, "ui.footer");
            stack.push(Block::Footer);
        }
        "footer" => {
            emit1(out, "ui.footer", arg!(0), data);
            stack.push(Block::Footer);
        }
        "tabs" => {
            emit0(out, "ui.tabs");
            stack.push(Block::Tabs);
        }
        "panel" if matches!(ctx, Some(Block::Tabs)) => {
            emit1(out, "ui.tab_panel", arg!(0), data);
            stack.push(Block::TabPanel);
        }
        "select" => {
            emit2(out, "ui.select", arg!(0), arg!(1), data);
            stack.push(Block::Select);
        }
        "dialog" => {
            emit2(out, "ui.dialog", arg!(0), arg!(1), data);
            stack.push(Block::Dialog);
        }

        // ── Context-sensitive leaves ───────────────────────────────────────────
        "nav" | "nav*" => {
            let label = arg!(0);
            let href = arg!(1);
            match ctx {
                Some(Block::Menu) => {
                    if kw == "nav*" {
                        emit2(out, "ui.menu_active", label, href, data);
                    } else {
                        emit2(out, "ui.menu_item", label, href, data);
                    }
                }
                _ => emit2(out, "ui.nav_item", label, href, data),
            }
        }
        "active" => emit2(out, "ui.menu_active", arg!(0), arg!(1), data),
        "item" if matches!(ctx, Some(Block::Menu)) => {
            emit2(out, "ui.menu_item", arg!(0), arg!(1), data);
        }
        "item" if matches!(ctx, Some(Block::Tree) | Some(Block::TreeGroup)) => {
            emit2(out, "ui.tree_item", arg!(0), arg!(1), data);
        }
        "li" => match ctx {
            Some(Block::Ol) => emit1(out, "ui.ol_item", arg!(0), data),
            _ => emit1(out, "ui.list_item", arg!(0), data),
        },
        "head" if matches!(ctx, Some(Block::Table) | Some(Block::Tfoot)) => {
            emit0(out, "ui.tr");
            for a in &args {
                emit1(out, "ui.th", a, data);
            }
            emit0(out, "ui.tr_end");
        }
        "row" if matches!(ctx, Some(Block::Table) | Some(Block::Tfoot)) => {
            emit0(out, "ui.tr");
            for a in &args {
                emit1(out, "ui.td", a, data);
            }
            emit0(out, "ui.tr_end");
        }
        "step" if matches!(ctx, Some(Block::Steps)) => {
            emit2(out, "ui.step", arg!(0), arg!(1), data);
        }
        "crumb" if matches!(ctx, Some(Block::Breadcrumb)) => {
            emit2(out, "ui.breadcrumb_item", arg!(0), arg!(1), data);
        }
        "page" if matches!(ctx, Some(Block::Pagination)) => {
            emit2(out, "ui.page_item", arg!(0), arg!(1), data);
        }
        "current" if matches!(ctx, Some(Block::Pagination)) => {
            emit1(out, "ui.page_current", arg!(0), data);
        }
        "tab" if matches!(ctx, Some(Block::Tabs)) => {
            emit2(out, "ui.tab", arg!(0), arg!(1), data);
        }
        "tabs_body" if matches!(ctx, Some(Block::Tabs)) => {
            emit0(out, "ui.tabs_body");
        }
        "option" if matches!(ctx, Some(Block::Select)) => {
            emit2(out, "ui.option", arg!(0), arg!(1), data);
        }

        // ── Global leaves ──────────────────────────────────────────────────────
        "field" => emit2(out, "ui.field", arg!(0), arg!(1), data),
        "btn" => emit2(out, "ui.button", arg!(0), arg!(1), data),
        "text" => emit1(out, "ui.text", arg!(0), data),
        "title" => emit1(out, "ui.title", arg!(0), data),
        "badge" => emit1(out, "ui.badge", arg!(0), data),
        "divider" => emit0(out, "ui.divider"),
        "caption" => emit1(out, "ui.caption", arg!(0), data),
        "empty" => emit1(out, "ui.empty", arg!(0), data),
        "code" => emit1(out, "ui.code", arg!(0), data),
        "kbd" => emit1(out, "ui.kbd", arg!(0), data),
        "link" => emit2(out, "ui.link", arg!(0), arg!(1), data),
        "image" => emit2(out, "ui.image", arg!(0), arg!(1), data),
        "stat" => emit2(out, "ui.stat", arg!(0), arg!(1), data),
        "alert" => emit2(out, "ui.alert", arg!(0), arg!(1), data),
        "status" => emit2(out, "ui.status", arg!(0), arg!(1), data),
        "progress" => emit2(out, "ui.progress", arg!(0), arg!(1), data),
        "meter" => emit2(out, "ui.meter", arg!(0), arg!(1), data),
        "submit" => emit1(out, "ui.submit", arg!(0), data),
        "input" => emit2(out, "ui.input", arg!(0), arg!(1), data),
        "password" => emit2(out, "ui.password", arg!(0), arg!(1), data),
        "number" => emit2(out, "ui.number", arg!(0), arg!(1), data),
        "file" => emit2(out, "ui.file", arg!(0), arg!(1), data),
        "textarea" => emit2(out, "ui.textarea", arg!(0), arg!(1), data),
        "checkbox" => emit3(out, "ui.checkbox", arg!(0), arg!(1), arg!(2), data),
        "radio" => emit3(out, "ui.radio", arg!(0), arg!(1), arg!(2), data),
        "hidden" => emit2(out, "ui.hidden", arg!(0), arg!(1), data),
        "dialog_trigger" => emit2(out, "ui.dialog_trigger", arg!(0), arg!(1), data),
        "dialog_alert" => emit3(out, "ui.dialog_alert", arg!(0), arg!(1), arg!(2), data),
        "dialog_confirm" => emit4(
            out,
            "ui.dialog_confirm",
            arg!(0),
            arg!(1),
            arg!(2),
            arg!(3),
            data,
        ),
        "dialog_prompt" => emit5(
            out,
            "ui.dialog_prompt",
            [arg!(0), arg!(1), arg!(2), arg!(3), arg!(4)],
            data,
        ),

        // Unknown — pass through as raw Flint
        _ => {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
    }

    Ok(())
}

fn is_label_def(line: &str) -> bool {
    let t = line.trim();
    let Some(name) = t.strip_suffix(':') else {
        return false;
    };
    !name.is_empty()
        && !name.contains(' ')
        && !name.contains(',')
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
}

fn tokenize_render_line(line: &str) -> (String, Vec<String>) {
    let t = line.trim();
    let (kw, rest) = t
        .split_once(char::is_whitespace)
        .map(|(k, r)| (k, r.trim()))
        .unwrap_or((t, ""));

    let args = if rest.is_empty() {
        Vec::new()
    } else {
        parse_arg_list(rest)
    };
    (kw.to_string(), args)
}

fn parse_arg_list(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut chars = s.chars().peekable();

    loop {
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }

        let arg: String = if chars.peek() == Some(&'"') {
            let mut buf = String::from('"');
            chars.next();
            loop {
                match chars.next() {
                    None => break,
                    Some('"') => {
                        buf.push('"');
                        break;
                    }
                    Some('\\') => {
                        buf.push('\\');
                        if let Some(c) = chars.next() {
                            buf.push(c);
                        }
                    }
                    Some(c) => buf.push(c),
                }
            }
            buf
        } else {
            let mut buf = String::new();
            loop {
                match chars.peek() {
                    None | Some(',') => break,
                    Some(c) if c.is_whitespace() => break,
                    _ => buf.push(chars.next().unwrap()),
                }
            }
            buf.trim().to_string()
        };

        if !arg.is_empty() {
            args.push(arg);
        }

        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        if chars.peek() == Some(&',') {
            chars.next();
        } else {
            break;
        }
    }

    args
}

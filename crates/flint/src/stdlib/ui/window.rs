//! `ui.window` — `dst = html ++ <styled page frame opener>`. Emits the
//! document shell (doctype, default stylesheet, `<title>`, header with
//! `title`) and opens the content stack. Pair with `ui.window_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_html, UI_CSS, UI_JS};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.window")?, "ui.window", 0)?;
        let title = expect_str(arg(args, 1, "ui.window")?, "ui.window", 1)?;
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{title}</title>\n<style>\n{UI_CSS}</style>\n{UI_JS}</head>\n<body>\n<main class=\"flint-window\"><div class=\"flint-surface\"><div class=\"flint-header\"><p class=\"flint-eyebrow\">Flint UI</p><h1>{title}</h1></div><div class=\"flint-stack\">\n"
        )))))
    })
}

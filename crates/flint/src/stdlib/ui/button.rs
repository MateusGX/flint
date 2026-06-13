//! `ui.button` — `dst = html ++ <link styled as a button>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.button")?, "ui.button", 0)?;
        let label = expect_str(arg(args, 1, "ui.button")?, "ui.button", 1)?;
        let href = expect_str(arg(args, 2, "ui.button")?, "ui.button", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<a class=\"flint-button\" href=\"{href}\">{label}</a>\n"
        )))))
    })
}

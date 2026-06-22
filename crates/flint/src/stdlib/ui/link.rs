//! `ui.link` — `dst = html ++ <inline hyperlink>`. Renders a plain `<a>` tag
//! styled as a classic blue hyperlink, unlike `ui.button` which is a raised block.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.link")?, "ui.link", 0)?;
        let label = expect_str(arg(args, 1, "ui.link")?, "ui.link", 1)?;
        let href = expect_str(arg(args, 2, "ui.link")?, "ui.link", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<a href=\"{href}\">{label}</a>\n"
        )))))
    })
}

//! `ui.kbd` — `dst = html ++ <keyboard shortcut>`. Renders `<kbd>` with a
//! raised button style for displaying keyboard shortcuts.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.kbd")?, "ui.kbd", 0)?;
        let text = expect_str(arg(args, 1, "ui.kbd")?, "ui.kbd", 1)?;
        let text = escape_html(text);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<kbd class=\"flint-kbd\">{text}</kbd>\n"
        )))))
    })
}

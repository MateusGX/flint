//! `ui.empty` — `dst = html ++ <empty state placeholder>`. Displays a
//! centered message when a list or table has no records.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.empty")?, "ui.empty", 0)?;
        let message = expect_str(arg(args, 1, "ui.empty")?, "ui.empty", 1)?;
        let message = escape_html(message);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-empty\">{message}</div>\n"
        )))))
    })
}

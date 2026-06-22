//! `ui.badge` — `dst = html ++ <inline status tag>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.badge")?, "ui.badge", 0)?;
        let label = expect_str(arg(args, 1, "ui.badge")?, "ui.badge", 1)?;
        let label = escape_html(label);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<span class=\"flint-badge\">{label}</span>\n"
        )))))
    })
}

//! `ui.alert` — `dst = html ++ <message box>`.
//! `kind` must be one of: "error", "warning", "info", "success".

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.alert")?, "ui.alert", 0)?;
        let kind = expect_str(arg(args, 1, "ui.alert")?, "ui.alert", 1)?;
        let msg = expect_str(arg(args, 2, "ui.alert")?, "ui.alert", 2)?;
        let kind = escape_html(kind);
        let msg = escape_html(msg);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-alert flint-alert-{kind}\">{msg}</div>\n"
        )))))
    })
}

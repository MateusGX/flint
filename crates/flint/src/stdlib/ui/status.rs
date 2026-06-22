//! `ui.status` — `dst = html ++ <colored dot + label>`.
//! `kind` must be one of: "online", "offline", "busy", "away".

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.status")?, "ui.status", 0)?;
        let label = expect_str(arg(args, 1, "ui.status")?, "ui.status", 1)?;
        let kind = expect_str(arg(args, 2, "ui.status")?, "ui.status", 2)?;
        let label = escape_html(label);
        let kind = escape_html(kind);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<span class=\"flint-status flint-status-{kind}\"><span class=\"flint-status-dot\"></span>{label}</span>\n"
        )))))
    })
}

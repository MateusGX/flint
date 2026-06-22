//! `ui.dialog_trigger` — `dst = html ++ <button that opens a dialog>`.
//! `id` must match the `id` passed to `ui.dialog`, `ui.dialog_alert`,
//! `ui.dialog_confirm`, or `ui.dialog_prompt`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.dialog_trigger")?, "ui.dialog_trigger", 0)?;
        let label = expect_str(arg(args, 1, "ui.dialog_trigger")?, "ui.dialog_trigger", 1)?;
        let id = expect_str(arg(args, 2, "ui.dialog_trigger")?, "ui.dialog_trigger", 2)?;
        let label = escape_html(label);
        let id = escape_attr(id);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<button type=\"button\" class=\"flint-button\" onclick=\"flintDialog('{id}')\">{label}</button>\n"
        )))))
    })
}

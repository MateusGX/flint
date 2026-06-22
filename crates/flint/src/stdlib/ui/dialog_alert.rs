//! `ui.dialog_alert` — `dst = html ++ <self-contained alert dialog>`.
//! Renders a hidden dialog with a message and a single OK button. Show with
//! `ui.dialog_trigger`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.dialog_alert")?, "ui.dialog_alert", 0)?;
        let id = expect_str(arg(args, 1, "ui.dialog_alert")?, "ui.dialog_alert", 1)?;
        let title = expect_str(arg(args, 2, "ui.dialog_alert")?, "ui.dialog_alert", 2)?;
        let message = expect_str(arg(args, 3, "ui.dialog_alert")?, "ui.dialog_alert", 3)?;
        let id_a = escape_attr(id);
        let title = escape_html(title);
        let message = escape_html(message);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div id=\"{id_a}\" class=\"flint-dialog-overlay\" style=\"display:none\"><div class=\"flint-dialog\"><div class=\"flint-dialog-titlebar\"><span>{title}</span><button class=\"flint-dialog-close\" onclick=\"flintDialogClose('{id_a}')\" title=\"Fechar\">X</button></div><div class=\"flint-dialog-body\"><p class=\"flint-text\">{message}</p></div><div class=\"flint-dialog-footer\"><button type=\"button\" class=\"flint-button\" onclick=\"flintDialogClose('{id_a}')\">OK</button></div></div></div>\n"
        )))))
    })
}

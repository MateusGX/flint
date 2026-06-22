//! `ui.dialog_prompt` — `dst = html ++ <self-contained prompt dialog>`.
//! Renders a hidden dialog with a labeled text input that POSTs to `action`.
//! Show with `ui.dialog_trigger`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.dialog_prompt")?, "ui.dialog_prompt", 0)?;
        let id = expect_str(arg(args, 1, "ui.dialog_prompt")?, "ui.dialog_prompt", 1)?;
        let title = expect_str(arg(args, 2, "ui.dialog_prompt")?, "ui.dialog_prompt", 2)?;
        let label = expect_str(arg(args, 3, "ui.dialog_prompt")?, "ui.dialog_prompt", 3)?;
        let name = expect_str(arg(args, 4, "ui.dialog_prompt")?, "ui.dialog_prompt", 4)?;
        let action = expect_str(arg(args, 5, "ui.dialog_prompt")?, "ui.dialog_prompt", 5)?;
        let id_a = escape_attr(id);
        let title = escape_html(title);
        let label = escape_html(label);
        let name = escape_attr(name);
        let action = escape_attr(action);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div id=\"{id_a}\" class=\"flint-dialog-overlay\" style=\"display:none\"><div class=\"flint-dialog\"><div class=\"flint-dialog-titlebar\"><span>{title}</span><button class=\"flint-dialog-close\" onclick=\"flintDialogClose('{id_a}')\" title=\"Fechar\">X</button></div><form method=\"POST\" action=\"{action}\"><div class=\"flint-dialog-body\"><div class=\"flint-input\"><label>{label}</label><input name=\"{name}\" autofocus></div></div><div class=\"flint-dialog-footer\"><button type=\"submit\" class=\"flint-button\">OK</button><button type=\"button\" class=\"flint-button\" onclick=\"flintDialogClose('{id_a}')\">Cancelar</button></div></form></div></div>\n"
        )))))
    })
}

//! `ui.dialog` — `dst = html ++ <dialog overlay + titlebar opener>`.
//! Renders a hidden Windows XP-style modal dialog. Show it with
//! `ui.dialog_trigger`. Add body content with any `ui.*` natives, then close
//! with `ui.dialog_end`. Use `ui.action_bar`/`ui.action_bar_end` inside for
//! footer buttons; call `flintDialogClose('id')` in `onclick` to dismiss.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.dialog")?, "ui.dialog", 0)?;
        let id = expect_str(arg(args, 1, "ui.dialog")?, "ui.dialog", 1)?;
        let title = expect_str(arg(args, 2, "ui.dialog")?, "ui.dialog", 2)?;
        let id_a = escape_attr(id);
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div id=\"{id_a}\" class=\"flint-dialog-overlay\" style=\"display:none\"><div class=\"flint-dialog\"><div class=\"flint-dialog-titlebar\"><span>{title}</span><button class=\"flint-dialog-close\" onclick=\"flintDialogClose('{id_a}')\" title=\"Fechar\">X</button></div><div class=\"flint-dialog-body\">\n"
        )))))
    })
}

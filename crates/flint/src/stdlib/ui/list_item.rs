//! `ui.list_item` — `dst = html ++ <li>`. List item inside `ui.list`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.list_item")?, "ui.list_item", 0)?;
        let text = expect_str(arg(args, 1, "ui.list_item")?, "ui.list_item", 1)?;
        let text = escape_html(text);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<li class=\"flint-list-item\">{text}</li>\n"
        )))))
    })
}

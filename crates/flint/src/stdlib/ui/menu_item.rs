//! `ui.menu_item` — `dst = html ++ <menu link>`. Use inside
//! `ui.menu`/`ui.menu_end`. For the active/current item use `ui.menu_active`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.menu_item")?, "ui.menu_item", 0)?;
        let label = expect_str(arg(args, 1, "ui.menu_item")?, "ui.menu_item", 1)?;
        let href = expect_str(arg(args, 2, "ui.menu_item")?, "ui.menu_item", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<a class=\"flint-menu-item\" href=\"{href}\">{label}</a>\n"
        )))))
    })
}

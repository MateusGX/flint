//! `ui.menu_active` — `dst = html ++ <active menu item>`. Renders the current
//! page entry in the menu with inverted (blue) styling. Use inside
//! `ui.menu`/`ui.menu_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.menu_active")?, "ui.menu_active", 0)?;
        let label = expect_str(arg(args, 1, "ui.menu_active")?, "ui.menu_active", 1)?;
        let href = expect_str(arg(args, 2, "ui.menu_active")?, "ui.menu_active", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<a class=\"flint-menu-active\" href=\"{href}\">{label}</a>\n"
        )))))
    })
}

//! `ui.menu` — `dst = html ++ <vertical nav menu opener>`. Renders a sidebar
//! navigation menu with a blue category header. Add links with `ui.menu_item`
//! or `ui.menu_active`, close with `ui.menu_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.menu")?, "ui.menu", 0)?;
        let title = expect_str(arg(args, 1, "ui.menu")?, "ui.menu", 1)?;
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<nav class=\"flint-menu\"><div class=\"flint-menu-title\">{title}</div>\n"
        )))))
    })
}

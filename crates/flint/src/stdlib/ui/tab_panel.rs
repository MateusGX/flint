//! `ui.tab_panel` — `dst = html ++ <tab panel opener>`. `id` must match the
//! `id` passed to the corresponding `ui.tab` button. Pair with
//! `ui.tab_panel_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_attr;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.tab_panel")?, "ui.tab_panel", 0)?;
        let id = expect_str(arg(args, 1, "ui.tab_panel")?, "ui.tab_panel", 1)?;
        let id = escape_attr(id);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div id=\"{id}\" class=\"flint-tab-panel\">\n"
        )))))
    })
}

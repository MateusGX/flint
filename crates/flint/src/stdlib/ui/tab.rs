//! `ui.tab` — `dst = html ++ <tab button>`. Use between `ui.tabs` and
//! `ui.tabs_body`. `id` must match the `id` passed to the corresponding
//! `ui.tab_panel`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.tab")?, "ui.tab", 0)?;
        let label = expect_str(arg(args, 1, "ui.tab")?, "ui.tab", 1)?;
        let id = expect_str(arg(args, 2, "ui.tab")?, "ui.tab", 2)?;
        let label = escape_html(label);
        let id = escape_attr(id);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<button type=\"button\" class=\"flint-tab-btn\" data-tab=\"{id}\">{label}</button>\n"
        )))))
    })
}

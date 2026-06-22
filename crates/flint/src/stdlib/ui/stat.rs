//! `ui.stat` — `dst = html ++ <KPI metric widget>`. Displays a large number
//! with a label underneath, suitable for dashboard summary cards.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.stat")?, "ui.stat", 0)?;
        let label = expect_str(arg(args, 1, "ui.stat")?, "ui.stat", 1)?;
        let value = expect_str(arg(args, 2, "ui.stat")?, "ui.stat", 2)?;
        let label = escape_html(label);
        let value = escape_html(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-stat\"><span class=\"flint-stat-value\">{value}</span><span class=\"flint-stat-label\">{label}</span></div>\n"
        )))))
    })
}

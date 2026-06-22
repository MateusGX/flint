//! `ui.breadcrumb_item` — `dst = html ++ <breadcrumb link>`. Use inside
//! `ui.breadcrumb`/`ui.breadcrumb_end`. CSS handles the `»` separator via
//! `::after` on all items except the last.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.breadcrumb_item")?, "ui.breadcrumb_item", 0)?;
        let label = expect_str(arg(args, 1, "ui.breadcrumb_item")?, "ui.breadcrumb_item", 1)?;
        let href = expect_str(arg(args, 2, "ui.breadcrumb_item")?, "ui.breadcrumb_item", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<a class=\"flint-bc-item\" href=\"{href}\">{label}</a>"
        )))))
    })
}

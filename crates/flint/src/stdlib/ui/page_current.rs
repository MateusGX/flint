//! `ui.page_current` — `dst = html ++ <current page indicator>`. Renders the
//! active page number without a link. Use inside `ui.pagination`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.page_current")?, "ui.page_current", 0)?;
        let label = expect_str(arg(args, 1, "ui.page_current")?, "ui.page_current", 1)?;
        let label = escape_html(label);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<span class=\"flint-page-item flint-page-current\">{label}</span>"
        )))))
    })
}

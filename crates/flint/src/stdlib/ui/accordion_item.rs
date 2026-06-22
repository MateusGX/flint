//! `ui.accordion_item` — `dst = html ++ <collapsible section opener>`. Emits
//! a clickable header button and opens the body div (hidden by default).
//! Pair with `ui.accordion_item_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.accordion_item")?, "ui.accordion_item", 0)?;
        let title = expect_str(arg(args, 1, "ui.accordion_item")?, "ui.accordion_item", 1)?;
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-accordion-item\"><button type=\"button\" class=\"flint-accordion-header\" onclick=\"flintAccordionToggle(this)\">{title}</button><div class=\"flint-accordion-body\" style=\"display:none\">\n"
        )))))
    })
}

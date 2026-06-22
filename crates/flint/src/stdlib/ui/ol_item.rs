//! `ui.ol_item` — `dst = html ++ <li>`. Numbered list item; use inside
//! `ui.ol`/`ui.ol_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.ol_item")?, "ui.ol_item", 0)?;
        let text = expect_str(arg(args, 1, "ui.ol_item")?, "ui.ol_item", 1)?;
        let text = escape_html(text);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<li class=\"flint-ol-item\">{text}</li>\n"
        )))))
    })
}

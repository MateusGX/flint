//! `ui.text` — `dst = html ++ <paragraph>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.text")?, "ui.text", 0)?;
        let value = expect_str(arg(args, 1, "ui.text")?, "ui.text", 1)?;
        let value = escape_html(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<p class=\"flint-text\">{value}</p>\n"
        )))))
    })
}

//! `ui.title` — `dst = html ++ <heading>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.title")?, "ui.title", 0)?;
        let value = expect_str(arg(args, 1, "ui.title")?, "ui.title", 1)?;
        let value = escape_html(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<h2>{value}</h2>\n"
        )))))
    })
}

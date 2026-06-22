//! `ui.td` — `dst = html ++ <td>`. Table data cell; use inside `ui.tr`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.td")?, "ui.td", 0)?;
        let value = expect_str(arg(args, 1, "ui.td")?, "ui.td", 1)?;
        let value = escape_html(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<td>{value}</td>\n"
        )))))
    })
}

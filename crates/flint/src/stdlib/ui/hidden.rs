//! `ui.hidden` — `dst = html ++ <hidden input>`. Use inside a form for
//! CSRF tokens, record IDs, or any value not shown to the user.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_attr;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.hidden")?, "ui.hidden", 0)?;
        let name = expect_str(arg(args, 1, "ui.hidden")?, "ui.hidden", 1)?;
        let value = expect_str(arg(args, 2, "ui.hidden")?, "ui.hidden", 2)?;
        let name = escape_attr(name);
        let value = escape_attr(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<input type=\"hidden\" name=\"{name}\" value=\"{value}\">\n"
        )))))
    })
}

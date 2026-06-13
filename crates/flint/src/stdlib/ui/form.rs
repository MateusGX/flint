//! `ui.form` — `dst = html ++ <HTML form opener>`. Pair with `ui.form_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_attr;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.form")?, "ui.form", 0)?;
        let method = expect_str(arg(args, 1, "ui.form")?, "ui.form", 1)?;
        let action = expect_str(arg(args, 2, "ui.form")?, "ui.form", 2)?;
        let method = escape_attr(method);
        let action = escape_attr(action);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<form class=\"flint-form flint-card\" method=\"{method}\" action=\"{action}\">\n"
        )))))
    })
}

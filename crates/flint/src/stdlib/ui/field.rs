//! `ui.field` — `dst = html ++ <label/value display row>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.field")?, "ui.field", 0)?;
        let label = expect_str(arg(args, 1, "ui.field")?, "ui.field", 1)?;
        let value = expect_str(arg(args, 2, "ui.field")?, "ui.field", 2)?;
        let label = escape_html(label);
        let value = escape_html(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<dl class=\"flint-field\"><dt>{label}</dt><dd>{value}</dd></dl>\n"
        )))))
    })
}

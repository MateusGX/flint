//! `ui.input` — `dst = html ++ <labeled text input>`. Use inside a form
//! opened with `ui.form`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.input")?, "ui.input", 0)?;
        let label = expect_str(arg(args, 1, "ui.input")?, "ui.input", 1)?;
        let name = expect_str(arg(args, 2, "ui.input")?, "ui.input", 2)?;
        let label = escape_html(label);
        let name = escape_attr(name);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<label class=\"flint-input\"><span>{label}</span><input name=\"{name}\"></label>\n"
        )))))
    })
}

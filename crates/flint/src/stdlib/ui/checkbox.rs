//! `ui.checkbox` — `dst = html ++ <labeled checkbox>`. Use inside a form.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.checkbox")?, "ui.checkbox", 0)?;
        let label = expect_str(arg(args, 1, "ui.checkbox")?, "ui.checkbox", 1)?;
        let name = expect_str(arg(args, 2, "ui.checkbox")?, "ui.checkbox", 2)?;
        let value = expect_str(arg(args, 3, "ui.checkbox")?, "ui.checkbox", 3)?;
        let label = escape_html(label);
        let name = escape_attr(name);
        let value = escape_attr(value);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-check\"><label><input type=\"checkbox\" name=\"{name}\" value=\"{value}\"> {label}</label></div>\n"
        )))))
    })
}

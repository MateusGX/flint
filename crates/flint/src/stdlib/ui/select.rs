//! `ui.select` — `dst = html ++ <labeled select opener>`. Add options with
//! `ui.option`, then close with `ui.select_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.select")?, "ui.select", 0)?;
        let label = expect_str(arg(args, 1, "ui.select")?, "ui.select", 1)?;
        let name = expect_str(arg(args, 2, "ui.select")?, "ui.select", 2)?;
        let label = escape_html(label);
        let name = escape_attr(name);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-input\"><label>{label}</label><select name=\"{name}\">\n"
        )))))
    })
}

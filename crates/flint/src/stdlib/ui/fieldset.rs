//! `ui.fieldset` — `dst = html ++ <fieldset opener>`. Groups related form
//! fields under a `<legend>`. Pair with `ui.fieldset_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.fieldset")?, "ui.fieldset", 0)?;
        let legend = expect_str(arg(args, 1, "ui.fieldset")?, "ui.fieldset", 1)?;
        let legend = escape_html(legend);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<fieldset class=\"flint-fieldset\"><legend>{legend}</legend>\n"
        )))))
    })
}

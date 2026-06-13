//! `ui.submit` — `dst = html ++ <submit button>`. Use inside a form opened
//! with `ui.form`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.submit")?, "ui.submit", 0)?;
        let label = expect_str(arg(args, 1, "ui.submit")?, "ui.submit", 1)?;
        let label = escape_html(label);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<button class=\"flint-button\" type=\"submit\">{label}</button>\n"
        )))))
    })
}

//! `ui.caption` — `dst = html ++ <table caption>`. Must be called immediately
//! after `ui.table` before any `ui.tr`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.caption")?, "ui.caption", 0)?;
        let text = expect_str(arg(args, 1, "ui.caption")?, "ui.caption", 1)?;
        let text = escape_html(text);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<caption class=\"flint-caption\">{text}</caption>\n"
        )))))
    })
}

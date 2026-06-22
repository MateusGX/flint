//! `ui.progress` — `dst = html ++ <progress bar>`. `value` and `max` are
//! string representations of integers; percentage is computed server-side.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::percent;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.progress")?, "ui.progress", 0)?;
        let value_str = expect_str(arg(args, 1, "ui.progress")?, "ui.progress", 1)?;
        let max_str = expect_str(arg(args, 2, "ui.progress")?, "ui.progress", 2)?;
        let pct = percent(value_str, max_str);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-progress\"><div class=\"flint-progress-bar\" style=\"width:{pct}%\"></div></div>\n"
        )))))
    })
}

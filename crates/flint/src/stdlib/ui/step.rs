//! `ui.step` — `dst = html ++ <wizard step>`. Use inside `ui.steps`.
//! Pass `"1"` as `active` to highlight the current step, `"0"` otherwise.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.step")?, "ui.step", 0)?;
        let label = expect_str(arg(args, 1, "ui.step")?, "ui.step", 1)?;
        let active = expect_str(arg(args, 2, "ui.step")?, "ui.step", 2)?;
        let label = escape_html(label);
        let class = if active == "1" {
            " flint-step-active"
        } else {
            ""
        };
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<li class=\"flint-step{class}\">{label}</li>\n"
        )))))
    })
}

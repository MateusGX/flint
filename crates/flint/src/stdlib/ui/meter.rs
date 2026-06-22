//! `ui.meter` — `dst = html ++ <level gauge>`. Like `ui.progress` but
//! color-coded by level: green (≤50 %), orange (≤75 %), red (>75 %).
//! `value` and `max` are string representations of integers.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::percent;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.meter")?, "ui.meter", 0)?;
        let value_str = expect_str(arg(args, 1, "ui.meter")?, "ui.meter", 1)?;
        let max_str = expect_str(arg(args, 2, "ui.meter")?, "ui.meter", 2)?;
        let pct = percent(value_str, max_str);
        let level = if pct <= 50 {
            "low"
        } else if pct <= 75 {
            "medium"
        } else {
            "high"
        };
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-meter\"><div class=\"flint-meter-bar flint-meter-{level}\" style=\"width:{pct}%\"></div></div>\n"
        )))))
    })
}

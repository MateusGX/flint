//! `ui.card` — `dst = html ++ <bordered panel opener with title>`. Pair with
//! `ui.card_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.card")?, "ui.card", 0)?;
        let title = expect_str(arg(args, 1, "ui.card")?, "ui.card", 1)?;
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<section class=\"flint-card\"><h2>{title}</h2><div class=\"flint-section\">\n"
        )))))
    })
}

//! `ui.card_end` — `dst = html ++ <close card panel>`. Closes a card opened
//! with `ui.card`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.card_end")?, "ui.card_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}</div></section>\n"
        )))))
    })
}

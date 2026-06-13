//! `ui.row` — `dst = html ++ <horizontal responsive layout opener>`. Pair
//! with `ui.row_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.row")?, "ui.row", 0)?;
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-row\">\n"
        )))))
    })
}

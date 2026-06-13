//! `ui.column` — `dst = html ++ <vertical layout opener>`. Pair with
//! `ui.column_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.column")?, "ui.column", 0)?;
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-column\">\n"
        )))))
    })
}

//! `ui.column_end` — `dst = html ++ <close column>`. Closes a column opened
//! with `ui.column`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.column_end")?, "ui.column_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!("{html}</div>\n")))))
    })
}

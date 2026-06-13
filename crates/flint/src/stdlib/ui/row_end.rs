//! `ui.row_end` — `dst = html ++ <close row>`. Closes a row opened with
//! `ui.row`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.row_end")?, "ui.row_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!("{html}</div>\n")))))
    })
}

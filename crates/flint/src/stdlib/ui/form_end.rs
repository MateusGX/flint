//! `ui.form_end` — `dst = html ++ <close form>`. Closes a form opened with
//! `ui.form`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.form_end")?, "ui.form_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!("{html}</form>\n")))))
    })
}

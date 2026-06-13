//! `ui.section_end` — `dst = html ++ <close section>`. Closes a section
//! opened with `ui.section`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.section_end")?, "ui.section_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!("{html}</section>\n")))))
    })
}

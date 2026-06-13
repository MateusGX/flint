//! `string.concat` — `dst = a ++ b`. Both arguments must be `str`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let a = expect_str(arg(args, 0, "string.concat")?, "string.concat", 0)?;
        let b = expect_str(arg(args, 1, "string.concat")?, "string.concat", 1)?;
        Ok(Some(Value::Str(Arc::from(format!("{a}{b}")))))
    })
}

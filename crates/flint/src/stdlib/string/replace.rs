use std::sync::Arc;

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.replace")?, "string.replace", 0)?;
        let from = expect_str(arg(args, 1, "string.replace")?, "string.replace", 1)?;
        let to = expect_str(arg(args, 2, "string.replace")?, "string.replace", 2)?;
        Ok(Some(Value::Str(Arc::from(s.replace(from, to)))))
    })
}

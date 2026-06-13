use std::sync::Arc;

use crate::stdlib::{arg, expect_int, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let n = expect_int(arg(args, 0, "string.from_int")?, "string.from_int", 0)?;
        Ok(Some(Value::Str(Arc::from(n.to_string()))))
    })
}

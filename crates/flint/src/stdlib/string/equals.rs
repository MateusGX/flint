//! `string.equals` — `dst = 1` if `a == b`, else `0`.

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let a = expect_str(arg(args, 0, "string.equals")?, "string.equals", 0)?;
        let b = expect_str(arg(args, 1, "string.equals")?, "string.equals", 1)?;
        Ok(Some(Value::Int(if a == b { 1 } else { 0 })))
    })
}

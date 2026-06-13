//! `string.len` — `dst` = the number of characters in `s`, as an `int`.

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.len")?, "string.len", 0)?;
        Ok(Some(Value::Int(s.chars().count() as i64)))
    })
}

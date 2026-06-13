use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.ends_with")?, "string.ends_with", 0)?;
        let suffix = expect_str(arg(args, 1, "string.ends_with")?, "string.ends_with", 1)?;
        Ok(Some(Value::Int(s.ends_with(suffix) as i64)))
    })
}

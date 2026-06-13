use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.contains")?, "string.contains", 0)?;
        let sub = expect_str(arg(args, 1, "string.contains")?, "string.contains", 1)?;
        Ok(Some(Value::Int(s.contains(sub) as i64)))
    })
}

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.to_int")?, "string.to_int", 0)?;
        let n = s
            .trim()
            .parse::<i64>()
            .map_err(|_| format!("'string.to_int' could not parse '{s}' as an integer"))?;
        Ok(Some(Value::Int(n)))
    })
}

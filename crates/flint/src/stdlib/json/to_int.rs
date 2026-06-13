//! `json.to_int` — `dst` = `j` as an `int`; runtime error if `j` isn't a JSON
//! integer.

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_json, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.to_int")?, "json.to_int", 0)?;
        let n = json
            .as_i64()
            .ok_or_else(|| "json.to_int: value is not an integer".to_string())?;
        Ok(Some(Value::Int(n)))
    })
}

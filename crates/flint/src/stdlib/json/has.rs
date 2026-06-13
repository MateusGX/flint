use crate::stdlib::{arg, expect_json, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.has")?, "json.has", 0)?;
        let key = expect_str(arg(args, 1, "json.has")?, "json.has", 1)?;
        let has = json.get(key).is_some();
        Ok(Some(Value::Int(has as i64)))
    })
}

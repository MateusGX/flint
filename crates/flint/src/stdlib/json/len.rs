use crate::stdlib::{arg, expect_json, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.len")?, "json.len", 0)?;
        let len = match json {
            serde_json::Value::Array(a) => a.len(),
            serde_json::Value::Object(o) => o.len(),
            serde_json::Value::String(s) => s.len(),
            _ => return Err("'json.len' expects an array, object or string".to_string()),
        };
        Ok(Some(Value::Int(len as i64)))
    })
}

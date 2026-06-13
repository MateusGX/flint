use std::sync::Arc;

use crate::stdlib::{arg, expect_json, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.type")?, "json.type", 0)?;
        let type_name = match json {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "bool",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => "object",
        };
        Ok(Some(Value::Str(Arc::from(type_name))))
    })
}

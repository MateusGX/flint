use std::sync::Arc;

use crate::stdlib::{arg, expect_json, native};
use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.keys")?, "json.keys", 0)?;
        let keys: Vec<Json> = match json {
            Json::Object(obj) => obj.keys().map(|k| Json::String(k.clone())).collect(),
            _ => return Err("'json.keys' expects an object".to_string()),
        };
        Ok(Some(Value::Json(Arc::new(Json::Array(keys)))))
    })
}

use std::sync::Arc;

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.split")?, "string.split", 0)?;
        let sep = expect_str(arg(args, 1, "string.split")?, "string.split", 1)?;
        let parts: Vec<Json> = s.split(sep).map(|p| Json::String(p.to_string())).collect();
        Ok(Some(Value::Json(Arc::new(Json::Array(parts)))))
    })
}

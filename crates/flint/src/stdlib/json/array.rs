//! `json.array` — `dst` = an empty array `[]`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::native;

pub(super) fn make() -> NativeFn {
    native(|_| Ok(Some(Value::Json(Arc::new(Json::Array(Vec::new()))))))
}

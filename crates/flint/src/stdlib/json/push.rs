//! `json.push` — `dst` = a copy of `j` (normalized into an array if needed)
//! with `value` appended. Copy-on-write: see the module doc on [`super`].

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::json::support::value_to_json;
use crate::stdlib::{arg, expect_json, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.push")?, "json.push", 0)?;
        let value = arg(args, 1, "json.push")?;
        let mut document = json.clone();
        if !document.is_array() {
            document = Json::Array(Vec::new());
        }
        document
            .as_array_mut()
            .expect("just normalized to an array")
            .push(value_to_json(value));
        Ok(Some(Value::Json(Arc::new(document))))
    })
}

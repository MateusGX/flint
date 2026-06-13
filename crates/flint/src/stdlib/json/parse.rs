//! `json.parse` — parses `s` (`str`) as JSON; runtime error if invalid.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "json.parse")?, "json.parse", 0)?;
        let value: Json =
            serde_json::from_str(s).map_err(|e| format!("json.parse: invalid JSON: {e}"))?;
        Ok(Some(Value::Json(Arc::new(value))))
    })
}

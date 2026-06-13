//! `json.from_str` — `dst` = the JSON string corresponding to `s` (`str`).

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "json.from_str")?, "json.from_str", 0)?;
        Ok(Some(Value::Json(Arc::new(Json::from(s)))))
    })
}

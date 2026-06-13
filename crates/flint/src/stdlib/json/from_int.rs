//! `json.from_int` — `dst` = the JSON number corresponding to `n` (`int`).

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::{arg, expect_int, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let n = expect_int(arg(args, 0, "json.from_int")?, "json.from_int", 0)?;
        Ok(Some(Value::Json(Arc::new(Json::from(n)))))
    })
}

use std::sync::Arc;

use crate::stdlib::{arg, expect_int, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let n = expect_int(arg(args, 0, "json.bool")?, "json.bool", 0)?;
        Ok(Some(Value::Json(Arc::new(serde_json::Value::Bool(n != 0)))))
    })
}

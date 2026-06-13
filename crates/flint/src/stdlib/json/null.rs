use std::sync::Arc;

use crate::stdlib::native;
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|_args| Ok(Some(Value::Json(Arc::new(serde_json::Value::Null)))))
}

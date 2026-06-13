use std::sync::Arc;

use crate::stdlib::native;
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|_args| {
        let id = uuid::Uuid::new_v4().to_string();
        Ok(Some(Value::Str(Arc::from(id))))
    })
}

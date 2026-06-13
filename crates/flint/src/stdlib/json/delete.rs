use std::sync::Arc;

use crate::stdlib::{arg, expect_json, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.delete")?, "json.delete", 0)?;
        let key = expect_str(arg(args, 1, "json.delete")?, "json.delete", 1)?;
        let mut doc = json.clone();
        if let Some(obj) = doc.as_object_mut() {
            obj.remove(key);
        }
        Ok(Some(Value::Json(Arc::new(doc))))
    })
}

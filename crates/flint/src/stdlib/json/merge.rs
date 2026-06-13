use std::sync::Arc;

use crate::stdlib::{arg, expect_json, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let base = expect_json(arg(args, 0, "json.merge")?, "json.merge", 0)?;
        let patch = expect_json(arg(args, 1, "json.merge")?, "json.merge", 1)?;
        let mut result = base.clone();
        if let (Some(base_obj), Some(patch_obj)) = (result.as_object_mut(), patch.as_object()) {
            for (k, v) in patch_obj {
                base_obj.insert(k.clone(), v.clone());
            }
        }
        Ok(Some(Value::Json(Arc::new(result))))
    })
}

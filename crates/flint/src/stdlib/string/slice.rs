use std::sync::Arc;

use crate::stdlib::{arg, expect_int, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.slice")?, "string.slice", 0)?;
        let start = expect_int(arg(args, 1, "string.slice")?, "string.slice", 1)?;
        let end = expect_int(arg(args, 2, "string.slice")?, "string.slice", 2)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let start = start.clamp(0, len) as usize;
        let end = end.clamp(0, len) as usize;
        let sliced: String = chars[start.min(end)..end.max(start)].iter().collect();
        Ok(Some(Value::Str(Arc::from(sliced))))
    })
}

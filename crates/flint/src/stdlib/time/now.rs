use crate::stdlib::native;
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|_args| {
        let ms = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => i64::try_from(duration.as_millis())
                .map_err(|_| "'time.now' result is outside the int range".to_string())?,
            Err(_) => 0,
        };
        Ok(Some(Value::Int(ms)))
    })
}

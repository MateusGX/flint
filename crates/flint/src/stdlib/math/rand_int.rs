use crate::stdlib::{arg, expect_int, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let min = expect_int(arg(args, 0, "math.rand_int")?, "math.rand_int", 0)?;
        let max = expect_int(arg(args, 1, "math.rand_int")?, "math.rand_int", 1)?;
        if min > max {
            return Err(format!(
                "'math.rand_int' min ({min}) must be <= max ({max})"
            ));
        }
        use rand::Rng;
        let n: i64 = rand::thread_rng().gen_range(min..=max);
        Ok(Some(Value::Int(n)))
    })
}

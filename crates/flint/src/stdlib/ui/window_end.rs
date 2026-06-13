//! `ui.window_end` — `dst = html ++ <close stack, frame, body, document>`.
//! Closes a window opened with `ui.window`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.window_end")?, "ui.window_end", 0)?;
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}</div></section></main>\n</body>\n</html>\n"
        )))))
    })
}

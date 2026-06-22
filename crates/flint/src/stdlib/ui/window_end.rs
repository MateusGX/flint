//! `ui.window_end` — `dst = html ++ <close stack, frame, body, document>`.
//! Closes a window opened with `ui.window`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.window_end", "</div></div></main>\n</body>\n</html>\n")
}

//! `ui.steps_end` — `dst = html ++ </ol>`. Closes a steps bar opened with
//! `ui.steps`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.steps_end", "</ol>\n")
}

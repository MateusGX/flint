//! `ui.main_end` — `dst = html ++ </div>`. Closes a main column opened with
//! `ui.main`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.main_end", "</div>\n")
}

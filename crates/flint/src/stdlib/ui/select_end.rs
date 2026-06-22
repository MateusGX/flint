//! `ui.select_end` — `dst = html ++ </select></div>`. Closes a select opened
//! with `ui.select`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.select_end", "</select></div>\n")
}

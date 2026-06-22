//! `ui.tr_end` — `dst = html ++ </tr>`. Closes a row opened with `ui.tr`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tr_end", "</tr>\n")
}

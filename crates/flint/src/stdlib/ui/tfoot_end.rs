//! `ui.tfoot_end` — `dst = html ++ </tfoot>`. Closes a table footer opened
//! with `ui.tfoot`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tfoot_end", "</tfoot>\n")
}

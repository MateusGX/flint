//! `ui.row_end` — `dst = html ++ <close row>`. Closes a row opened with
//! `ui.row`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.row_end", "</div>\n")
}

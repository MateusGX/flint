//! `ui.column_end` — `dst = html ++ <close column>`. Closes a column opened
//! with `ui.column`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.column_end", "</div>\n")
}

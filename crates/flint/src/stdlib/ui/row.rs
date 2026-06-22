//! `ui.row` — `dst = html ++ <horizontal responsive layout opener>`. Pair
//! with `ui.row_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.row", "<div class=\"flint-row\">\n")
}

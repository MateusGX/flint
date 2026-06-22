//! `ui.column` — `dst = html ++ <vertical layout opener>`. Pair with
//! `ui.column_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.column", "<div class=\"flint-column\">\n")
}

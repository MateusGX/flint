//! `ui.tfoot` — `dst = html ++ <tfoot opener>`. Table footer section for
//! totals and summary rows. Add rows with `ui.tr`/`ui.tr_end` + `ui.td`.
//! Pair with `ui.tfoot_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tfoot", "<tfoot class=\"flint-tfoot\">\n")
}

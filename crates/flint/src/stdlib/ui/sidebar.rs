//! `ui.sidebar` — `dst = html ++ <sidebar column opener>`. Use inside
//! `ui.layout`/`ui.layout_end`. Pair with `ui.sidebar_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.sidebar", "<div class=\"flint-sidebar\">\n")
}

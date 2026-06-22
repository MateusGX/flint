//! `ui.main` — `dst = html ++ <main content column opener>`. Use inside
//! `ui.layout`/`ui.layout_end` after `ui.sidebar_end`. Pair with
//! `ui.main_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.main", "<div class=\"flint-main\">\n")
}

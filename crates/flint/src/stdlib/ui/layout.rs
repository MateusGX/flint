//! `ui.layout` — `dst = html ++ <two-column layout opener>`. Opens a
//! sidebar + main content container. Add a `ui.sidebar`/`ui.sidebar_end`
//! column then a `ui.main`/`ui.main_end` column, then close with
//! `ui.layout_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.layout", "<div class=\"flint-layout\">\n")
}

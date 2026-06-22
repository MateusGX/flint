//! `ui.navbar` — `dst = html ++ <nav bar opener>`. Emits a horizontal
//! navigation bar. Add links with `ui.nav_item`, close with `ui.navbar_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.navbar", "<nav class=\"flint-navbar\">\n")
}

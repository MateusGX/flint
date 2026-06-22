//! `ui.tabs` — `dst = html ++ <tabs container opener + tab bar opener>`.
//! Add tab buttons with `ui.tab`, then call `ui.tabs_body` to switch to
//! panels, add panels with `ui.tab_panel`/`ui.tab_panel_end`, then close
//! the whole group with `ui.tabs_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal(
        "ui.tabs",
        "<div class=\"flint-tabs\"><div class=\"flint-tab-bar\">\n",
    )
}

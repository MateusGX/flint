//! `ui.tabs_body` — `dst = html ++ <close tab bar, open panels area>`.
//! Separates the tab buttons from the tab panels. Call after all `ui.tab`
//! buttons and before the first `ui.tab_panel`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tabs_body", "</div><div class=\"flint-tab-panels\">\n")
}

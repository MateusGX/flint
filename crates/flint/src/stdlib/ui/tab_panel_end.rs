//! `ui.tab_panel_end` — `dst = html ++ </div>`. Closes a panel opened with
//! `ui.tab_panel`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tab_panel_end", "</div>\n")
}

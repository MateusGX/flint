//! `ui.action_bar_end` — `dst = html ++ </div>`. Closes an action bar opened
//! with `ui.action_bar`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.action_bar_end", "</div>\n")
}

//! `ui.action_bar` — `dst = html ++ <action bar opener>`. Bottom strip for
//! primary form actions (Save, Cancel). Pair with `ui.action_bar_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.action_bar", "<div class=\"flint-action-bar\">\n")
}

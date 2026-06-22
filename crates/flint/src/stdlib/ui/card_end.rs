//! `ui.card_end` — `dst = html ++ <close card panel>`. Closes a card opened
//! with `ui.card`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.card_end", "</div></div></section>\n")
}

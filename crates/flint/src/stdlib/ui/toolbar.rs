//! `ui.toolbar` — `dst = html ++ <toolbar opener>`. Horizontal strip for
//! grouping action buttons. Pair with `ui.toolbar_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.toolbar", "<div class=\"flint-toolbar\">\n")
}

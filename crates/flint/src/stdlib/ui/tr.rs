//! `ui.tr` — `dst = html ++ <tr>`. Pair with `ui.tr_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tr", "<tr>\n")
}

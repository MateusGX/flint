//! `ui.divider` — `dst = html ++ <horizontal rule>`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.divider", "<hr class=\"flint-divider\">\n")
}

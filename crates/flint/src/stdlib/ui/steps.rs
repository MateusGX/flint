//! `ui.steps` — `dst = html ++ <wizard step bar opener>`. Add steps with
//! `ui.step`, close with `ui.steps_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.steps", "<ol class=\"flint-steps\">\n")
}

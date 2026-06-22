//! `ui.breadcrumb` — `dst = html ++ <breadcrumb nav opener>`. Add links with
//! `ui.breadcrumb_item`, close with `ui.breadcrumb_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.breadcrumb", "<nav class=\"flint-breadcrumb\">")
}

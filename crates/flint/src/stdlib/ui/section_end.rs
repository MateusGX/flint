//! `ui.section_end` — `dst = html ++ <close section>`. Closes a section
//! opened with `ui.section`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.section_end", "</section>\n")
}

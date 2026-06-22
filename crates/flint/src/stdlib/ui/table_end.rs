//! `ui.table_end` — `dst = html ++ </table>`. Closes a table opened with
//! `ui.table`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.table_end", "</table>\n")
}

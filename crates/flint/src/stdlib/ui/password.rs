//! `ui.password` — `dst = html ++ <labeled password input>`. Use inside a
//! form; renders `<input type="password">`.

use crate::vm::NativeFn;

use super::support::labeled_input;

pub(super) fn make() -> NativeFn {
    labeled_input("ui.password", Some("password"))
}

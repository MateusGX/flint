//! `debug.print` — prints its arguments to `stdout`, space-separated, each
//! converted to its textual representation.

use crate::vm::NativeFn;

use crate::stdlib::native;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let rendered: Vec<String> = args.iter().map(|v| v.to_string()).collect();
        println!("{}", rendered.join(" "));
        Ok(None)
    })
}

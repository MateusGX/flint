//! Standard library of native functions for Flint programs.
//!
//! These are the "pure" natives — they only look at their arguments and
//! return a value, with no access to per-request state (compare with
//! `crate::http`'s `http.*` natives, which are scoped to a single request).
//!
//! # Layout: one file per native
//!
//! Each namespace (`debug`, `string`, `json`, ...) is a directory whose
//! `mod.rs` does only two things — declare one `mod` per native and expose
//! `pub fn register(registry: &mut NativeRegistry)` — while every native's
//! actual implementation lives alone in its own file as a `fn make() ->
//! NativeFn`. Opening `json::set` means opening `json/set.rs`: nothing else
//! is there. This keeps adding, debugging, editing and testing a single
//! native a one-file affair, and `mod.rs` reads as a flat index of "what's
//! in this namespace."
//!
//! # Adding a new namespace
//!
//! 1. Create a directory (e.g. `src/stdlib/time/`) whose `mod.rs` exposes
//!    `pub fn register(registry: &mut NativeRegistry)`, mirroring
//!    [`debug`], [`string`] or [`json`] — one `mod name;` plus one
//!    `registry.register_exact("time.name", arg_count, name::make())` per
//!    fixed-arity native.
//! 2. Call it from [`register_all`] below.
//!
//! That's the entire contract — no changes to the VM, compiler, or any other
//! module are needed to add a namespace (or a single native) of natives.

mod crypto;
mod debug;
mod env;
mod json;
mod math;
mod string;
mod time;
mod ui;
pub use ui::{UI_CSS, UI_JS};

use crate::vm::{NativeFn, NativeRegistry, Value};

/// Registers the complete standard library.
pub fn register_all(registry: &mut NativeRegistry) {
    debug::register(registry);
    string::register(registry);
    json::register(registry);
    math::register(registry);
    time::register(registry);
    env::register(registry);
    crypto::register(registry);
    ui::register(registry);
}

/// Boxes a closure as a [`NativeFn`]. A small convenience so each module's
/// `register` reads as a flat list of `registry.register_exact(name, arity,
/// native(...))` calls without repeating the trait-object boilerplate.
pub(crate) fn native(
    f: impl Fn(&[Value]) -> Result<Option<Value>, String> + Send + Sync + 'static,
) -> NativeFn {
    Box::new(f)
}

/// Fetches argument `idx`, producing a consistent "wrong arity" error if the
/// caller didn't pass enough arguments.
pub(crate) fn arg<'a>(args: &'a [Value], idx: usize, native: &str) -> Result<&'a Value, String> {
    args.get(idx).ok_or_else(|| {
        format!(
            "'{native}' expects at least {} argument(s), got {}",
            idx + 1,
            args.len()
        )
    })
}

pub(crate) fn expect_str<'a>(
    value: &'a Value,
    native: &str,
    position: usize,
) -> Result<&'a str, String> {
    value.as_str().ok_or_else(|| {
        format!(
            "'{native}' expects a string as argument {}, found '{}'",
            position + 1,
            value.type_name()
        )
    })
}

pub(crate) fn expect_int(value: &Value, native: &str, position: usize) -> Result<i64, String> {
    value.as_int().ok_or_else(|| {
        format!(
            "'{native}' expects an integer as argument {}, found '{}'",
            position + 1,
            value.type_name()
        )
    })
}

pub(crate) fn expect_json<'a>(
    value: &'a Value,
    native: &str,
    position: usize,
) -> Result<&'a serde_json::Value, String> {
    value.as_json().ok_or_else(|| {
        format!(
            "'{native}' expects a json value as argument {}, found '{}'",
            position + 1,
            value.type_name()
        )
    })
}

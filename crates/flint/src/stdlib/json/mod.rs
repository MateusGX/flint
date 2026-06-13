//! `json.*` — construction, inspection and (copy-on-write) mutation of JSON
//! documents. JSON is represented in the VM as a single opaque `Value::Json`,
//! so all manipulation goes through these natives rather than new language
//! syntax — keeping the language itself unchanged while still making JSON a
//! first-class citizen at runtime.
//!
//! `json.set`/`json.push` return a *new* document rather than mutating in
//! place: `Value::Json` wraps an `Arc`, which may be shared (e.g. aliased
//! across registers via `mov`), so copy-on-write is the only safe option
//! without adding interior mutability to `Value`.

mod array;
mod bool_;
mod delete;
mod from_int;
mod from_str;
mod get;
mod has;
mod keys;
mod len;
mod merge;
mod null;
mod object;
mod parse;
mod push;
mod set;
mod stringify;
pub(crate) mod support;
mod to_int;
mod to_str;
mod type_;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("json.parse", 1, parse::make());
    registry.register_exact("json.stringify", 1, stringify::make());
    registry.register_exact("json.object", 0, object::make());
    registry.register_exact("json.array", 0, array::make());
    registry.register_exact("json.from_int", 1, from_int::make());
    registry.register_exact("json.from_str", 1, from_str::make());
    registry.register_exact("json.to_int", 1, to_int::make());
    registry.register_exact("json.to_str", 1, to_str::make());
    registry.register_exact("json.get", 2, get::make());
    registry.register_exact("json.set", 3, set::make());
    registry.register_exact("json.push", 2, push::make());
    registry.register_exact("json.len", 1, len::make());
    registry.register_exact("json.has", 2, has::make());
    registry.register_exact("json.type", 1, type_::make());
    registry.register_exact("json.delete", 2, delete::make());
    registry.register_exact("json.keys", 1, keys::make());
    registry.register_exact("json.bool", 1, bool_::make());
    registry.register_exact("json.null", 0, null::make());
    registry.register_exact("json.merge", 2, merge::make());
}

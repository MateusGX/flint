//! `string.*` — the minimal set of string operations needed to work with
//! text coming from requests, JSON, and route parameters without leaving the
//! assembly-style language (which has no built-in string syntax beyond
//! literals and concatenation via `mov`).

mod concat;
mod contains;
mod ends_with;
mod equals;
mod escape_html;
mod from_int;
mod from_value;
mod len;
mod replace;
mod slice;
mod split;
mod starts_with;
mod to_int;
mod to_lower;
mod to_upper;
mod trim;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("string.concat", 2, concat::make());
    registry.register_exact("string.len", 1, len::make());
    registry.register_exact("string.equals", 2, equals::make());
    registry.register_exact("string.contains", 2, contains::make());
    registry.register_exact("string.starts_with", 2, starts_with::make());
    registry.register_exact("string.ends_with", 2, ends_with::make());
    registry.register_exact("string.escape_html", 1, escape_html::make());
    registry.register_exact("string.slice", 3, slice::make());
    registry.register_exact("string.replace", 3, replace::make());
    registry.register_exact("string.split", 2, split::make());
    registry.register_exact("string.trim", 1, trim::make());
    registry.register_exact("string.to_upper", 1, to_upper::make());
    registry.register_exact("string.to_lower", 1, to_lower::make());
    registry.register_exact("string.to_int", 1, to_int::make());
    registry.register_exact("string.from_int", 1, from_int::make());
    registry.register_exact("string.from", 1, from_value::make());
}

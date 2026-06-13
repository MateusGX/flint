//! `http.*` — natives scoped to a single request/response exchange.
//!
//! These differ from `crate::stdlib`'s natives in one key way: they're not pure
//! functions of their arguments. Each one closes over the [`HttpExchange`]
//! for *this* request, so a handler can read what came in (`http.method`,
//! `http.param`, `http.json_body`, ...) and shape what goes out
//! (`http.set_status`, `http.json`, ...). `crate::http::dispatch` builds a fresh
//! registry — and therefore a fresh set of these closures — per request.
//!
//! Same one-file-per-native layout as `crate::stdlib` (see that module's `mod.rs`
//! for the rationale): each file below is a `fn make(&Arc<Mutex<HttpExchange>>)
//! -> NativeFn`, and [`register`] is the flat index tying names to them.
//! `support` holds what's shared — argument/type checking, and the `reader`/
//! `lookup` factories that the three-and-three request-reading natives
//! (`method`/`path`/`body`, `param`/`query`/`header`) are built from.

mod abort;
mod body;
mod cookie;
mod form;
mod header;
mod html;
mod json;
mod json_body;
mod method;
mod param;
mod path;
mod query;
mod redirect;
mod set_cookie;
mod set_header;
mod set_status;
mod support;
mod text;

use std::sync::{Arc, Mutex};

use crate::vm::NativeRegistry;

use crate::http::exchange::HttpExchange;

pub fn register(registry: &mut NativeRegistry, exchange: Arc<Mutex<HttpExchange>>) {
    registry.register_exact("http.method", 0, method::make(&exchange));
    registry.register_exact("http.path", 0, path::make(&exchange));
    registry.register_exact("http.body", 0, body::make(&exchange));

    registry.register_exact("http.param", 1, param::make(&exchange));
    registry.register_exact("http.query", 1, query::make(&exchange));
    registry.register_exact("http.header", 1, header::make(&exchange));
    registry.register_exact("http.cookie", 1, cookie::make(&exchange));

    registry.register_exact("http.json_body", 0, json_body::make(&exchange));
    registry.register_exact("http.form", 1, form::make(&exchange));
    registry.register_exact("http.set_status", 1, set_status::make(&exchange));
    registry.register_exact("http.set_header", 2, set_header::make(&exchange));
    registry.register_exact("http.set_cookie", 2, set_cookie::make(&exchange));
    registry.register_exact("http.text", 1, text::make(&exchange));
    registry.register_exact("http.html", 1, html::make(&exchange));
    registry.register_exact("http.json", 1, json::make(&exchange));
    registry.register_exact("http.redirect", 1, redirect::make(&exchange));
    registry.register_exact("http.abort", 0, abort::make(&exchange));
}

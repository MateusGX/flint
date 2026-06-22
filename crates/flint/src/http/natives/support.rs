//! Shared building blocks for `http.*` natives.
//!
//! `reader`/`lookup` are factories for the two recurring shapes among the
//! request-reading natives — "return one string field" and "look a key up in
//! one of the request's string maps" — so `method`/`path`/`body` and
//! `param`/`query`/`header` each reduce to a one-line call into one of them.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::vm::{NativeFn, Value};

use crate::http::exchange::HttpExchange;

pub(super) use crate::stdlib::{arg, expect_int, expect_json, expect_str, native};

/// A native that reads one piece of the request and returns it as a string —
/// `http.method`, `http.path`, `http.body`. Takes no arguments.
pub(super) fn reader(
    exchange: &Arc<Mutex<HttpExchange>>,
    select: impl Fn(&HttpExchange) -> String + Send + Sync + 'static,
) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |_args| {
        let exchange = exchange.lock().expect("exchange mutex poisoned");
        Ok(Some(Value::Str(Arc::from(select(&exchange).as_str()))))
    })
}

/// A native that looks up a single key in one of the request's string maps —
/// `http.param`, `http.query`, `http.header`. Returns `""` for a missing key
/// (rather than failing the handler) since "absent" is a routine, expected
/// case for these — e.g. an optional query parameter.
pub(super) fn lookup(
    name: &'static str,
    exchange: &Arc<Mutex<HttpExchange>>,
    select: impl Fn(&HttpExchange) -> &HashMap<String, String> + Send + Sync + 'static,
    normalize_key: impl Fn(&str) -> String + Send + Sync + 'static,
) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let key = expect_str(arg(args, 0, name)?, name, 0)?;
        let key = normalize_key(key);
        let exchange = exchange.lock().expect("exchange mutex poisoned");
        let value = select(&exchange).get(&key).cloned().unwrap_or_default();
        Ok(Some(Value::Str(Arc::from(value.as_str()))))
    })
}

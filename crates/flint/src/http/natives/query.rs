//! `http.query` — `dst` = the query-string parameter `name` (`str`); `""` if
//! absent.

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::lookup;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    lookup("http.query", exchange, |ex| &ex.query, |k| k.to_string())
}

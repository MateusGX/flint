//! `http.method` — `dst` = the request's HTTP method (`str`, e.g. `"GET"`).

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::reader;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    reader(exchange, |ex| ex.method.clone())
}

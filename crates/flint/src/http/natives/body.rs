//! `http.body` — `dst` = the request body, decoded as UTF-8 (`str`).

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::reader;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    reader(exchange, |ex| ex.body.clone())
}

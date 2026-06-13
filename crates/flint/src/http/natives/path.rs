//! `http.path` — `dst` = the request's path (`str`, e.g. `"/users/42"`).

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::reader;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    reader(exchange, |ex| ex.path.clone())
}

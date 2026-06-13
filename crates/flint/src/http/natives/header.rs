//! `http.header` — `dst` = the header `name` (`str`, case-insensitive
//! comparison); `""` if absent.

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::lookup;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    lookup(
        "http.header",
        exchange,
        |ex| &ex.request_headers,
        |k| k.to_ascii_lowercase(),
    )
}

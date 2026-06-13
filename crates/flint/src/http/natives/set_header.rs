//! `http.set_header` — appends a `(name, value)` header (`str`, `str`) to
//! the response. Stacks rather than replaces — repeated calls add headers.

use std::sync::{Arc, Mutex};

use axum::http::{HeaderName, HeaderValue};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_str, native};

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let name = expect_str(arg(args, 0, "http.set_header")?, "http.set_header", 0)?;
        let value = expect_str(arg(args, 1, "http.set_header")?, "http.set_header", 1)?;
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| format!("'http.set_header' received an invalid header name: {e}"))?;
        HeaderValue::from_str(value)
            .map_err(|e| format!("'http.set_header' received an invalid header value: {e}"))?;
        exchange
            .lock()
            .expect("exchange mutex poisoned")
            .response
            .headers
            .push((name.to_string(), value.to_string()));
        Ok(None)
    })
}

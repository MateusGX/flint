use std::sync::{Arc, Mutex};

use axum::http::HeaderValue;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_str, native};
use crate::vm::NativeFn;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let url = expect_str(arg(args, 0, "http.redirect")?, "http.redirect", 0)?.to_string();
        HeaderValue::from_str(&url)
            .map_err(|e| format!("'http.redirect' received an invalid URL/header value: {e}"))?;
        let mut ex = exchange.lock().expect("exchange mutex poisoned");
        ex.response.status = 302;
        ex.response.headers.push(("location".to_string(), url));
        Ok(None)
    })
}

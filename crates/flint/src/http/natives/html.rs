//! `http.html` — sets the response body to HTML (`str`); fills in
//! `Content-Type: text/html; charset=utf-8` if the handler hasn't set one.

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::{HttpExchange, ResponseBody};
use crate::http::natives::support::{arg, expect_str, native};

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let body = expect_str(arg(args, 0, "http.html")?, "http.html", 0)?;
        exchange
            .lock()
            .expect("exchange mutex poisoned")
            .response
            .body = ResponseBody::Html(body.to_string());
        Ok(None)
    })
}

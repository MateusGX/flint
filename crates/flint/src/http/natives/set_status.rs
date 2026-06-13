//! `http.set_status` — sets the response status to `code` (`int`); must be
//! a valid HTTP status code.

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_int, native};

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let code = expect_int(arg(args, 0, "http.set_status")?, "http.set_status", 0)?;
        let code = u16::try_from(code)
            .ok()
            .filter(|code| axum::http::StatusCode::from_u16(*code).is_ok())
            .ok_or_else(|| {
                format!("'http.set_status' was given {code}, which is not a valid HTTP status code")
            })?;
        exchange
            .lock()
            .expect("exchange mutex poisoned")
            .response
            .status = code;
        Ok(None)
    })
}

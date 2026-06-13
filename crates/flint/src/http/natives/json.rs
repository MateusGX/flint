//! `http.json` — sets the response body to JSON (`json`); fills in
//! `Content-Type: application/json` if the handler hasn't set one. Replaces
//! any body set by an earlier `http.text`/`http.json` call.

use std::sync::{Arc, Mutex};

use crate::vm::NativeFn;

use crate::http::exchange::{HttpExchange, ResponseBody};
use crate::http::natives::support::{arg, expect_json, native};

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let json = expect_json(arg(args, 0, "http.json")?, "http.json", 0)?;
        exchange
            .lock()
            .expect("exchange mutex poisoned")
            .response
            .body = ResponseBody::Json(json.clone());
        Ok(None)
    })
}

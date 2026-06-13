//! `http.json_body` — `dst` = the request body parsed as JSON (`json`);
//! runtime error if it isn't valid JSON.

use std::sync::{Arc, Mutex};

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::native;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |_args| {
        let exchange = exchange.lock().expect("exchange mutex poisoned");
        let value: Json = serde_json::from_str(&exchange.body).map_err(|e| {
            format!("'http.json_body' failed: the request body is not valid JSON ({e})")
        })?;
        Ok(Some(Value::Json(Arc::new(value))))
    })
}

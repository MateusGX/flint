use std::sync::Arc;
use std::sync::Mutex;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_str, native};
use crate::vm::NativeFn;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let name = expect_str(arg(args, 0, "http.cookie")?, "http.cookie", 0)?;
        let ex = exchange.lock().expect("exchange mutex poisoned");
        let cookie_header = ex
            .request_headers
            .get("cookie")
            .cloned()
            .unwrap_or_default();
        let value = parse_cookie(&cookie_header, name).unwrap_or_default();
        Ok(Some(crate::vm::Value::Str(Arc::from(value))))
    })
}

fn parse_cookie(header: &str, name: &str) -> Option<String> {
    for pair in header.split(';') {
        let pair = pair.trim();
        if let Some((k, v)) = pair.split_once('=') {
            if k.trim() == name {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

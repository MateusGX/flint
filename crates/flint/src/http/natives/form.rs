use std::sync::Arc;
use std::sync::Mutex;

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_str, native};
use crate::vm::NativeFn;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let field = expect_str(arg(args, 0, "http.form")?, "http.form", 0)?;
        let ex = exchange.lock().expect("exchange mutex poisoned");
        let value = form_urlencoded::parse(ex.body.as_bytes())
            .find(|(k, _)| k == field)
            .map(|(_, v)| v.into_owned())
            .unwrap_or_default();
        Ok(Some(crate::vm::Value::Str(Arc::from(value))))
    })
}

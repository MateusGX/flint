use std::sync::{Arc, Mutex};

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::native;
use crate::vm::NativeFn;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |_args| {
        exchange.lock().expect("exchange mutex poisoned").aborted = true;
        Err("__flint_abort__".to_string())
    })
}

use std::sync::{Arc, Mutex};

use crate::http::exchange::HttpExchange;
use crate::http::natives::support::{arg, expect_str, native};
use crate::vm::NativeFn;

pub(super) fn make(exchange: &Arc<Mutex<HttpExchange>>) -> NativeFn {
    let exchange = Arc::clone(exchange);
    native(move |args| {
        let name = expect_str(arg(args, 0, "http.set_cookie")?, "http.set_cookie", 0)?.to_string();
        let value = expect_str(arg(args, 1, "http.set_cookie")?, "http.set_cookie", 1)?.to_string();
        validate_cookie_name(&name)?;
        validate_cookie_value(&value)?;
        let mut ex = exchange.lock().expect("exchange mutex poisoned");
        ex.response
            .headers
            .push(("set-cookie".to_string(), format!("{name}={value}")));
        Ok(None)
    })
}

fn validate_cookie_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("'http.set_cookie' received an empty cookie name".to_string());
    }
    if name.bytes().any(|b| {
        b <= 0x20
            || b >= 0x7f
            || matches!(
                b,
                b'(' | b')'
                    | b'<'
                    | b'>'
                    | b'@'
                    | b','
                    | b';'
                    | b':'
                    | b'\\'
                    | b'"'
                    | b'/'
                    | b'['
                    | b']'
                    | b'?'
                    | b'='
                    | b'{'
                    | b'}'
            )
    }) {
        return Err(format!(
            "'http.set_cookie' received an invalid cookie name '{name}'"
        ));
    }
    Ok(())
}

fn validate_cookie_value(value: &str) -> Result<(), String> {
    if value
        .bytes()
        .any(|b| b <= 0x20 || b >= 0x7f || matches!(b, b';' | b',' | b'\\' | b'"'))
    {
        return Err("'http.set_cookie' received an invalid cookie value".to_string());
    }
    Ok(())
}

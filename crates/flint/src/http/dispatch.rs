//! Bridging *one* HTTP request to the VM and back — the part of the pipeline
//! [`crate::http::router`] hands off to once it's matched a route.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::vm::{NativeRegistry, Vm};
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};

use crate::http::exchange::HttpExchange;
use crate::http::exchange::ResponseBody;
use crate::http::natives;
use crate::http::response::response_to_axum;
use crate::log::LogLevel;

/// Builds an [`HttpExchange`] from the request, runs the handler with a
/// fresh `Vm` and `NativeRegistry` (cheap — see `Vm::call`'s docs on lazy
/// memory), and converts the resulting `HttpResponse` into an `axum`
/// response (via [`response_to_axum`]).
///
/// A fresh VM/registry per request is the simplest model: it makes "no state
/// leaks between requests" true by construction, and the cost is negligible
/// next to the network I/O the request itself involves.
pub(crate) async fn dispatch(
    program: Arc<crate::vm::Program>,
    handler_address: usize,
    method: Method,
    uri: Uri,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let query = parse_query(uri.query());
    let request_headers = headers
        .iter()
        .map(|(name, value)| {
            let value = String::from_utf8_lossy(value.as_bytes()).into_owned();
            (name.as_str().to_string(), value)
        })
        .collect();
    let body = String::from_utf8_lossy(&body).into_owned();

    let exchange = Arc::new(Mutex::new(HttpExchange::new(
        method.as_str().to_string(),
        uri.path().to_string(),
        params,
        query,
        request_headers,
        body,
    )));

    let mut registry = NativeRegistry::new();
    crate::stdlib::register_all(&mut registry);
    natives::register(&mut registry, Arc::clone(&exchange));

    let start = Instant::now();
    let mut vm = Vm::new(registry);
    let vm_result = vm.call(&program, handler_address);
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    let exchange = exchange.lock().expect("exchange mutex poisoned");

    if let Err(ref error) = vm_result {
        if error.message != "__flint_abort__" {
            if crate::log::get() >= LogLevel::Error {
                eprintln!(
                    "  {red}error{reset}  {bold}{} {}{reset}  500  {error}",
                    exchange.method,
                    exchange.path,
                    red = crate::log::RED,
                    reset = crate::log::RESET,
                    bold = crate::log::BOLD,
                );
            }
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Flint runtime error: {error}"),
            )
                .into_response();
        }
    }

    log_request(&exchange, handler_address, elapsed_ms);

    response_to_axum(&exchange.response)
}

/// Runs a compiled page handler with an empty synthetic GET request and returns
/// the HTML body it assembles. Used by static-site export so it shares the same
/// VM, stdlib, and `ui.*` rendering path as the HTTP runtime.
pub fn render_static_html(
    program: Arc<crate::vm::Program>,
    handler_address: usize,
    path: &str,
) -> Result<String, String> {
    let exchange = Arc::new(Mutex::new(HttpExchange::new(
        "GET".to_string(),
        path.to_string(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        String::new(),
    )));

    let mut registry = NativeRegistry::new();
    crate::stdlib::register_all(&mut registry);
    natives::register(&mut registry, Arc::clone(&exchange));

    let mut vm = Vm::new(registry);
    if let Err(error) = vm.call(&program, handler_address) {
        if error.message != "__flint_abort__" {
            return Err(format!("Flint runtime error: {error}"));
        }
    }

    let exchange = exchange.lock().expect("exchange mutex poisoned");
    if exchange.response.status != 200 {
        return Err(format!(
            "static page returned HTTP status {}",
            exchange.response.status
        ));
    }

    match &exchange.response.body {
        ResponseBody::Html(html) => Ok(html.clone()),
        ResponseBody::Empty => Err("static page produced an empty response".to_string()),
        ResponseBody::Text(_) => Err("static page produced text instead of HTML".to_string()),
        ResponseBody::Json(_) => Err("static page produced JSON instead of HTML".to_string()),
    }
}

fn log_request(exchange: &HttpExchange, handler_address: usize, elapsed_ms: f64) {
    use crate::log::{self, LogLevel};

    let level = log::get();
    let status = exchange.response.status;
    let is_slow = elapsed_ms > 1000.0;

    let should_log = match level {
        LogLevel::Info | LogLevel::Debug => true,
        LogLevel::Warn => is_slow || status >= 500,
        _ => false,
    };

    if !should_log {
        return;
    }

    let sc = log::status_color(status);
    let slow = if is_slow {
        format!("  {}⚠ slow{}", log::YELLOW, log::RESET)
    } else {
        String::new()
    };

    println!(
        "  {dim}→{reset}  {bold}{method} {path}{reset}  {sc}{status}{reset}  {dim}{elapsed_ms:.1}ms{reset}{slow}",
        dim = log::DIM,
        reset = log::RESET,
        bold = log::BOLD,
        method = exchange.method,
        path = exchange.path,
    );

    if level >= LogLevel::Debug {
        if !exchange.params.is_empty() {
            let mut pairs: Vec<_> = exchange
                .params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect();
            pairs.sort();
            println!(
                "       {}params{}   {}",
                log::DIM,
                log::RESET,
                pairs.join("  ")
            );
        }
        if !exchange.query.is_empty() {
            let mut pairs: Vec<_> = exchange
                .query
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect();
            pairs.sort();
            println!(
                "       {}query{}    {}",
                log::DIM,
                log::RESET,
                pairs.join("  ")
            );
        }
        if !exchange.body.is_empty() {
            println!(
                "       {}body{}     {} bytes",
                log::DIM,
                log::RESET,
                exchange.body.len()
            );
        }
        println!(
            "       {}handler{}  0x{handler_address:x}",
            log::DIM,
            log::RESET
        );
    }
}

/// Parses a URL query string (`a=1&b=2`) into a lookup map, the form
/// `http.query` reads from. Absent entirely → an empty map, matching how
/// `http.query` treats any other missing key (`""`, not an error).
pub(crate) fn parse_query(query: Option<&str>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(query) = query {
        for (key, value) in form_urlencoded::parse(query.as_bytes()) {
            params.insert(key.into_owned(), value.into_owned());
        }
    }
    params
}

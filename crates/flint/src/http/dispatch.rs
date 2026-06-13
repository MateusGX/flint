//! Bridging *one* HTTP request to the VM and back — the part of the pipeline
//! [`crate::http::router`] hands off to once it's matched a route.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::vm::{NativeRegistry, Vm};
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};

use crate::http::exchange::HttpExchange;
use crate::http::natives;
use crate::http::response::response_to_axum;

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

    let mut vm = Vm::new(registry);
    if let Err(error) = vm.call(&program, handler_address) {
        if error.message != "__flint_abort__" {
            tracing::error!(%error, "Flint handler raised a runtime error");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Flint runtime error: {error}"),
            )
                .into_response();
        }
    }

    let exchange = exchange.lock().expect("exchange mutex poisoned");
    response_to_axum(&exchange.response)
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

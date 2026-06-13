use std::collections::HashMap;

use axum::body::Body;

/// Per-request context shared between the HTTP server and the `http.*`
/// natives: the captured request, plus the response a handler builds while
/// it runs.
///
/// A fresh `HttpExchange` is created for every request and wrapped in
/// `Arc<Mutex<_>>` — that's what lets `http.*` natives (plain `Fn` closures
/// with deliberately no access to the VM) read and write it without any
/// borrow-checker conflicts. See `crate::http::natives` for how they use it,
/// and `crate::http::dispatch` for how it's built and converted back to an
/// `axum` response.
pub struct HttpExchange {
    pub method: String,
    pub path: String,
    /// Path parameters captured by the route pattern, e.g. `:id` in
    /// `/users/:id`.
    pub params: HashMap<String, String>,
    /// Query-string parameters, e.g. `?page=2` → `{"page": "2"}`.
    pub query: HashMap<String, String>,
    /// Request headers, keyed by lower-cased name (matching HTTP's
    /// case-insensitive header semantics).
    pub request_headers: HashMap<String, String>,
    /// The raw request body, decoded as UTF-8 (lossily, so a non-UTF-8 body
    /// doesn't fail the request — it just won't parse as JSON either).
    pub body: String,
    pub response: HttpResponse,
    /// Set by `http.abort` to signal the handler wants to terminate early
    /// with whatever response has been assembled so far.
    pub aborted: bool,
}

impl HttpExchange {
    pub fn new(
        method: String,
        path: String,
        params: HashMap<String, String>,
        query: HashMap<String, String>,
        request_headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            method,
            path,
            params,
            query,
            request_headers,
            body,
            response: HttpResponse::default(),
            aborted: false,
        }
    }
}

/// The response a handler assembles via `http.set_status`/`set_header`/
/// `text`/`json`. Defaults to `200 OK` with an empty body, so a handler that
/// only cares about the body doesn't need to touch the status at all.
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: ResponseBody,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status: 200,
            headers: Vec::new(),
            body: ResponseBody::Empty,
        }
    }
}

pub enum ResponseBody {
    Empty,
    Text(String),
    Html(String),
    Json(serde_json::Value),
}

impl ResponseBody {
    /// Renders this body into its `axum` representation, alongside the
    /// `Content-Type` it suggests for itself — `response_to_axum` decides
    /// whether to actually apply that suggestion (only if the handler hasn't
    /// already set one via `http.set_header`). Each variant knowing how to
    /// render itself is what lets adding a new body type stay local to this
    /// enum: extend it and this `match`, and `response_to_axum` needs no
    /// changes at all.
    pub(crate) fn render(&self) -> (Option<&'static str>, Body) {
        match self {
            ResponseBody::Empty => (None, Body::empty()),
            ResponseBody::Text(text) => {
                (Some("text/plain; charset=utf-8"), Body::from(text.clone()))
            }
            ResponseBody::Html(html) => {
                (Some("text/html; charset=utf-8"), Body::from(html.clone()))
            }
            ResponseBody::Json(value) => (Some("application/json"), Body::from(value.to_string())),
        }
    }
}

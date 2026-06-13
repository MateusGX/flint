//! Converting the [`HttpResponse`] a handler assembled into the `axum`
//! response that's actually sent.

use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::http::exchange::HttpResponse;

/// Renders `response.body` (via [`ResponseBody::render`](crate::http::exchange::ResponseBody::render),
/// which knows each variant's suggested `Content-Type`), then layers status
/// and headers on top — filling in the suggested `Content-Type` only if the
/// handler didn't already set one explicitly via `http.set_header`.
pub(crate) fn response_to_axum(response: &HttpResponse) -> Response {
    let status = StatusCode::from_u16(response.status).unwrap_or(StatusCode::OK);
    let (default_content_type, body) = response.body.render();

    let mut builder = Response::builder().status(status);

    let has_content_type = response
        .headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("content-type"));
    if !has_content_type {
        if let Some(content_type) = default_content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
    }
    for (name, value) in &response.headers {
        builder = builder.header(name.as_str(), value.as_str());
    }

    builder.body(body).unwrap_or_else(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Flint: failed to build response",
        )
            .into_response()
    })
}

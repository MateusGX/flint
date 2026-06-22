//! Integration tests for the HTTP layer: compile small Flint apps in
//! memory, build a router with [`flint::http::router`], and drive it with
//! `tower::ServiceExt::oneshot` — no real socket needed.

use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{HeaderValue, Request, StatusCode};
use flint::http::AppModule;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn app_module(source: &str) -> AppModule {
    let compiled = flint::lang::compile_app_source(source).expect("the test source should compile");
    AppModule {
        program: Arc::new(compiled.program),
        routes: compiled.routes,
        source_path: PathBuf::from("<test>"),
    }
}

fn page_module(source: &str, path: &str) -> AppModule {
    let page =
        flint::lang::compile_page_source(source, path, "pages").expect("the page should compile");
    let compiled = page
        .compile(std::path::Path::new("."))
        .expect("the page source should compile");
    AppModule {
        program: Arc::new(compiled.program),
        routes: compiled.routes,
        source_path: PathBuf::from(path),
    }
}

async fn send(
    modules: Vec<AppModule>,
    request: Request<Body>,
) -> (StatusCode, Vec<(String, String)>, Vec<u8>) {
    let response = flint::http::router(modules)
        .expect("test routes should be valid")
        .oneshot(request)
        .await
        .expect("router is infallible");
    let status = response.status();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap().to_string(),
            )
        })
        .collect();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("reading the body should succeed")
        .to_bytes()
        .to_vec();
    (status, headers, body)
}

fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

#[tokio::test]
async fn get_route_returns_text_response() {
    let modules = vec![app_module(
        r#"section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello from Flint!"
    ncall http.text, r0
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/hello")
        .body(Body::empty())
        .unwrap();
    let (status, headers, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        header(&headers, "content-type"),
        Some("text/plain; charset=utf-8")
    );
    assert_eq!(body, b"Hello from Flint!");
}

#[tokio::test]
async fn get_route_returns_html_response() {
    let modules = vec![app_module(
        r#"section .route
    GET "/" -> show_home

section .text
show_home:
    mov r0, "<h1>Hello</h1>"
    ncall http.html, r0
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let (status, headers, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        header(&headers, "content-type"),
        Some("text/html; charset=utf-8")
    );
    assert_eq!(body, b"<h1>Hello</h1>");
}

#[tokio::test]
async fn ui_page_renders_default_styled_controls() {
    let modules = vec![page_module(
        r#"section .route
    GET "/profile"

section .text
    mov r0, "name"
    ncallr r1, http.query, r0

section .render
    window "Profile"
        text "Rendered without writing HTML"
        card "Details"
            field "Name", r1
            btn   "API",  "/hello"
        end
    end
"#,
        "pages/profile.flint.ui",
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/profile?name=Ada")
        .body(Body::empty())
        .unwrap();
    let (status, headers, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        header(&headers, "content-type"),
        Some("text/html; charset=utf-8")
    );
    let body = String::from_utf8(body).unwrap();
    assert!(body.contains("<h1>Profile</h1>"), "{body}");
    assert!(body.contains("Rendered without writing HTML"), "{body}");
    assert!(body.contains("<dd>Ada</dd>"), "{body}");
    assert!(body.contains("class=\"flint-button\""), "{body}");
}

#[tokio::test]
async fn get_route_resolves_path_parameters_into_json_response() {
    let modules = vec![app_module(
        r#"section .route
    GET "/users/:id" -> show_user

section .text
show_user:
    mov r0, "id"
    ncallr r1, http.param, r0
    ncallr r2, json.object
    mov r3, "id"
    ncallr r2, json.set, r2, r3, r1
    ncall http.json, r2
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/users/42")
        .body(Body::empty())
        .unwrap();
    let (status, headers, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(header(&headers, "content-type"), Some("application/json"));
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({ "id": "42" }));
}

#[tokio::test]
async fn post_route_reads_json_body_and_sets_status() {
    let modules = vec![app_module(
        r#"section .route
    POST "/echo" -> echo_body

section .text
echo_body:
    ncallr r0, http.json_body
    ncallr r1, json.object
    mov r2, "received"
    ncallr r1, json.set, r1, r2, r0
    mov r2, 201
    ncall http.set_status, r2
    ncall http.json, r1
    ret
"#,
    )];

    let request = Request::builder()
        .method("POST")
        .uri("/echo")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"Ada"}"#))
        .unwrap();
    let (status, headers, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(header(&headers, "content-type"), Some("application/json"));
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({ "received": { "name": "Ada" } }));
}

#[tokio::test]
async fn invalid_response_header_is_a_runtime_error() {
    let modules = vec![app_module(
        r#"section .route
    GET "/bad-header" -> bad_header

section .text
bad_header:
    mov r0, "bad header"
    mov r1, "ok"
    ncall http.set_header, r0, r1
    mov r0, "done"
    ncall http.text, r0
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/bad-header")
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        String::from_utf8_lossy(&body).contains("invalid header name"),
        "{}",
        String::from_utf8_lossy(&body)
    );
}

#[tokio::test]
async fn request_headers_preserve_non_utf8_bytes_lossily() {
    let modules = vec![app_module(
        r#"section .route
    GET "/header" -> echo_header

section .text
echo_header:
    mov r0, "x-bin"
    ncallr r1, http.header, r0
    ncall http.text, r1
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/header")
        .header("x-bin", HeaderValue::from_bytes(b"\xff").unwrap())
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(String::from_utf8(body).unwrap(), "\u{fffd}");
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let modules = vec![app_module(
        r#"section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "hi"
    ncall http.text, r0
    ret
"#,
    )];

    let request = Request::builder()
        .method("GET")
        .uri("/does-not-exist")
        .body(Body::empty())
        .unwrap();
    let (status, _, _) = send(modules, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn handler_runtime_errors_become_500_responses() {
    let modules = vec![app_module(
        r#"section .route
    GET "/boom" -> boom

section .text
boom:
    ncallr r0, http.json_body
    ret
"#,
    )];

    // No body is sent, so `http.json_body` fails to parse "" as JSON — the
    // handler raises a runtime error, which the server turns into a 500
    // rather than letting it propagate or crash the process.
    let request = Request::builder()
        .method("GET")
        .uri("/boom")
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(modules, request).await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(String::from_utf8_lossy(&body).contains("Flint runtime error"));
}

#[tokio::test]
async fn routes_from_multiple_modules_are_all_registered() {
    let modules = vec![
        app_module(
            r#"section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "hi"
    ncall http.text, r0
    ret
"#,
        ),
        app_module(
            r#"section .route
    GET "/bye" -> say_bye

section .text
say_bye:
    mov r0, "bye"
    ncall http.text, r0
    ret
"#,
        ),
    ];

    let hello = Request::builder()
        .method("GET")
        .uri("/hello")
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(modules.clone(), hello).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, b"hi");

    let bye = Request::builder()
        .method("GET")
        .uri("/bye")
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(modules, bye).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, b"bye");
}

#[test]
fn duplicate_routes_are_rejected_before_axum_route_registration() {
    let modules = vec![
        app_module(
            r#"section .route
    GET "/dupe" -> first

section .text
first:
    ret
"#,
        ),
        app_module(
            r#"section .route
    GET "/dupe" -> second

section .text
second:
    ret
"#,
        ),
    ];

    let err = flint::http::router(modules).unwrap_err();
    assert!(
        err.to_string().contains("duplicate route GET /dupe"),
        "{err}"
    );
}

#[test]
fn structurally_conflicting_routes_are_rejected_before_axum_route_registration() {
    let modules = vec![
        app_module(
            r#"section .route
    GET "/users/:id" -> first

section .text
first:
    ret
"#,
        ),
        app_module(
            r#"section .route
    GET "/users/:name" -> second

section .text
second:
    ret
"#,
        ),
    ];

    let err = flint::http::router(modules).unwrap_err();
    assert!(
        err.to_string()
            .contains("conflicting route pattern GET /users/:name"),
        "{err}"
    );
}

#[test]
fn bare_route_directive_is_rejected() {
    let err = flint::lang::compile_app_source("handler:\n    ret\n\nroute GET \"/\" -> handler\n")
        .unwrap_err();
    assert!(err.to_string().contains("section .route"), "{err}");
}

//! Turning a `Vec<AppModule>` into a running (or buildable) `axum` server —
//! the framework's outermost layer, and the thing a Flint app's `main`
//! actually calls. What happens to *one* request once it's routed here lives
//! in [`crate::http::dispatch`].

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::lang::AppModule;
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::{HeaderMap, Method, Uri};
use axum::routing::{delete, get, head, options, patch, post, put, MethodRouter};
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::http::dispatch::dispatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterError {
    pub message: String,
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RouterError {}

/// Builds an `axum::Router` that dispatches to every route declared by
/// `modules`.
///
/// The result is a plain `axum::Router` — apply your own `tower::Layer`s
/// with `.layer(...)`, exactly as you would for any other axum app (request
/// logging via `tower_http::trace::TraceLayer`, auth, rate limiting, ...).
/// This is the framework's extension point for cross-cutting request
/// concerns: no bespoke middleware system to learn, just the `tower`
/// ecosystem every Rust backend developer already knows.
///
/// [`serve`] wraps this with a sensible default (`TraceLayer`) for the
/// common case.
pub fn router(modules: Vec<AppModule>) -> Result<Router, RouterError> {
    build_router(modules)
}

pub fn try_router(modules: Vec<AppModule>) -> Result<Router, RouterError> {
    build_router(modules)
}

fn build_router(modules: Vec<AppModule>) -> Result<Router, RouterError> {
    let mut router = Router::new();
    let mut seen = HashSet::new();
    let mut seen_shapes = HashSet::new();
    let mut by_path: BTreeMap<String, MethodRouter> = BTreeMap::new();

    for module in modules {
        for route in &module.routes {
            validate_runtime_route_path(&route.path)?;
            let key = (route.method.clone(), route.path.clone());
            let shape_key = (route.method.clone(), route_shape(&route.path));
            if !seen.insert(key) {
                return Err(RouterError {
                    message: format!("duplicate route {} {}", route.method, route.path),
                });
            }
            if !seen_shapes.insert(shape_key) {
                return Err(RouterError {
                    message: format!("conflicting route pattern {} {}", route.method, route.path),
                });
            }

            let program = Arc::clone(&module.program);
            let address = route.handler_address;

            // Each route gets its own closure capturing the program it
            // dispatches into and the address of its handler — `axum`
            // erases these into a uniform `MethodRouter` regardless of how
            // many routes we register or what they capture.
            let handler = move |method: Method,
                                uri: Uri,
                                path: Path<HashMap<String, String>>,
                                headers: HeaderMap,
                                body: Bytes| {
                let program = Arc::clone(&program);
                async move { dispatch(program, address, method, uri, path, headers, body).await }
            };

            let method_router = match route.method.as_str() {
                "GET" => get(handler),
                "POST" => post(handler),
                "PUT" => put(handler),
                "PATCH" => patch(handler),
                "DELETE" => delete(handler),
                "HEAD" => head(handler),
                "OPTIONS" => options(handler),
                other => {
                    return Err(RouterError {
                        message: format!("unknown HTTP method '{other}' for route {}", route.path),
                    });
                }
            };

            let path = route.path.clone();
            let method_router = match by_path.remove(&path) {
                Some(existing) => existing.merge(method_router),
                None => method_router,
            };
            by_path.insert(path, method_router);
        }
    }

    for (path, method_router) in by_path {
        router = router.route(&path, method_router);
    }

    Ok(router)
}

/// Builds the router (with [`router`]) and serves it on `addr`, applying
/// `TraceLayer::new_for_http()` for request logging — the framework's
/// default demonstration of the "extend via `tower::Layer`" pattern. Build
/// and layer your own router with [`router`] for full control.
pub async fn serve(modules: Vec<AppModule>, addr: SocketAddr) -> std::io::Result<()> {
    serve_with_ready(modules, addr, |_| {}).await
}

pub async fn serve_with_ready(
    modules: Vec<AppModule>,
    addr: SocketAddr,
    ready: impl FnOnce(SocketAddr),
) -> std::io::Result<()> {
    let app = try_router(modules)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    ready(addr);
    tracing::info!(%addr, "Flint listening");
    axum::serve(listener, app).await
}

fn validate_runtime_route_path(path: &str) -> Result<(), RouterError> {
    if !path.starts_with('/') {
        return Err(RouterError {
            message: format!("route path '{path}' must start with '/'"),
        });
    }
    if path.contains('?') || path.contains('#') {
        return Err(RouterError {
            message: format!("route path '{path}' must not contain query strings or fragments"),
        });
    }
    if path.chars().any(char::is_control) || path.chars().any(char::is_whitespace) {
        return Err(RouterError {
            message: format!("route path '{path}' must not contain whitespace"),
        });
    }
    if path != "/" && path.ends_with('/') {
        return Err(RouterError {
            message: format!("route path '{path}' must not end with '/'"),
        });
    }
    if path != "/" && path.split('/').skip(1).any(str::is_empty) {
        return Err(RouterError {
            message: format!("route path '{path}' must not contain empty segments"),
        });
    }
    for segment in path.split('/').skip(1) {
        if let Some(param) = segment.strip_prefix(':') {
            let mut chars = param.chars();
            let valid_start = chars
                .next()
                .is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic());
            let valid_rest = chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric());
            if !valid_start || !valid_rest {
                return Err(RouterError {
                    message: format!(
                        "route path '{path}' has invalid parameter segment '{segment}'"
                    ),
                });
            }
        }
    }
    Ok(())
}

fn route_shape(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if segment.starts_with(':') {
                ":".to_string()
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

//! HTTP server for Flint apps: turns compiled Flint modules with `route`
//! directives into a running `axum` server, bridging each request/response
//! to and from the VM.
//!
//! The pieces, and where to look if you're extending one of them:
//! - [`AppModule`] (re-exported from [`crate::lang`]) — a compiled `.fl`
//!   file plus the routes it declares. Built by `crate::lang::load_app_dir`.
//! - [`HttpExchange`] (`exchange` module) — the per-request bridge: captured
//!   request data in, response-being-built out.
//! - `natives` — registers `http.*`, the natives handlers use to read the
//!   request and shape the response. Follows the same one-file-per-native
//!   layout as [`crate::stdlib`] — see that module's `mod.rs` for the rationale.
//! - [`router`]/[`serve`] (`router` module) — turn a `Vec<AppModule>` into a
//!   running `axum` server.
//! - `dispatch` — the bridge for *one* request: builds an `HttpExchange`,
//!   runs it through a fresh `Vm`, converts the result back (`response`).
//!
//! ```no_run
//! # async fn run() -> std::io::Result<()> {
//! let modules = flint::lang::load_app_dir("api", ".").unwrap();
//! flint::http::serve(modules, "127.0.0.1:3000".parse().unwrap()).await
//! # }
//! ```

mod dispatch;
pub mod exchange;
pub mod natives;
mod response;
mod router;

pub use crate::lang::AppModule;
pub use dispatch::render_static_html;
pub use exchange::HttpExchange;
pub use router::{router, serve, serve_with_ready, RouterError};

use super::symbols::Symbols;
use super::CompileError;

/// HTTP methods a `route` directive may name. Kept as a flat list rather
/// than a parsed enum — the only thing the rest of the pipeline does with a
/// method is pass its (normalized) name on to the HTTP server's router.
const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

/// A `route METHOD "/path" -> handler` directive, with `handler` resolved to
/// a concrete instruction address (so the HTTP server can call straight into
/// the program without repeating name lookups per request).
#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    /// Normalized to uppercase, e.g. `"GET"`.
    pub method: String,
    pub path: String,
    pub handler: String,
    pub handler_address: usize,
}

pub(super) fn compile_route(
    method: &str,
    path: &str,
    handler: &str,
    line: usize,
    symbols: &Symbols,
) -> Result<Route, CompileError> {
    let normalized_method = method.to_uppercase();
    if !HTTP_METHODS.contains(&normalized_method.as_str()) {
        return Err(CompileError {
            line,
            message: format!(
                "unknown HTTP method '{method}' in route directive — expected one of {}",
                HTTP_METHODS.join(", ")
            ),
        });
    }
    validate_route_path(path).map_err(|message| CompileError { line, message })?;

    let handler_address = symbols
        .labels
        .get(handler)
        .copied()
        .ok_or_else(|| CompileError {
            line,
            message: format!(
                "route handler '{handler}' is not a defined label — define it with '{handler}:'"
            ),
        })?;

    Ok(Route {
        method: normalized_method,
        path: path.to_string(),
        handler: handler.to_string(),
        handler_address,
    })
}

pub(crate) fn validate_route_path(path: &str) -> Result<(), String> {
    if !path.starts_with('/') {
        return Err(format!("route path '{path}' must start with '/'"));
    }
    if path.contains('?') || path.contains('#') {
        return Err(format!(
            "route path '{path}' must not contain query strings or fragments"
        ));
    }
    if path.chars().any(char::is_control) || path.chars().any(char::is_whitespace) {
        return Err(format!("route path '{path}' must not contain whitespace"));
    }
    if path != "/" && path.ends_with('/') {
        return Err(format!("route path '{path}' must not end with '/'"));
    }
    if path != "/" && path.split('/').skip(1).any(str::is_empty) {
        return Err(format!(
            "route path '{path}' must not contain empty segments"
        ));
    }

    for segment in path.split('/').skip(1) {
        if let Some(param) = segment.strip_prefix(':') {
            let mut chars = param.chars();
            let valid_start = chars
                .next()
                .is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic());
            let valid_rest = chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric());
            if !valid_start || !valid_rest {
                return Err(format!(
                    "route path '{path}' has invalid parameter segment '{segment}'"
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Route;
    use crate::lang::compiler::{compile_app, CompileError, CompiledApp};
    use crate::lang::lexer::lex;
    use crate::lang::parser::parse;

    fn compile_app_source(source: &str) -> Result<CompiledApp, CompileError> {
        let tokens = lex(source).unwrap();
        let ast = parse(&tokens).unwrap();
        compile_app(&ast)
    }

    #[test]
    fn compiles_routes_resolving_handlers_to_addresses() {
        let app = compile_app_source(
            "route get \"/users\" -> list_users\nroute POST \"/users\" -> create_user\nlist_users:\n  ret\ncreate_user:\n  ret\n",
        )
        .unwrap();
        assert_eq!(
            app.routes,
            vec![
                Route {
                    method: "GET".to_string(),
                    path: "/users".to_string(),
                    handler: "list_users".to_string(),
                    handler_address: 0,
                },
                Route {
                    method: "POST".to_string(),
                    path: "/users".to_string(),
                    handler: "create_user".to_string(),
                    handler_address: 1,
                },
            ]
        );
    }

    #[test]
    fn reports_routes_with_undefined_handlers() {
        let err = compile_app_source("route GET \"/users\" -> missing\n").unwrap_err();
        assert!(
            err.message
                .contains("route handler 'missing' is not a defined label"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_routes_with_unknown_http_methods() {
        let err =
            compile_app_source("route FETCH \"/users\" -> handler\nhandler:\n  ret\n").unwrap_err();
        assert!(
            err.message.contains("unknown HTTP method 'FETCH'"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_invalid_route_paths() {
        let err =
            compile_app_source("route GET \"users\" -> handler\nhandler:\n  ret\n").unwrap_err();
        assert!(
            err.message.contains("must start with '/'"),
            "{}",
            err.message
        );

        let err = compile_app_source("route GET \"/users/:9\" -> handler\nhandler:\n  ret\n")
            .unwrap_err();
        assert!(
            err.message.contains("invalid parameter segment"),
            "{}",
            err.message
        );
    }
}

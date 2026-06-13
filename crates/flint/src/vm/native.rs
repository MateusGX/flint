use std::collections::HashMap;

use crate::vm::value::Value;

/// A Rust-implemented function callable from bytecode via `ncall`/`ncallr`.
///
/// Natives receive argument values by-value (already cloned out of their
/// registers) and return an optional result. They have no access to VM
/// internals — this keeps the call site free of borrow-checker conflicts and
/// keeps natives easy to test in isolation.
///
/// `Send + Sync` is required so a `NativeRegistry` (and thus a `Vm`) can be
/// built and used from any of the HTTP server's async worker threads.
pub type NativeFn = Box<dyn Fn(&[Value]) -> Result<Option<Value>, String> + Send + Sync>;

/// Lookup table from native function name (as used in `ncall name, ...`) to
/// its Rust implementation.
///
/// This registry intentionally starts empty — the VM doesn't ship any
/// natives of its own. The standard set lives in `crate::stdlib`
/// (`crate::stdlib::register_all`), and request-scoped natives (e.g. `http.*`)
/// are registered by `crate::http` per request. To add a new namespace of
/// natives, follow the `register(&mut NativeRegistry)` convention used by
/// those modules — see `crate::stdlib`'s `mod.rs` for the pattern.
#[derive(Default)]
pub struct NativeRegistry {
    functions: HashMap<String, NativeFn>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Registers a variadic native.
    ///
    /// Most natives should use [`register_exact`](Self::register_exact) so
    /// extra arguments are rejected consistently. This lower-level form is for
    /// intentionally variadic natives such as `debug.print`.
    pub fn register(&mut self, name: impl Into<String>, f: NativeFn) {
        self.functions.insert(name.into(), f);
    }

    pub fn register_exact(&mut self, name: impl Into<String>, expected: usize, f: NativeFn) {
        let name = name.into();
        self.functions
            .insert(name.clone(), exact_arity(name, expected, f));
    }

    pub fn get(&self, name: &str) -> Option<&NativeFn> {
        self.functions.get(name)
    }
}

fn exact_arity(name: String, expected: usize, f: NativeFn) -> NativeFn {
    Box::new(move |args| {
        if args.len() != expected {
            return Err(format!(
                "'{name}' expects exactly {expected} argument(s), got {}",
                args.len()
            ));
        }
        f(args)
    })
}

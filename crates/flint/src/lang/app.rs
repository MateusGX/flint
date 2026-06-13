//! Loading directories of `.fl` files as a multi-module HTTP app — the
//! basis of Flint's file/directory-based routing convention.
//!
//! Rather than "linking" multiple compiled programs together (which would
//! mean relocating instruction addresses and merging string constant pools),
//! each `.fl` file is compiled completely independently into its own
//! [`AppModule`]. The HTTP server then registers every module's routes on a
//! single router, each dispatching into its own program. This keeps the
//! compiler simple and lets every file be reasoned about — and erred about —
//! on its own.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::lang::compiler::Route;
use crate::lang::Error;

/// One independently-compiled `.fl` file: its bytecode plus the routes it
/// declares, ready to be registered on an HTTP router.
#[derive(Debug, Clone)]
pub struct AppModule {
    pub program: Arc<crate::vm::Program>,
    pub routes: Vec<Route>,
    pub source_path: PathBuf,
}

/// Failure to load an app directory: either an I/O problem, a `use` expansion
/// error, or a compile error in one of its `.fl` files. All variants are
/// tagged with the offending path so the author can find the file that needs
/// fixing.
#[derive(Debug)]
pub enum LoadError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Include {
        path: PathBuf,
        message: String,
    },
    Page {
        path: PathBuf,
        message: String,
    },
    Compile {
        path: PathBuf,
        source: Error,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            LoadError::Include { path, message } => {
                write!(f, "{}: {message}", path.display())
            }
            LoadError::Page { path, message } => {
                write!(f, "{}: page error: {message}", path.display())
            }
            LoadError::Compile { path, source } => write!(f, "{}: {source}", path.display()),
        }
    }
}

impl std::error::Error for LoadError {}

/// Compiles every `.fl` file directly inside `routes_dir` (not recursively)
/// into its own [`AppModule`], in deterministic (sorted-by-name) order.
///
/// `project_root` is the directory against which `use "..."` paths inside
/// each `.fl` file are resolved. Pass the project root (the directory that
/// contains `flint.toml`); the server then finds `services/`, `repositories/`
/// etc. relative to it.
pub fn load_app_dir(
    routes_dir: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
) -> Result<Vec<AppModule>, LoadError> {
    let dir = routes_dir.as_ref();
    let root = project_root.as_ref();

    let entries = fs::read_dir(dir).map_err(|source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    })?;

    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| LoadError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "fl") {
            paths.push(path);
        }
    }
    paths.sort();

    let mut modules = Vec::with_capacity(paths.len());
    for path in paths {
        let source = fs::read_to_string(&path).map_err(|source| LoadError::Io {
            path: path.clone(),
            source,
        })?;
        let expanded =
            crate::lang::preprocessor::expand(&source, root).map_err(|e| LoadError::Include {
                path: path.clone(),
                message: e.to_string(),
            })?;
        let app =
            crate::lang::compile_app_source(&expanded).map_err(|source| LoadError::Compile {
                path: path.clone(),
                source,
            })?;
        modules.push(AppModule {
            program: Arc::new(app.program),
            routes: app.routes,
            source_path: path,
        });
    }

    Ok(modules)
}

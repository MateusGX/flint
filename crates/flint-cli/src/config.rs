//! Reads and validates `flint.toml` project manifests.

use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: Project,
    #[serde(default)]
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Server {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_routes")]
    pub routes: String,
    #[serde(default = "default_pages")]
    pub pages: String,
    #[serde(default = "default_services")]
    pub services: String,
    #[serde(default = "default_repositories")]
    pub repositories: String,
    #[serde(default = "default_components")]
    pub components: String,
    // services, repositories, and components are resolved via `use`/`@use`
    // directives in .fl and .flint.ui files; these fields document the
    // convention and are available for tooling.
    /// Verbosity of the built-in request logger.
    /// Values: `"off"` | `"error"` | `"warn"` | `"info"` (default) | `"debug"`
    #[serde(default = "default_log")]
    pub log: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            routes: default_routes(),
            pages: default_pages(),
            services: default_services(),
            repositories: default_repositories(),
            components: default_components(),
            log: default_log(),
        }
    }
}

fn default_version() -> String {
    "0.1.0".into()
}
fn default_host() -> String {
    "127.0.0.1".into()
}
fn default_port() -> u16 {
    3000
}
fn default_routes() -> String {
    "routes".into()
}
fn default_pages() -> String {
    "pages".into()
}
fn default_services() -> String {
    "services".into()
}
fn default_repositories() -> String {
    "repositories".into()
}
fn default_components() -> String {
    "components".into()
}
fn default_log() -> String {
    "info".into()
}

/// Reads and parses an `flint.toml` file.
pub fn load(path: &Path) -> Result<Config, String> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        format!(
            "could not read '{}': {e}\n\nRun `flint new <name>` to create a new project, or make sure\nyou're in a directory that contains an `flint.toml`.",
            path.display()
        )
    })?;

    let config = toml::from_str::<Config>(&content)
        .map_err(|e| format!("'{}' is not valid: {e}", path.display()))?;
    validate_project_name(&config.project.name)?;
    Ok(config)
}

pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("project.name must not be empty".to_string());
    }
    let mut chars = name.chars();
    let starts_valid = chars.next().is_some_and(|ch| ch.is_ascii_alphanumeric());
    let rest_valid = chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_');
    if starts_valid && rest_valid {
        Ok(())
    } else {
        Err(format!(
            "project name '{name}' is invalid; use ASCII letters, numbers, '-' or '_', starting with a letter or number"
        ))
    }
}

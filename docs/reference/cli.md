# CLI and Manifest

The `Flint` CLI lives in the `flint-cli` crate. It creates projects, serves
them in development, and builds standalone release binaries.

## Commands

| Command | Description |
|---|---|
| `flint new <name>` | Create a project from the minimal template. |
| `flint new <name> --template minimal` | Same as the default template. |
| `flint new <name> --template tasks` | Create a Tasks API example. |
| `flint serve [dir]` | Start the development server for a project. |
| `flint run [dir]` | Alias for `flint serve [dir]`. |
| `flint build [dir]` | Build a standalone release binary into `dist/`. |
| `flint version` | Print the CLI version. |
| `flint --version` | Print the CLI version. |
| `flint -V` | Print the CLI version. |

`[dir]` defaults to the current directory.

## Project Manifest

Every project needs a `flint.toml`:

```toml
[project]
name    = "my-app"
version = "0.1.0"

[server]
host         = "127.0.0.1"
port         = 3000
routes       = "routes"
pages        = "pages"
services     = "services"
repositories = "repositories"
```

## Defaults

If optional fields are missing, the CLI uses:

| Field | Default |
|---|---|
| `project.version` | `"0.1.0"` |
| `server.host` | `"127.0.0.1"` |
| `server.port` | `3000` |
| `server.routes` | `"routes"` |
| `server.pages` | `"pages"` |
| `server.services` | `"services"` |
| `server.repositories` | `"repositories"` |

`services` and `repositories` document the project convention. They are not
loaded automatically; include files from them with `use`.

## Minimal Template

```sh
flint new my-app
```

Creates:

```txt
my-app/
├── flint.toml
├── pages/
│   └── index.flint.ui
└── routes/
    └── hello.fl
```

The route serves `GET /hello`. The page serves `GET /`.

## Tasks Template

```sh
flint new my-app --template tasks
```

Creates:

```txt
my-app/
├── flint.toml
├── pages/
│   └── index.flint.ui
├── routes/
│   ├── hello.fl
│   └── tasks.fl
├── services/
│   └── tasks.fl
└── repositories/
    └── tasks.fl
```

Routes:

| Method | Path |
|---|---|
| `GET` | `/` |
| `GET` | `/hello` |
| `GET` | `/tasks` |
| `GET` | `/tasks/:id` |
| `POST` | `/tasks` |

## Serve

```sh
flint serve
flint serve path/to/project
```

The command:

1. reads `flint.toml`
2. loads route modules from `server.routes`
3. loads page files from `server.pages`
4. binds to `server.host:server.port`
5. serves with request tracing enabled

## Build

```sh
flint build
```

The command generates a temporary `.flint-build/` Rust project, embeds route
source and generated page source, runs `cargo build --release`, then copies the
binary to `dist/<project-name>`.

Before invoking Cargo, `flint build` compiles the embedded route/page sources
and validates the route table. This catches invalid Flint source and duplicate
or conflicting routes during the build command instead of leaving them for the
generated binary to find at startup.

When run from the source workspace, the generated project depends on the local
`flint` crate path. Set `FLINT_LIB_PATH=/path/to/flint/crate` to override that
path. If no local crate path is available, the generated project falls back to
the CLI package version.

The generated binary uses `FLINT_ADDR` to choose its listen address at runtime:

```sh
FLINT_ADDR=0.0.0.0:8080 ./dist/my-app
```

If `FLINT_ADDR` is missing, the generated binary listens on `0.0.0.0:3000`.
`ASMB_ADDR` is still accepted as a compatibility fallback.
If the address is invalid or cannot be bound, the generated binary prints the
error and exits with status `1`.

## Exit Behavior

The CLI exits with an error when:

- the project directory has no `flint.toml`
- the manifest is invalid TOML
- route or page loading fails
- the configured bind address is invalid
- the HTTP server fails to start
- `flint build` cannot generate, compile, or copy the binary

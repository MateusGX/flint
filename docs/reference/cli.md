# CLI and Manifest

The `Flint` CLI lives in the `flint-cli` crate. It creates projects, serves
them in development, builds portable bytecode artifacts, and exports static
HTML sites.

## Commands

| Command | Description |
|---|---|
| `flint new <name>` | Create a project from the minimal template. |
| `flint new <name> --template minimal` | Same as the default template. |
| `flint new <name> --template tasks` | Create a Tasks API example. |
| `flint new <name> --template static` | Create a UI-only static site project. |
| `flint serve [dir]` | Start the development server with hot reload (source projects only). |
| `flint run <file.flintbc>` | Serve a compiled bytecode artifact (no hot reload). |
| `flint build [dir]` | Compile portable bytecode into `dist/`. |
| `flint build --static [dir]` | Export `pages/**/*.flint.ui` to static HTML in `dist/`. |
| `flint update` | Update the CLI to the latest release. |
| `flint upgrade` | Alias for `flint update`. |
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
components   = "components"
log          = "info"
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
| `server.components` | `"components"` |
| `server.log` | `"info"` |

`services`, `repositories`, and `components` document the project convention.
They are not loaded automatically; include files from them with `use` (in `.fl`
files) or `@use` (in `.flint.ui` pages).

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
├── components/
│   └── navbar.fl
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
├── components/
│   └── navbar.fl
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

## Static Template

```sh
flint new my-site --template static
```

Creates a UI-only project:

```txt
my-site/
├── flint.toml
├── components/
│   └── navbar.fl
└── pages/
    ├── index.flint.ui
    └── about.flint.ui
```

Export it with:

```sh
flint build --static
```

This writes upload-ready HTML such as `dist/index.html` and
`dist/about/index.html`.

## Serve

```sh
flint serve
flint serve path/to/project
```

`flint serve` is the development server. The command:

1. reads `flint.toml`
2. loads route modules from `server.routes`
3. loads UI page files from `server.pages`
4. binds to `server.host:server.port`
5. starts the built-in request logger at the configured `server.log` level
6. watches source files for changes and reloads automatically

**Hot reload** — whenever a `.fl`, `.flint.ui`, or `flint.toml` file changes
under `routes/`, `pages/`, `services/`, `repositories/`, or `components/`,
the server recompiles and restarts on the same port. If a compile error occurs,
the error is printed and the server waits for the next change before retrying.

`flint serve` only accepts source projects. Passing a `.flintbc` file is an
error; use `flint run` instead.

## Run

```sh
flint run dist/my-app.flintbc
FLINT_ADDR=0.0.0.0:8080 flint run dist/my-app.flintbc
```

`flint run` serves a compiled bytecode artifact with no hot reload and no
`flint.toml` required. It is intended for production and CI use.

`FLINT_ADDR` sets the listen address. If missing, the runtime listens on
`0.0.0.0:3000`. `ASMB_ADDR` is accepted as a compatibility fallback.

`flint run` only accepts `.flintbc` files. Passing a source directory is an
error; use `flint serve` instead.

## Build

```sh
flint build
```

The command compiles route source and generated UI pages into a portable
bytecode artifact at `dist/<project-name>.flintbc`. It does not invoke Cargo
and does not require Rust to be installed on the project machine.

During the build, Flint validates the route table and writes VM-ready bytecode:
instructions, string pools, initial memory, and resolved route handler
addresses. Source text and handler names are not stored in the artifact, and
the payload is lightly obfuscated to avoid casual string/bytecode inspection.

Run the artifact with:

```sh
flint run dist/my-app.flintbc
```

See [Run](#run) for address configuration and environment variables.

## Static Export

```sh
flint build --static
```

This compiles UI pages from `server.pages` and writes HTML files under
`dist/`. Route paths become directory-index files:

| Page route | Output |
|---|---|
| `/` | `dist/index.html` |
| `/about` | `dist/about/index.html` |
| `/docs/start` | `dist/docs/start/index.html` |

Static export supports `GET` UI pages with concrete paths. Dynamic routes such
as `/users/:id`, non-`GET` page routes, and pages that call request-dependent
`http.*` natives are rejected because they need a live request.

Flint UI styles and client helpers are written once as shared assets:

```txt
dist/flint.css
dist/flint.js
```

Each generated HTML file links to those assets with the correct relative path.

## Logging

The built-in request logger is configured via `server.log` in `flint.toml`:

```toml
[server]
log = "info"   # off | error | warn | info | debug
```

| Level | What is logged |
|---|---|
| `off` | Nothing |
| `error` | VM runtime errors (HTTP 500 responses) |
| `warn` | Errors and slow requests (> 1 s) |
| `info` | All requests — one line each with method, path, status, and latency *(default)* |
| `debug` | All requests, plus path params, query string, body size, and handler address |

Example output at `info` level:

```
  →  GET /tasks     200  1.2ms
  →  GET /tasks/42  404  0.8ms
  →  POST /tasks    201  2.1ms
```

Example output at `debug` level:

```
  →  GET /tasks/42  200  1.2ms
       params   id=42
       query    page=1
       body     0 bytes
       handler  0x2a
```

When running a bytecode artifact with `flint run`, there is no `flint.toml`,
so the log level is read from the `FLINT_LOG` environment variable.
Unrecognised values and missing variables fall back to `info`.

```sh
FLINT_LOG=debug flint run dist/my-app.flintbc
```

## Update

```sh
flint update
```

Fetches the latest release version from GitHub, compares it against the
installed version, and re-runs the install script if an update is available:

```
  current  0.2.0
  latest   0.3.0

  Updating to 0.3.0...
```

If the installed version is already the latest:

```
  current  0.3.0
  latest   0.3.0

  Already up to date.
```

`flint upgrade` is an alias for `flint update`. The command requires `curl`
and a shell — the same prerequisites as the original install script.

To install a specific version instead of the latest, re-run the install
script with `FLINT_VERSION`:

```sh
FLINT_VERSION=v0.2.0 curl -fsSL https://flint.devlayer.app/install.sh | sh
```

## Exit Behavior

The CLI exits with an error when:

- the project directory has no `flint.toml`
- the manifest is invalid TOML
- route or page loading fails
- the configured bind address is invalid
- the HTTP server fails to start
- `flint build` cannot validate or write the bytecode artifact
- `flint build --static` cannot render or write the static site

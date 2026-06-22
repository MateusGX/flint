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
| `flint serve [dir]` | Start the development server for a project. |
| `flint serve <file.flintbc>` | Serve a compiled bytecode artifact. |
| `flint run [dir]` | Alias for `flint serve [dir]`. |
| `flint run <file.flintbc>` | Alias for serving a bytecode artifact. |
| `flint build [dir]` | Compile portable bytecode into `dist/`. |
| `flint build --static [dir]` | Export `app/**/*.flint.ui` to static HTML in `dist/`. |
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
routes       = "api"
pages        = "app"
services     = "services"
repositories = "repositories"
components   = "components"
```

## Defaults

If optional fields are missing, the CLI uses:

| Field | Default |
|---|---|
| `project.version` | `"0.1.0"` |
| `server.host` | `"127.0.0.1"` |
| `server.port` | `3000` |
| `server.routes` | `"api"` |
| `server.pages` | `"app"` |
| `server.services` | `"services"` |
| `server.repositories` | `"repositories"` |
| `server.components` | `"components"` |

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
├── app/
│   └── index.flint.ui
└── api/
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
├── app/
│   └── index.flint.ui
├── api/
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
└── app/
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
flint serve dist/my-app.flintbc
```

The command:

1. reads `flint.toml`
2. loads route modules from `server.routes`
3. loads UI page files from `server.pages`
4. binds to `server.host:server.port`
5. serves with request tracing enabled

When given a `.flintbc` file, `serve` skips source loading and executes the
compiled bytecode directly.

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

Run the artifact with any compatible Flint CLI:

```sh
flint run dist/my-app.flintbc
FLINT_ADDR=0.0.0.0:8080 flint run dist/my-app.flintbc
```

When running bytecode, `FLINT_ADDR` chooses the listen address. If it is
missing, Flint listens on `0.0.0.0:3000`.
`ASMB_ADDR` is still accepted as a compatibility fallback.
If the address is invalid or cannot be bound, the runtime prints the
error and exits with status `1`.

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

## Exit Behavior

The CLI exits with an error when:

- the project directory has no `flint.toml`
- the manifest is invalid TOML
- route or page loading fails
- the configured bind address is invalid
- the HTTP server fails to start
- `flint build` cannot validate or write the bytecode artifact
- `flint build --static` cannot render or write the static site

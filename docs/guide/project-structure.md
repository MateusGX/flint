# Project Structure

Small apps can keep everything in one route file. Larger apps are easier to
read when you separate UI pages, HTTP controllers, services, and data access.

## Default Layout

```txt
my-app/
├── flint.toml
├── app/
│   └── index.flint.ui
├── api/
│   ├── hello.fl
│   └── tasks.fl
├── components/
│   └── navbar.fl
├── services/
│   └── tasks.fl
└── repositories/
    └── tasks.fl
```

## What Each Directory Does

| Directory | Loaded automatically | Purpose |
|---|---|---|
| `app/` | Yes, recursively for `*.flint.ui` | Server-rendered UI pages. |
| `api/` | Yes, direct `*.fl` children only when present | HTTP route handlers and controllers. |
| `components/` | No | Reusable UI fragments included with `@use` in `.flint.ui` pages. |
| `services/` | No | Business rules included with `use`. |
| `repositories/` | No | Data access included with `use`. |

The common API flow is:

```txt
route/controller -> service -> repository
```

Pages can use the same services, and pull in shared UI fragments from components:

```txt
page -> component (UI fragment)
page -> service -> repository
```

UI-only projects can omit `api/` entirely and use `flint build --static` to
export upload-ready HTML from `app/**/*.flint.ui`.

## Manifest

`flint.toml` defines the project and directory conventions:

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

Only `routes` and `pages` control automatic loading. `services`,
`repositories`, and `components` are documented conventions available to
tooling and humans; include files from them with `use` or `@use`.

## Route Modules

Every `.fl` file directly inside `api/` is compiled as an independent
module:

```txt
api/tasks.fl
api/hello.fl
```

Each module gets its own bytecode program, label namespace, function table, and
string pool. Routes from every module are registered on one HTTP router.

Nested route directories are not loaded automatically.

## Shared Code With `use`

Use `use` to inline shared code before compilation:

```txt
use "services/tasks.fl"
```

`api/tasks.fl`:

```txt
use "services/tasks.fl"

section .route
    GET "/tasks" -> tasks_controller_list

section .text
tasks_controller_list:
    call tasks_service_list
    ncall http.json, r0
    ret
```

`services/tasks.fl`:

```txt
use "repositories/tasks.fl"

section .text
tasks_service_list:
    call tasks_repository_all
    ret
```

`repositories/tasks.fl`:

```txt
section .text
tasks_repository_all:
    mov r0, "[{\"id\":\"1\",\"title\":\"Buy milk\"}]"
    ncallr r0, json.parse, r0
    ret
```

Includes are resolved from the project root, not from the current file. Included
`.fl` files must use sections too; put shared functions under `section .text`.

## UI Pages

Pages are loaded recursively from `app/`:

```txt
app/index.flint.ui       -> /
app/tasks/index.flint.ui -> /tasks
app/tasks/[id].flint.ui  -> /tasks/:id
app/admin/index.flint.ui -> /admin
```

Pages can include services:

```txt
@use "services/tasks.fl"

section .route
    GET "/tasks"

section .text
call tasks_service_list
ncallr r1, json.stringify, r0

section .render
    window "Tasks"
        code r1
    end
```

Use `.flint.ui` pages when you want styled controls and default layout. Use
route files when the output is mostly JSON, text, redirects, or
request/response control flow.

## Naming Rules

After includes are expanded, all labels and functions in a module share one
flat namespace.

Prefer names that include the feature and layer:

```txt
tasks_controller_get
tasks_service_get
tasks_repository_find_by_id
tasks_get_found
tasks_get_done
```

Avoid generic names:

```txt
found:
done:
error:
```

They are likely to collide after `use` expansion.

## Build Output

`flint build` compiles all route source and generated page source into portable
bytecode at:

```txt
dist/<project-name>.flintbc
```

Run it with `flint run dist/<project-name>.flintbc`.

For static sites, `flint build --static` writes directory-index HTML files:

```txt
dist/index.html
dist/about/index.html
dist/flint.css
dist/flint.js
```

See [CLI and Manifest](/reference/cli) for full command details.

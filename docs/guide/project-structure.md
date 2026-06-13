# Project Structure

Small apps can keep everything in one route file. Larger apps are easier to
read when you separate HTML pages, HTTP controllers, services, and data access.

## Default Layout

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

## What Each Directory Does

| Directory | Loaded automatically | Purpose |
|---|---|---|
| `pages/` | Yes, recursively for `*.flint.html` and `*.flint.ui` | Server-rendered pages. |
| `routes/` | Yes, direct `*.fl` children only | HTTP route handlers and controllers. |
| `services/` | No | Business rules included with `use`. |
| `repositories/` | No | Data access included with `use`. |

The common API flow is:

```txt
route/controller -> service -> repository
```

Pages can use the same services:

```txt
page -> service -> repository
```

## Manifest

`flint.toml` defines the project and directory conventions:

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

Only `routes` and `pages` control automatic loading. `services` and
`repositories` are documented conventions available to tooling and humans.

## Route Modules

Every `.fl` file directly inside `routes/` is compiled as an independent
module:

```txt
routes/tasks.fl
routes/hello.fl
```

Each module gets its own bytecode program, label namespace, function table, and
string pool. Routes from every module are registered on one HTTP router.

Nested route directories are not loaded automatically.

## Shared Code With `use`

Use `use` to inline shared code before compilation:

```txt
use "services/tasks.fl"
```

`routes/tasks.fl`:

```txt
use "services/tasks.fl"

tasks_controller_list:
    call tasks_service_list
    ncall http.json, r0
    ret

route GET "/tasks" -> tasks_controller_list
```

`services/tasks.fl`:

```txt
use "repositories/tasks.fl"

tasks_service_list:
    call tasks_repository_all
    ret
```

`repositories/tasks.fl`:

```txt
tasks_repository_all:
    mov r0, "[{\"id\":\"1\",\"title\":\"Buy milk\"}]"
    ncallr r0, json.parse, r0
    ret
```

Includes are resolved from the project root, not from the current file.

## Visual Pages

Pages are loaded recursively from `pages/`:

```txt
pages/index.flint.html       -> /
pages/tasks/index.flint.html -> /tasks
pages/tasks/[id].flint.html  -> /tasks/:id
pages/admin/index.flint.ui   -> /admin
```

Pages can include services:

```html
@page "/tasks"
@use "services/tasks.fl"
<%
call tasks_service_list
ncallr r1, json.stringify, r0
%>
<pre><%= r1 %></pre>
```

Use `.flint.html` pages when the output is handwritten HTML. Use `.flint.ui`
pages when you want styled controls and default layout. Use route files when
the output is mostly JSON, text, redirects, or request/response control flow.

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

`flint build` creates a generated Rust project under `.flint-build/`, embeds
all route source and generated page source, builds a release binary, and copies
it to:

```txt
dist/<project-name>
```

See [CLI and Manifest](/reference/cli) for full command details.

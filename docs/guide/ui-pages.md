# UI Pages

UI pages live under `pages/`, end with `.flint.ui`, and compile into normal
Flint route modules. They use section blocks:

```txt
section .route
    GET "/"

section .render
    window "Dashboard"
        text "Rendered with Flint UI."
    end
```

UI pages use sections only.

```txt
pages/
├── index.flint.ui
└── users/
    └── [id].flint.ui
```

## Your First UI Page

Create `pages/index.flint.ui`:

```txt
section .route
    GET "/"

section .render
    window "Dashboard"
        text "This page uses Flint UI controls."
        card "Actions"
            btn "Open API", "/hello"
        end
    end
```

Run:

```sh
flint serve
```

Open:

```txt
http://127.0.0.1:3000/
```

The page compiler emits a route handler that returns HTML through `http.html`.

## Sections

UI pages support these sections:

| Section | Purpose |
|---|---|
| `section .route` | Optional method and path for the generated handler. |
| `section .data` | String constants for render commands. |
| `section .bss` | Scratch memory cells emitted into generated Flint source. |
| `section .text` | Raw Flint instructions that run before rendering. |
| `section .render` | UI DSL commands that append styled HTML. |

The first non-comment, non-`@use` line must be a `section` header.

## Routes

Inside `section .route`, write one method and path:

```txt
section .route
    POST "/tasks/new"
```

If the route section has no route line, the path is inferred from the file
path and the method defaults to `GET`:

| File | Inferred route |
|---|---|
| `pages/index.flint.ui` | `GET /` |
| `pages/tasks/index.flint.ui` | `GET /tasks` |
| `pages/tasks/[id].flint.ui` | `GET /tasks/:id` |

## Includes

Use `@use` before the first section to include shared `.fl` code:

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

`@use` paths are resolved from the project root, just like `use` in `.fl`
files. Included files must use sections.

## Render Commands

`section .render` is a small control DSL. String literals, identifiers from
`section .data`, and registers can be passed as arguments. Blocks close with
`end`.

Common commands:

| Command | Emits |
|---|---|
| `window "Title" ... end` | Full page shell and frame. |
| `card "Title" ... end` | Raised content panel. |
| `block "Title" ... end` | Unframed content section; optional second arg adds a subtitle. |
| `row ... end` / `col ... end` | Side-by-side column layout (`display:table`). |
| `text "Value"` | Paragraph text. |
| `title "Value"` | Heading. |
| `field "Label", r1` | Label/value row. |
| `btn "Label", "/href"` | Link styled as a button. |
| `form "POST", "/path" ... end` | HTML form. |
| `input "Label", "name"` | Text input. |
| `submit "Save"` | Submit button. |

The showcase under `examples/ui-showcase/pages/` demonstrates the larger set:
navbar, breadcrumb, tables, tabs, dialogs, accordions, trees, forms, status
badges, meters, progress bars, and layout primitives.

## Data and Dynamic Values

Use `section .data` for reusable strings:

```txt
section .route
    GET "/hello"

section .data
    greeting db "Hello from data"

section .render
    window "Home"
        text greeting
    end
```

Use `section .text` for request data and other logic:

```txt
section .route
    GET "/hello"

section .text
    mov r0, "name"
    ncallr r1, http.query, r0

section .render
    window "Hello"
        field "Name", r1
    end
```

Generated page handlers reserve `r14` for the HTML accumulator and use `r15`,
`r13`, `r12`, `r11`, and `r10` as scratch registers while compiling render
arguments.

## Components

Reusable UI fragments are plain Flint functions that receive `r14` (the HTML
accumulator), append markup to it, and return. Because `call` and Flint
mnemonics pass through `section .render` unchanged, any label in scope can be
called directly inside the render DSL.

### Shared components with `@use`

Put the component in a `.fl` file under `components/`:

```txt
; components/site_nav.fl
section .text
site_nav:
    ncallr r14, ui.navbar, r14
    mov r15, "Home"
    mov r13, "/"
    ncallr r14, ui.nav_item, r14, r15, r13
    mov r15, "Tasks"
    mov r13, "/tasks"
    ncallr r14, ui.nav_item, r14, r15, r13
    ncallr r14, ui.navbar_end, r14
    ret
```

Include it with `@use` and call it inside `section .render`:

```txt
@use "components/site_nav.fl"

section .route
    GET "/tasks"

section .render
    window "Tasks"
        call site_nav
        card "All tasks"
            text "..."
        end
    end
```

Any number of pages can `@use` the same file. The function name must be unique
across the project (see [label namespace rules](/reference/language#labels)).

### Inline helpers

For fragments used only within one page, define a label in `section .text` and
call it from `section .render`:

```txt
section .route
    GET "/dashboard"

section .text
render_header:
    mov r15, "Dashboard"
    ncallr r14, ui.title, r14, r15
    mov r15, "Welcome back."
    ncallr r14, ui.text, r14, r15
    ret

section .render
    window "Dashboard"
        call render_header
        card "Stats"
            text "..."
        end
    end
```

### Component contract

- Read `r14` as the incoming HTML accumulator and write the updated value back
  to `r14` before `ret`.
- Use `r0`–`r9` for internal work; those registers are free inside the component.
- Do not rely on `r15`, `r13`, `r12`, `r11`, or `r10` surviving across `call`
  — the render compiler uses them as scratch registers for its own arguments.

## Forms

Forms generate regular HTML forms with the default Flint style:

```txt
section .route
    GET "/tasks/new"

section .render
    window "New task"
        form "POST", "/tasks"
            input "Title", "title"
            submit "Create"
        end
    end
```

Read submitted values from the target route with `http.form`.

## Generated Shape

This page:

```txt
section .route
    GET "/"

section .render
    window "Home"
        text "Welcome"
    end
```

generates a route handler shaped like this:

```txt
section .route
    GET "/" -> __page_index

section .text
__page_index:
    mov r14, ""
    mov r15, "Home"
    ncallr r14, ui.window, r14, r15
    mov r15, "Welcome"
    ncallr r14, ui.text, r14, r15
    ncallr r14, ui.window_end, r14
    ncall http.html, r14
    ret
```

The VM does not have a special UI mode. `ui.*` functions are ordinary stdlib
natives that run when the generated handler executes.

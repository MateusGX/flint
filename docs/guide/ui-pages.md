# UI Pages

UI pages live under `pages/`, end with `.flint.ui`, and render with Flint's
built-in default style. They compile through the exact same pipeline as
[Visual Pages](/guide/pages) — there is no separate control language. By
convention, a UI page's body is just `<% ... %>` code that calls `ui.*`
natives to append styled HTML fragments to the `r14` accumulator.

```txt
pages/
├── index.flint.ui
└── users/
    └── [id].flint.ui
```

## Your First UI Page

Create `pages/index.flint.ui`:

```txt
@page "/"
<%
mov r15, "Dashboard"
ncallr r14, ui.window, r14, r15
mov r15, "This page uses Flint UI natives."
ncallr r14, ui.text, r14, r15
mov r15, "Actions"
ncallr r14, ui.card, r14, r15
mov r15, "Open API"
mov r1, "/hello"
ncallr r14, ui.button, r14, r15, r1
ncallr r14, ui.card_end, r14
ncallr r14, ui.window_end, r14
%>
```

Run:

```sh
flint serve
```

Open:

```txt
http://127.0.0.1:3000/
```

The page is compiled into a normal route handler that returns HTML through
`http.html`.

## Directives

UI pages use the same preamble directives as HTML pages:

| Directive | Meaning |
|---|---|
| `@page` | Serve the page with `GET` and infer the path from the file name. |
| `@page "/path"` | Serve the page with `GET /path`. |
| `@page POST "/path"` | Serve the page with an explicit method and path. |
| `@route METHOD "/path"` | Equivalent explicit route form. |
| `@use "path.fl"` | Include shared Flint code before the generated handler. |

## `ui.*` Natives

Every `ui.*` native takes the current HTML accumulator as its first argument
and returns the new accumulator value — the same shape as `string.concat`,
but appending a styled fragment instead of a literal. `ncall`/`ncallr`
arguments must be registers, so string literals go through a scratch register
first:

```txt
mov r15, "Profile"
ncallr r14, ui.card, r14, r15
```

| Native | Call | Appends |
|---|---|---|
| `ui.window` | `ncallr dst, ui.window, html, title` | Document shell, default stylesheet, browser tab title (`<title>`), and a styled page frame with `title`. |
| `ui.window_end` | `ncallr dst, ui.window_end, html` | Closes a frame opened with `ui.window`. |
| `ui.card` | `ncallr dst, ui.card, html, title` | Bordered content panel with `title`. |
| `ui.card_end` | `ncallr dst, ui.card_end, html` | Closes a panel opened with `ui.card`. |
| `ui.section` | `ncallr dst, ui.section, html, title` | Unframed content group with `title`. |
| `ui.section_end` | `ncallr dst, ui.section_end, html` | Closes a group opened with `ui.section`. |
| `ui.row` | `ncallr dst, ui.row, html` | Horizontal responsive layout. |
| `ui.row_end` | `ncallr dst, ui.row_end, html` | Closes a layout opened with `ui.row`. |
| `ui.column` | `ncallr dst, ui.column, html` | Vertical layout. |
| `ui.column_end` | `ncallr dst, ui.column_end, html` | Closes a layout opened with `ui.column`. |
| `ui.title` | `ncallr dst, ui.title, html, value` | Heading. |
| `ui.text` | `ncallr dst, ui.text, html, value` | Paragraph text. |
| `ui.field` | `ncallr dst, ui.field, html, label, value` | Label/value display row. |
| `ui.button` | `ncallr dst, ui.button, html, label, href` | Link styled as a button. |
| `ui.form` | `ncallr dst, ui.form, html, method, action` | HTML form opener. |
| `ui.form_end` | `ncallr dst, ui.form_end, html` | Closes a form opened with `ui.form`. |
| `ui.input` | `ncallr dst, ui.input, html, label, name` | Labeled text input inside a form. |
| `ui.submit` | `ncallr dst, ui.submit, html, label` | Submit button inside a form. |

`title`, `value`, `label`, `href`, `method`, `action`, and `name` must all be
`str` registers. `ui.title`, `ui.text`, `ui.field`, and the label arguments of
`ui.button`/`ui.input`/`ui.submit` are HTML-escaped; `ui.button`'s `href`,
`ui.form`'s `method`/`action`, and `ui.input`'s `name` are attribute-escaped.

## Dynamic Values

Use normal Flint code to prepare values before passing them to `ui.*`
natives. Values must already be `str` — convert with `string.from` first if
you have an `int` or `float`:

```txt
@page "/hello"
<%
mov r0, "name"
ncallr r1, http.query, r0
mov r15, "Hello"
ncallr r14, ui.window, r14, r15
mov r15, "Name"
ncallr r14, ui.field, r14, r15, r1
ncallr r14, ui.window_end, r14
%>
```

`ui.field` HTML-escapes both `label` and `value` before appending
`<dl class="flint-field"><dt>Name</dt><dd>...</dd></dl>` to `r14`.

## Forms

Forms generate regular HTML forms with the default Flint style:

```txt
@page "/tasks/new"
<%
mov r15, "New task"
ncallr r14, ui.window, r14, r15
mov r15, "POST"
mov r1, "/tasks"
ncallr r14, ui.form, r14, r15, r1
mov r15, "Title"
mov r1, "title"
ncallr r14, ui.input, r14, r15, r1
mov r15, "Create"
ncallr r14, ui.submit, r14, r15
ncallr r14, ui.form_end, r14
ncallr r14, ui.window_end, r14
%>
```

Read submitted values from the target route with `http.form`.

## Generated Shape

This page:

```txt
@page "/"
<%
mov r15, "Home"
ncallr r14, ui.window, r14, r15
mov r15, "Welcome"
ncallr r14, ui.text, r14, r15
ncallr r14, ui.window_end, r14
%>
```

generates a route handler shaped like this:

```txt
__page_index:
    mov r14, ""
    mov r15, "Home"
    ncallr r14, ui.window, r14, r15
    mov r15, "Welcome"
    ncallr r14, ui.text, r14, r15
    ncallr r14, ui.window_end, r14
    ncall http.html, r14
    ret

route GET "/" -> __page_index
```

The VM does not have a special UI mode. `.flint.ui` pages compile through the
same path as `.flint.html` pages — `ui.*` natives are ordinary stdlib natives
that run when the handler executes.

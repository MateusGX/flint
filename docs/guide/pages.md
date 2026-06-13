# Visual Pages

Visual pages are HTML-first templates. They live under `pages/` and end with
`.flint.html`. If you want styled controls without writing HTML, use
[UI Pages](/guide/ui-pages) with `.flint.ui`.

```txt
pages/
├── index.flint.html
├── about.flint.html
└── users/
    └── [id].flint.html
```

When the server starts, each page is compiled into normal Flint source. That
generated source declares a route and a handler that builds one HTML string,
then sends it with `http.html`.

## Your First Page

Create `pages/index.flint.html`:

```html
@page "/"
<!doctype html>
<html>
<head>
    <title>Home</title>
</head>
<body>
    <h1>Hello from Flint</h1>
</body>
</html>
```

Run:

```sh
flint serve
```

Open:

```txt
http://127.0.0.1:3000/
```

## Route Directives

Directives are read only from the top preamble of the file. Once normal HTML
starts, later lines are treated as HTML text.

| Directive | Meaning |
|---|---|
| `@page` | Serve the page with `GET` and infer the path from the file name. |
| `@page "/path"` | Serve the page with `GET /path`. |
| `@page POST "/path"` | Serve the page with an explicit method and path. |
| `@route METHOD "/path"` | Equivalent explicit route form. |
| `@use "path.fl"` | Include shared Flint code before the generated handler. |

Supported methods are the same as route files:

```txt
GET POST PUT PATCH DELETE HEAD OPTIONS
```

## File-Based Routes

If a page does not declare a path, the route is inferred from the file path.

| File | Route |
|---|---|
| `pages/index.flint.html` | `/` |
| `pages/about.flint.html` | `/about` |
| `pages/blog/index.flint.html` | `/blog` |
| `pages/users/[id].flint.html` | `/users/:id` |

Path parameters from `[id]` are read with `http.param`:

```html
@page
<%
mov r0, "id"
ncallr r1, http.param, r0
%>
<h1>User <%= r1 %></h1>
```

## Code Blocks

Use `<% ... %>` for normal Flint instructions:

```html
@page "/hello"
<%
mov r0, "name"
ncallr r1, http.query, r0
ncallr r2, string.len, r1
cmp r2, 0
jne has_name
mov r1, "friend"
has_name:
%>
<h1>Hello <%= r1 %></h1>
```

Code block lines are inserted into the generated handler. Labels still share
the module namespace, so keep labels specific (or use [local
labels](/reference/language#labels)) if the page uses `@use`.

## Output Expressions

Use `<%= ... %>` to append a value to the HTML response:

```html
<p>Generated at <%= r0 %></p>
<p>Status: <%= "ok" %></p>
<p>Count: <%= 42 %></p>
```

The expression is copied into scratch register `r15`, converted with
`string.from`, escaped with `string.escape_html`, and concatenated into the
HTML accumulator.

Accepted expressions are values that can be used as the source of `mov`:
registers, integers, floats, and strings.

## Reserved Registers

Generated page handlers reserve two registers:

| Register | Used for |
|---|---|
| `r14` | HTML accumulator. |
| `r15` | Scratch value for HTML chunks and expressions. |

Avoid relying on `r14` and `r15` inside page code blocks unless you deliberately
save and restore them.

## Includes

Use `@use` to include services or helper functions:

```html
@page "/dashboard"
@use "services/dashboard.fl"
<%
call dashboard_summary
%>
<h1><%= r0 %></h1>
```

The include path is resolved from the project root, exactly like `use` in
`.fl` files.

## Generated Shape

This page:

```html
@page "/hello"
<h1>Hello</h1>
```

becomes source shaped like this:

```txt
__page_index:
    mov r14, ""
    mov r15, "<h1>Hello</h1>\n"
    ncallr r14, string.concat, r14, r15
    ncall http.html, r14
    ret

route GET "/hello" -> __page_index
```

The VM and HTTP runtime do not have a separate page mode. Pages are just a
friendlier way to generate route handlers.

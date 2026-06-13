# HTTP Runtime

The HTTP runtime turns compiled Flint modules into an `axum` router. It is used
by `flint serve`, by standalone binaries produced by `flint build`, and by
tests through `flint::http::router`.

`flint::http::router` and `flint::http::try_router` return a `RouterError`
instead of panicking when routes are invalid or conflicting.

## Loading Routes

`flint serve` reads `flint.toml`, then loads:

| Config field | Loaded from | Recursive |
|---|---|---|
| `server.routes` | `.fl` files directly inside the route directory | No |
| `server.pages` | `.flint.html` and `.flint.ui` files inside the pages directory | Yes |

Each route file is compiled independently into an app module. Page files are
first converted to generated Flint source, then compiled the same way.

## Route Matching

Routes are declared in source with:

```txt
route METHOD "/path" -> handler
```

Supported methods:

```txt
GET POST PUT PATCH DELETE HEAD OPTIONS
```

Dynamic path segments use `:name`:

```txt
route GET "/users/:id" -> show_user
```

Read path parameters with `http.param`.

## Request Dispatch

For every matched request, the runtime:

1. Parses path params, query params, headers, and body into an `HttpExchange`.
2. Builds a fresh `NativeRegistry`.
3. Registers all `stdlib` natives.
4. Registers request-scoped `http.*` natives that close over the exchange.
5. Creates a fresh VM.
6. Calls the matched handler address.
7. Converts the assembled response back to an HTTP response.

There is no VM state shared between requests.

## Request Data

The request body is decoded as UTF-8 lossily. Invalid UTF-8 does not fail the
request, but it may fail later if parsed as JSON.

Request headers are stored by lower-cased name. `http.header` therefore does a
case-insensitive lookup by lowercasing the requested key. Header values are
also decoded as UTF-8 lossily because Flint strings are UTF-8.

Query strings and forms are parsed with URL-encoded form parsing.

Missing `http.param`, `http.query`, `http.header`, `http.cookie`, and
`http.form` values return an empty string.

## Response Defaults

Every response starts as:

| Field | Default |
|---|---|
| Status | `200` |
| Headers | none |
| Body | empty |

Body natives suggest a content type:

| Native | Default `Content-Type` |
|---|---|
| `http.text` | `text/plain; charset=utf-8` |
| `http.html` | `text/html; charset=utf-8` |
| `http.json` | `application/json` |

If the handler already set a `Content-Type` header with `http.set_header`, the
runtime keeps the explicit header and does not apply the default.

## Headers and Cookies

`http.set_header` validates and appends a header. It does not replace previous
headers.

```txt
mov r0, "x-powered-by"
mov r1, "Flint"
ncall http.set_header, r0, r1
```

`http.set_cookie` validates the name and value, then appends a simple
`Set-Cookie` header in the form
`name=value`.

```txt
mov r0, "session"
mov r1, "abc"
ncall http.set_cookie, r0, r1
```

`http.cookie` reads from the incoming `Cookie` header by splitting on `;` and
`=`.

## Redirects

`http.redirect` validates the header value, sets the status to `302`, and
appends a `location` header.

```txt
mov r0, "/login"
ncall http.redirect, r0
ret
```

## Early Abort

`http.abort` stops the handler immediately and sends whatever response has
already been assembled.

```txt
mov r0, 401
ncall http.set_status, r0
mov r0, "unauthorized"
ncall http.text, r0
ncall http.abort
```

Internally, the native returns a sentinel error. The dispatcher recognizes it
and skips the normal `500` error response.

## Runtime Errors

If a handler raises any other VM or native error, the dispatcher logs it and
returns:

- status `500`
- body beginning with `Flint runtime error:`

Examples include invalid JSON in `http.json_body`, wrong native argument types,
unknown native functions, invalid status codes, stack underflow, and arithmetic
errors.

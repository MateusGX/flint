# Troubleshooting

This page lists common errors and where to look first.

## The CLI Is Not Found

If `flint` is not available:

```sh
flint version
# command not found: flint
```

Install it from the repository root:

```sh
cargo install --path crates/flint-cli --bin flint
```

Or run the debug binary directly:

```sh
./target/debug/flint version
```

## `flint.toml` Is Missing

`flint serve` and `flint build` expect a project directory containing
`flint.toml`.

```sh
flint new my-app
cd my-app
flint serve
```

If you pass a directory, pass the project root:

```sh
flint serve path/to/my-app
```

## A Route File Is Not Loaded

Only `.fl` files directly inside the configured `api` directory are loaded.

This is loaded:

```txt
api/tasks.fl
```

This is not loaded automatically:

```txt
api/admin/tasks.fl
```

Move it to `api/` or include it from a loaded route file.

## A Page Is Not Loaded

Pages must end with `.flint.ui` and live under the configured `app` directory
unless `server.pages` points somewhere else.

This is loaded:

```txt
app/index.flint.ui
app/users/[id].flint.ui
app/dashboard.flint.ui
```

This is not a page:

```txt
app/index.html
app/index.fl
```

If `app/` does not exist, the server simply loads no pages.

## A Route Handler Is Missing

This route:

```txt
section .route
    GET "/hello" -> say_hello
```

requires a matching global label in the same compiled module:

```txt
section .text
say_hello:
    ret
```

A local label (`.say_hello:`) cannot be a route handler — only global labels
can.

## An Include Cannot Be Found

`use` and `@use` paths are resolved from the project root:

```txt
use "services/tasks.fl"
```

not from the directory of the current file.

Check that the included file exists relative to `flint.toml`.

## Duplicate Label Errors

After includes are expanded, all global labels share one namespace.

This can fail:

```txt
section .text
one:
done:
    ret

two:
done:
    ret
```

Use specific labels:

```txt
section .text
one_done:
two_done:
```

or local labels scoped to each enclosing label:

```txt
section .text
one:
.done:
    ret

two:
.done:
    ret
```

## A Native Function Says the Type Is Wrong

Native functions expect specific value types. For example, `http.json` expects
a `json` value:

```txt
mov r0, "not json"
ncall http.json, r0
```

Use `http.text` for strings, or build JSON first:

```txt
mov r0, "Ada"
ncallr r0, json.from_str, r0
ncall http.json, r0
```

## `ncallr` Fails

`ncallr` requires a return value:

```txt
ncallr r0, http.text, r1
```

This fails because `http.text` only changes the response. Use `ncall`:

```txt
ncall http.text, r1
```

## JSON Updates Do Not Change the Original

`json.set`, `json.push`, `json.delete`, and `json.merge` return new JSON
values. Store the result:

```txt
ncallr r0, json.set, r0, r1, r2
```

If you write the result to another register, the original stays unchanged.

## JSON Null Checks

`json.get` returns JSON `null` when a key or non-negative array index is
missing. Negative indexes are invalid. Check nulls with `json.type`:

```txt
ncallr r1, json.type, r0
mov r2, "null"
ncallr r1, string.equals, r1, r2
```

`r1` is `1` when the value is null.

## Register Values Change After `call`

All functions share registers. If you need a value after calling another
function, save it:

```txt
push r0
call other_function
pop r0
```

Keep stack operations balanced. `call` and `ret` use the same stack for return
addresses.

## Page Output Looks Wrong

Generated page handlers use:

| Register | Purpose |
|---|---|
| `r14` | HTML accumulator. |
| `r15`, `r13`, `r12`, `r11`, `r10` | Scratch registers for render arguments. |

If `section .text` in a page writes to these registers and expects their
values to survive rendering, the generated render code may overwrite them. Use
lower registers for values consumed later, or save and restore carefully.

## `http.json_body` Returns 500

`http.json_body` parses the whole request body as JSON. Invalid JSON raises a
runtime error, which the HTTP runtime returns as a `500`.

Check the request:

```sh
curl -i -X POST http://127.0.0.1:3000/echo \
  -H 'Content-Type: application/json' \
  -d '{"name":"Ada"}'
```

## `flint build` Fails

`flint build` validates the project sources, compiles them to portable
bytecode, and writes the result to `dist/<project-name>.flintbc`.

Check:

- route and page sources compile under `flint serve`
- route paths are unique and do not conflict by parameter shape
- the project name is valid and can be used as the output file name
- `dist/` is writable

Run the artifact with `flint run dist/<project-name>.flintbc`.

Bytecode runs on `FLINT_ADDR` if set, otherwise
`0.0.0.0:3000`. Invalid or unavailable addresses are printed as startup errors
and exit with status `1`.

## `flint build --static` Fails

Static export renders UI pages to upload-ready HTML under `dist/`.
The shared Flint UI assets are emitted once as `dist/flint.css` and
`dist/flint.js`.

Check:

- pages live under `app/` or the configured `server.pages`
- page routes use `GET`
- page routes are concrete, not dynamic paths such as `/users/:id`
- page code does not call any `http.*` native other than `http.html`
  (all request-dependent natives such as `http.query`, `http.param`,
  `http.form`, `http.json_body`, `http.redirect`, and response-writing
  natives like `http.text` and `http.json` are rejected at export time)
- `dist/` is writable

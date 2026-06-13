# Native Functions

Native functions are Rust functions exposed to Flint bytecode through `ncall`
and `ncallr`.

```txt
ncall debug.print, r0
ncallr r1, json.object
ncallr r2, json.set, r2, r3, r4
```

Arguments must be registers. Load literals into registers before calling a
native.

## Namespaces

| Namespace | Purpose |
|---|---|
| `debug.*` | Print values during development. |
| `string.*` | Text operations and string conversion. |
| `json.*` | Create, inspect, update, parse, and serialize JSON. |
| `math.*` | Numeric helpers and random numbers. |
| `time.*` | Current Unix time. |
| `env.*` | Environment variables. |
| `crypto.*` | UUID generation. |
| `ui.*` | Build Flint's default styled HTML for UI pages. |
| `http.*` | Request and response helpers inside HTTP handlers. |

## Calling Rules

Use `ncall` for side effects:

```txt
ncall http.text, r0
```

Use `ncallr` for return values:

```txt
ncallr r0, json.object
```

If a native receives the wrong type, the runtime error names the native and
the invalid argument position. Standard natives also reject the wrong number
of arguments. If `ncallr` is used with a native that returns no value, the VM
raises a runtime error.

## `debug.*`

| Native | Call | Result |
|---|---|---|
| `debug.print` | `ncall debug.print, r0, r1, ...` | Prints arguments to stdout, space-separated. |

`debug.print` returns no value.

## `string.*`

### Tests

| Native | Call | Returns |
|---|---|---|
| `string.equals` | `ncallr dst, string.equals, a, b` | `1` when strings are equal, else `0`. |
| `string.contains` | `ncallr dst, string.contains, s, sub` | `1` when `s` contains `sub`, else `0`. |
| `string.starts_with` | `ncallr dst, string.starts_with, s, prefix` | `1` when `s` starts with `prefix`, else `0`. |
| `string.ends_with` | `ncallr dst, string.ends_with, s, suffix` | `1` when `s` ends with `suffix`, else `0`. |
| `string.escape_html` | `ncallr dst, string.escape_html, s` | Text escaped for safe HTML insertion. |

### Transform

| Native | Call | Returns |
|---|---|---|
| `string.concat` | `ncallr dst, string.concat, a, b` | `a` followed by `b`. |
| `string.trim` | `ncallr dst, string.trim, s` | `s` without leading and trailing whitespace. |
| `string.to_upper` | `ncallr dst, string.to_upper, s` | Uppercase text. |
| `string.to_lower` | `ncallr dst, string.to_lower, s` | Lowercase text. |
| `string.replace` | `ncallr dst, string.replace, s, from, to` | Text with all matches replaced. |
| `string.slice` | `ncallr dst, string.slice, s, start, end` | Character slice between clamped indices. |

`string.slice` indexes by Unicode scalar values (`chars`), clamps both indices
to the string length, and returns the range between the smaller and larger
index.

### Convert and Split

| Native | Call | Returns |
|---|---|---|
| `string.len` | `ncallr dst, string.len, s` | Character count as `int`. |
| `string.split` | `ncallr dst, string.split, s, sep` | JSON array of string parts. |
| `string.to_int` | `ncallr dst, string.to_int, s` | Parsed integer after trimming whitespace. |
| `string.from_int` | `ncallr dst, string.from_int, n` | Integer as string. |
| `string.from` | `ncallr dst, string.from, value` | Any VM value as string. |

`string.to_int` fails if the string cannot be parsed as an `i64`.

`string.from` uses the runtime display form: strings are emitted as raw text,
JSON is compact JSON, and numbers use their normal textual form.

`string.escape_html` replaces HTML-sensitive characters like `<`, `>`, `&`,
quotes, and apostrophes with entities. Page output expressions use it
automatically after `string.from`.

## `json.*`

JSON is a VM value. There is no JSON literal syntax in `.fl` source.

::: info Copy-on-write
`json.set`, `json.push`, `json.delete`, and `json.merge` return new JSON
documents. Store the returned value if you want to keep the change.
:::

### Create and Parse

| Native | Call | Returns |
|---|---|---|
| `json.object` | `ncallr dst, json.object` | Empty object `{}`. |
| `json.array` | `ncallr dst, json.array` | Empty array `[]`. |
| `json.null` | `ncallr dst, json.null` | JSON `null`. |
| `json.bool` | `ncallr dst, json.bool, n` | JSON `true` when `n != 0`, else `false`. |
| `json.parse` | `ncallr dst, json.parse, s` | Parsed JSON from a string. |

`json.parse` fails on invalid JSON.

### Read

| Native | Call | Returns |
|---|---|---|
| `json.get` | `ncallr dst, json.get, j, key` | Object field, non-negative array item, or JSON `null`. |
| `json.has` | `ncallr dst, json.has, j, key` | `1` if object field exists, else `0`. |
| `json.len` | `ncallr dst, json.len, j` | Length of array, object, or JSON string. |
| `json.type` | `ncallr dst, json.type, j` | JSON type name. |
| `json.keys` | `ncallr dst, json.keys, j` | JSON array of object keys. |

`json.get` accepts string keys for objects and non-negative integer keys for
arrays. Missing values return JSON `null`.

`json.len` fails unless the value is an array, object, or string.

`json.keys` fails unless the value is an object.

`json.type` returns one of:

```txt
null bool number string array object
```

### Update

| Native | Call | Returns |
|---|---|---|
| `json.set` | `ncallr dst, json.set, j, key, value` | Copy of `j` with `key` set. |
| `json.push` | `ncallr dst, json.push, j, value` | Copy of an array with `value` appended. |
| `json.delete` | `ncallr dst, json.delete, j, key` | Copy of object `j` without `key`. |
| `json.merge` | `ncallr dst, json.merge, base, patch` | Copy of `base` with fields from object `patch`. |

`json.set` accepts a string key for objects or a non-negative integer key for
arrays.
If the existing JSON value is not the needed container type, it is normalized
to an empty object or array first.

When setting an array index beyond the current length, the array is expanded
with JSON `null` values. Implicit expansion is capped at index `1000000` to
avoid accidental huge allocations.

`json.push` normalizes non-array input to an empty array before appending.

`json.delete` only removes keys from objects. Non-object input is returned
unchanged.

`json.merge` only merges when both values are objects. Otherwise, `base` is
returned unchanged.

Values inserted by `json.set` and `json.push` are converted as:

| VM value | JSON value |
|---|---|
| `int` | JSON number. |
| `float` | JSON number, or JSON `null` if not finite. |
| `str` | JSON string. |
| `json` | Original JSON value. |

### Convert

| Native | Call | Returns |
|---|---|---|
| `json.stringify` | `ncallr dst, json.stringify, j` | Compact JSON string. |
| `json.to_int` | `ncallr dst, json.to_int, j` | JSON integer as `int`. |
| `json.to_str` | `ncallr dst, json.to_str, j` | JSON string as `str`. |
| `json.from_int` | `ncallr dst, json.from_int, n` | Integer converted to JSON number. |
| `json.from_str` | `ncallr dst, json.from_str, s` | String converted to JSON string. |

`json.to_int` fails unless the JSON value is an integer. `json.to_str` fails
unless the JSON value is a string.

## `math.*`

| Native | Call | Returns |
|---|---|---|
| `math.abs` | `ncallr dst, math.abs, n` | Absolute value of an `int` or `float`; in-range only. |
| `math.min` | `ncallr dst, math.min, a, b` | Smaller numeric value. |
| `math.max` | `ncallr dst, math.max, a, b` | Larger numeric value. |
| `math.floor` | `ncallr dst, math.floor, n` | Rounded down as `int`; finite/in-range only. |
| `math.ceil` | `ncallr dst, math.ceil, n` | Rounded up as `int`; finite/in-range only. |
| `math.sqrt` | `ncallr dst, math.sqrt, n` | Square root as `float`. |
| `math.pow` | `ncallr dst, math.pow, base, exp` | `base ^ exp` as `float`. |
| `math.random` | `ncallr dst, math.random` | Random `float` in `[0.0, 1.0)`. |
| `math.rand_int` | `ncallr dst, math.rand_int, min, max` | Random `int` in `[min, max]`. |

`math.floor` and `math.ceil` fail if the rounded result is not finite or does
not fit in a Flint `int`.

`math.min`, `math.max`, and `math.pow` accept mixed `int` and `float`
arguments. Mixed `min` and `max` results are floats.

`math.rand_int` fails when `min > max`.

## `time.*`

| Native | Call | Returns |
|---|---|---|
| `time.now` | `ncallr dst, time.now` | Unix timestamp in milliseconds as `int`. |

If the system clock is before the Unix epoch, `time.now` returns `0`.

## `env.*`

| Native | Call | Returns |
|---|---|---|
| `env.get` | `ncallr dst, env.get, name` | Environment variable value, or `""` if missing. |

## `crypto.*`

| Native | Call | Returns |
|---|---|---|
| `crypto.uuid` | `ncallr dst, crypto.uuid` | Random UUID v4 string. |

## `ui.*`

`ui.*` natives build Flint's default styled HTML, one fragment at a time. Each
takes the current HTML accumulator as `html` and returns the accumulator with
a fragment appended — the same shape as `string.concat`. See
[UI Pages](/guide/ui-pages) for the page-level usage pattern.

| Native | Call | Returns |
|---|---|---|
| `ui.window` | `ncallr dst, ui.window, html, title` | `html` with the document shell, default stylesheet, browser tab title (`<title>`), and a styled page frame for `title` appended. |
| `ui.window_end` | `ncallr dst, ui.window_end, html` | `html` with a frame opened by `ui.window` closed. |
| `ui.card` | `ncallr dst, ui.card, html, title` | `html` with a bordered content panel for `title` appended. |
| `ui.card_end` | `ncallr dst, ui.card_end, html` | `html` with a panel opened by `ui.card` closed. |
| `ui.section` | `ncallr dst, ui.section, html, title` | `html` with an unframed content group for `title` appended. |
| `ui.section_end` | `ncallr dst, ui.section_end, html` | `html` with a group opened by `ui.section` closed. |
| `ui.row` | `ncallr dst, ui.row, html` | `html` with a horizontal responsive layout opened. |
| `ui.row_end` | `ncallr dst, ui.row_end, html` | `html` with a layout opened by `ui.row` closed. |
| `ui.column` | `ncallr dst, ui.column, html` | `html` with a vertical layout opened. |
| `ui.column_end` | `ncallr dst, ui.column_end, html` | `html` with a layout opened by `ui.column` closed. |
| `ui.title` | `ncallr dst, ui.title, html, value` | `html` with a heading for `value` appended. |
| `ui.text` | `ncallr dst, ui.text, html, value` | `html` with a paragraph for `value` appended. |
| `ui.field` | `ncallr dst, ui.field, html, label, value` | `html` with a label/value display row appended. |
| `ui.button` | `ncallr dst, ui.button, html, label, href` | `html` with a link styled as a button appended. |
| `ui.form` | `ncallr dst, ui.form, html, method, action` | `html` with an HTML form opened. |
| `ui.form_end` | `ncallr dst, ui.form_end, html` | `html` with a form opened by `ui.form` closed. |
| `ui.input` | `ncallr dst, ui.input, html, label, name` | `html` with a labeled text input appended. |
| `ui.submit` | `ncallr dst, ui.submit, html, label` | `html` with a submit button appended. |

`title`, `value`, `label`, `href`, `method`, `action`, and `name` must be
`str`. `ui.title`, `ui.text`, `ui.field`, and the label arguments of
`ui.button`/`ui.input`/`ui.submit` are HTML-escaped; `ui.button`'s `href`,
`ui.form`'s `method`/`action`, and `ui.input`'s `name` are attribute-escaped.

## `http.*`

`http.*` natives are available in HTTP handlers because the HTTP dispatcher
registers them per request. See [HTTP Runtime](/reference/http) for request
flow and response defaults.

### Request

| Native | Call | Returns |
|---|---|---|
| `http.method` | `ncallr dst, http.method` | HTTP method as `str`. |
| `http.path` | `ncallr dst, http.path` | Request path as `str`. |
| `http.body` | `ncallr dst, http.body` | Raw request body as `str`. |
| `http.param` | `ncallr dst, http.param, name` | Path parameter as `str`, or `""`. |
| `http.query` | `ncallr dst, http.query, name` | Query parameter as `str`, or `""`. |
| `http.header` | `ncallr dst, http.header, name` | Header value as lossy UTF-8 `str`, or `""`. |
| `http.cookie` | `ncallr dst, http.cookie, name` | Cookie value as `str`, or `""`. |
| `http.json_body` | `ncallr dst, http.json_body` | Parsed request body as `json`. |
| `http.form` | `ncallr dst, http.form, field` | URL-encoded form field as `str`, or `""`. |

`http.json_body` fails if the body is not valid JSON.

### Response

| Native | Call | Effect |
|---|---|---|
| `http.set_status` | `ncall http.set_status, code` | Set HTTP status code. |
| `http.set_header` | `ncall http.set_header, name, value` | Append a response header. |
| `http.set_cookie` | `ncall http.set_cookie, name, value` | Append a simple `Set-Cookie` header. |
| `http.text` | `ncall http.text, s` | Set plain text response body. |
| `http.html` | `ncall http.html, s` | Set HTML response body. |
| `http.json` | `ncall http.json, j` | Set JSON response body. |
| `http.redirect` | `ncall http.redirect, url` | Set status `302` and append `location`. |
| `http.abort` | `ncall http.abort` | Stop the handler and send the current response. |

`http.set_status` fails if the integer is not a valid HTTP status code.

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
takes the current HTML accumulator as its first argument and returns the
updated accumulator. UI pages normally call these through `section .render`;
direct calls are useful when reading generated output or writing custom route
handlers.

### Shell and Layout

| Native | Call |
|---|---|
| `ui.window` | `ncallr dst, ui.window, html, title` |
| `ui.window_end` | `ncallr dst, ui.window_end, html` |
| `ui.layout` | `ncallr dst, ui.layout, html` |
| `ui.layout_end` | `ncallr dst, ui.layout_end, html` |
| `ui.sidebar` | `ncallr dst, ui.sidebar, html` |
| `ui.sidebar_end` | `ncallr dst, ui.sidebar_end, html` |
| `ui.main` | `ncallr dst, ui.main, html` |
| `ui.main_end` | `ncallr dst, ui.main_end, html` |
| `ui.card` | `ncallr dst, ui.card, html, title` |
| `ui.card_end` | `ncallr dst, ui.card_end, html` |
| `ui.section` | `ncallr dst, ui.section, html, title[, subtitle]` |
| `ui.section_end` | `ncallr dst, ui.section_end, html` |
| `ui.row` | `ncallr dst, ui.row, html` |
| `ui.row_end` | `ncallr dst, ui.row_end, html` |
| `ui.column` | `ncallr dst, ui.column, html` |
| `ui.column_end` | `ncallr dst, ui.column_end, html` |
| `ui.toolbar` | `ncallr dst, ui.toolbar, html` |
| `ui.toolbar_end` | `ncallr dst, ui.toolbar_end, html` |
| `ui.action_bar` | `ncallr dst, ui.action_bar, html` |
| `ui.action_bar_end` | `ncallr dst, ui.action_bar_end, html` |
| `ui.footer` | `ncallr dst, ui.footer, html[, text]` |
| `ui.footer_end` | `ncallr dst, ui.footer_end, html` |
| `ui.divider` | `ncallr dst, ui.divider, html` |

### Navigation

| Native | Call |
|---|---|
| `ui.navbar` | `ncallr dst, ui.navbar, html` |
| `ui.nav_item` | `ncallr dst, ui.nav_item, html, label, href` |
| `ui.navbar_end` | `ncallr dst, ui.navbar_end, html` |
| `ui.menu` | `ncallr dst, ui.menu, html, title` |
| `ui.menu_item` | `ncallr dst, ui.menu_item, html, label, href` |
| `ui.menu_active` | `ncallr dst, ui.menu_active, html, label, href` |
| `ui.menu_end` | `ncallr dst, ui.menu_end, html` |
| `ui.breadcrumb` | `ncallr dst, ui.breadcrumb, html` |
| `ui.breadcrumb_item` | `ncallr dst, ui.breadcrumb_item, html, label, href` |
| `ui.breadcrumb_end` | `ncallr dst, ui.breadcrumb_end, html` |
| `ui.pagination` | `ncallr dst, ui.pagination, html` |
| `ui.page_item` | `ncallr dst, ui.page_item, html, label, href` |
| `ui.page_current` | `ncallr dst, ui.page_current, html, label` |
| `ui.pagination_end` | `ncallr dst, ui.pagination_end, html` |

### Content

| Native | Call |
|---|---|
| `ui.title` | `ncallr dst, ui.title, html, value` |
| `ui.text` | `ncallr dst, ui.text, html, value` |
| `ui.field` | `ncallr dst, ui.field, html, label, value` |
| `ui.badge` | `ncallr dst, ui.badge, html, label` |
| `ui.alert` | `ncallr dst, ui.alert, html, kind, message` |
| `ui.status` | `ncallr dst, ui.status, html, label, kind` |
| `ui.progress` | `ncallr dst, ui.progress, html, value, max` |
| `ui.meter` | `ncallr dst, ui.meter, html, value, max` |
| `ui.stat` | `ncallr dst, ui.stat, html, label, value` |
| `ui.code` | `ncallr dst, ui.code, html, value` |
| `ui.kbd` | `ncallr dst, ui.kbd, html, value` |
| `ui.link` | `ncallr dst, ui.link, html, label, href` |
| `ui.image` | `ncallr dst, ui.image, html, src, alt` |
| `ui.empty` | `ncallr dst, ui.empty, html, message` |

### Lists, Tables, and Groups

| Native | Call |
|---|---|
| `ui.list` | `ncallr dst, ui.list, html` |
| `ui.list_item` | `ncallr dst, ui.list_item, html, text` |
| `ui.list_end` | `ncallr dst, ui.list_end, html` |
| `ui.ol` | `ncallr dst, ui.ol, html` |
| `ui.ol_item` | `ncallr dst, ui.ol_item, html, text` |
| `ui.ol_end` | `ncallr dst, ui.ol_end, html` |
| `ui.table` | `ncallr dst, ui.table, html` |
| `ui.caption` | `ncallr dst, ui.caption, html, text` |
| `ui.tr` | `ncallr dst, ui.tr, html` |
| `ui.tr_end` | `ncallr dst, ui.tr_end, html` |
| `ui.th` | `ncallr dst, ui.th, html, label` |
| `ui.td` | `ncallr dst, ui.td, html, value` |
| `ui.tfoot` | `ncallr dst, ui.tfoot, html` |
| `ui.tfoot_end` | `ncallr dst, ui.tfoot_end, html` |
| `ui.table_end` | `ncallr dst, ui.table_end, html` |
| `ui.tabs` | `ncallr dst, ui.tabs, html` |
| `ui.tab` | `ncallr dst, ui.tab, html, label, id` |
| `ui.tabs_body` | `ncallr dst, ui.tabs_body, html` |
| `ui.tab_panel` | `ncallr dst, ui.tab_panel, html, id` |
| `ui.tab_panel_end` | `ncallr dst, ui.tab_panel_end, html` |
| `ui.tabs_end` | `ncallr dst, ui.tabs_end, html` |
| `ui.accordion` | `ncallr dst, ui.accordion, html` |
| `ui.accordion_item` | `ncallr dst, ui.accordion_item, html, title` |
| `ui.accordion_item_end` | `ncallr dst, ui.accordion_item_end, html` |
| `ui.accordion_end` | `ncallr dst, ui.accordion_end, html` |
| `ui.tree` | `ncallr dst, ui.tree, html` |
| `ui.tree_item` | `ncallr dst, ui.tree_item, html, label, href` |
| `ui.tree_group` | `ncallr dst, ui.tree_group, html, label` |
| `ui.tree_group_end` | `ncallr dst, ui.tree_group_end, html` |
| `ui.tree_end` | `ncallr dst, ui.tree_end, html` |
| `ui.steps` | `ncallr dst, ui.steps, html` |
| `ui.step` | `ncallr dst, ui.step, html, label, active` |
| `ui.steps_end` | `ncallr dst, ui.steps_end, html` |

### Forms, Actions, and Dialogs

| Native | Call |
|---|---|
| `ui.button` | `ncallr dst, ui.button, html, label, href` |
| `ui.form` | `ncallr dst, ui.form, html, method, action` |
| `ui.form_end` | `ncallr dst, ui.form_end, html` |
| `ui.fieldset` | `ncallr dst, ui.fieldset, html, legend` |
| `ui.fieldset_end` | `ncallr dst, ui.fieldset_end, html` |
| `ui.input` | `ncallr dst, ui.input, html, label, name` |
| `ui.password` | `ncallr dst, ui.password, html, label, name` |
| `ui.number` | `ncallr dst, ui.number, html, label, name` |
| `ui.file` | `ncallr dst, ui.file, html, label, name` |
| `ui.textarea` | `ncallr dst, ui.textarea, html, label, name` |
| `ui.select` | `ncallr dst, ui.select, html, label, name` |
| `ui.option` | `ncallr dst, ui.option, html, label, value` |
| `ui.select_end` | `ncallr dst, ui.select_end, html` |
| `ui.checkbox` | `ncallr dst, ui.checkbox, html, label, name, value` |
| `ui.radio` | `ncallr dst, ui.radio, html, label, name, value` |
| `ui.hidden` | `ncallr dst, ui.hidden, html, name, value` |
| `ui.submit` | `ncallr dst, ui.submit, html, label` |
| `ui.dialog` | `ncallr dst, ui.dialog, html, id, title` |
| `ui.dialog_end` | `ncallr dst, ui.dialog_end, html` |
| `ui.dialog_trigger` | `ncallr dst, ui.dialog_trigger, html, label, id` |
| `ui.dialog_alert` | `ncallr dst, ui.dialog_alert, html, id, title, message` |
| `ui.dialog_confirm` | `ncallr dst, ui.dialog_confirm, html, id, title, message, action` |
| `ui.dialog_prompt` | `ncallr dst, ui.dialog_prompt, html, id, title, label, name, action` |

Text-like values are escaped for HTML where appropriate; hrefs, form
attributes, names, ids, and similar arguments are attribute-escaped by the
individual native.

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

# First API

In this tutorial you will build three routes:

| Route | What it shows |
|---|---|
| `GET /hello` | Plain text responses. |
| `GET /users/:id` | Path parameters and JSON responses. |
| `POST /echo` | JSON request bodies and custom status codes. |

Start with:

```sh
flint new my-api
cd my-api
flint serve
```

Open the `routes/` directory in another terminal or editor.

## How Route Files Work

The server loads every `.fl` file directly inside `routes/`:

```txt
routes/
├── hello.fl
├── users.fl
└── echo.fl
```

Each file may define functions and route declarations. A route declaration maps
an HTTP method and path to a function:

```txt
route GET "/hello" -> say_hello
```

## Route 1: Plain Text

Create or replace `routes/hello.fl`:

```txt
say_hello:
    mov r0, "Hello from Flint!"
    ncall http.text, r0
    ret

route GET "/hello" -> say_hello
```

Test it:

```sh
curl http://127.0.0.1:3000/hello
```

Expected response:

```txt
Hello from Flint!
```

The request flow:

1. The router matches `GET /hello`.
2. The runtime creates a fresh VM.
3. The VM calls `say_hello`.
4. `http.text` sets the response body.
5. `ret` returns to the HTTP runtime.

## Route 2: Path Params and JSON

Create `routes/users.fl`:

```txt
show_user:
    mov r0, "id"
    ncallr r1, http.param, r0

    ncallr r2, json.object
    mov r3, "id"
    ncallr r2, json.set, r2, r3, r1

    mov r3, "source"
    mov r4, "Flint"
    ncallr r2, json.set, r2, r3, r4

    ncall http.json, r2
    ret

route GET "/users/:id" -> show_user
```

Test it:

```sh
curl http://127.0.0.1:3000/users/42
```

Expected response:

```json
{"id":"42","source":"Flint"}
```

The key pattern is:

```txt
mov r0, "id"
ncallr r1, http.param, r0
```

`http.param` receives the parameter name and returns the value captured from
the path.

## Building JSON

There is no JSON literal syntax. Build JSON values with native functions:

| Goal | Call |
|---|---|
| Empty object | `ncallr r0, json.object` |
| Empty array | `ncallr r0, json.array` |
| Set object field | `ncallr r0, json.set, r0, key, value` |
| Push array item | `ncallr r0, json.push, r0, value` |

`json.set` and `json.push` return new JSON documents. Store the result:

```txt
ncallr r0, json.object
mov r1, "name"
mov r2, "Ada"
ncallr r0, json.set, r0, r1, r2
```

## Route 3: Read a POST Body

Create `routes/echo.fl`:

```txt
echo_body:
    ncallr r0, http.json_body

    ncallr r1, json.object
    mov r2, "received"
    ncallr r1, json.set, r1, r2, r0

    mov r2, 201
    ncall http.set_status, r2
    ncall http.json, r1
    ret

route POST "/echo" -> echo_body
```

Test it:

```sh
curl -i -X POST http://127.0.0.1:3000/echo \
  -H 'Content-Type: application/json' \
  -d '{"name":"Ada"}'
```

Expected body:

```json
{"received":{"name":"Ada"}}
```

`http.json_body` parses the request body as JSON. Invalid JSON becomes a
runtime error and the HTTP runtime returns `500`.

## Change the Status Code

Every response starts as `200 OK`. Set another status with:

```txt
mov r0, 201
ncall http.set_status, r0
```

The response is sent after the handler returns, so the status can be set before
or after the body.

## Try Everything

```sh
curl http://127.0.0.1:3000/hello
curl http://127.0.0.1:3000/users/42
curl -i -X POST http://127.0.0.1:3000/echo \
  -H 'Content-Type: application/json' \
  -d '{"x":1}'
curl -i http://127.0.0.1:3000/missing
```

Next: [Visual Pages](/guide/pages).

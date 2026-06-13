# Installation

::: warning Experimental project
`Flint` does not have a published installer yet. Build the CLI locally with
Rust.
:::

## Requirements

Install Rust from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Check that the toolchain is available:

```sh
rustc --version
cargo --version
```

## Build the CLI

From the repository root:

```sh
cargo build --bin flint
```

The debug executable is created at:

```sh
./target/debug/flint
```

To install it on your `PATH`:

```sh
cargo install --path crates/flint-cli --bin flint
```

## Verify the Repository

Run the test suite:

```sh
cargo test
```

This exercises the compiler, VM, native functions, HTTP runtime, and CLI crate.

## Create a Project

```sh
flint new my-app
cd my-app
flint serve
```

The minimal template creates:

```txt
my-app/
├── flint.toml
├── pages/
│   └── index.flint.ui
└── routes/
    └── hello.fl
```

Try both generated endpoints:

```sh
curl http://127.0.0.1:3000/hello
```

and open:

```txt
http://127.0.0.1:3000/
```

## Use the Tasks Template

The `tasks` template creates a small JSON API with routes, services, and
repositories:

```sh
flint new my-app --template tasks
cd my-app
flint serve
```

Try:

```sh
curl http://127.0.0.1:3000/tasks
curl http://127.0.0.1:3000/tasks/1
curl -X POST http://127.0.0.1:3000/tasks \
  -H 'Content-Type: application/json' \
  -d '{"title":"Learn Flint"}'
```

## CLI Commands

| Command | What it does |
|---|---|
| `flint new <name>` | Create a project with the minimal template. |
| `flint new <name> --template tasks` | Create a complete Tasks API example. |
| `flint serve [dir]` | Start the development HTTP server. |
| `flint run [dir]` | Alias for `flint serve`. |
| `flint build [dir]` | Build a standalone release binary in `dist/`. |
| `flint version` | Print the CLI version. |

Next: [First API](/guide/first-api).

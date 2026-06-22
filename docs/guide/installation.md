# Installation

Get the `flint` CLI on your machine: download a prebuilt binary, or build it
from source with Rust.

## Download a Prebuilt Binary

### Install Script

On Linux and macOS:

```sh
curl -fsSL https://flint.devlayer.app/install.sh | sh
```

On Windows (PowerShell):

```powershell
irm https://flint.devlayer.app/install.ps1 | iex
```

This downloads the right archive for your platform from
[GitHub Releases](https://github.com/MateusGX/flint/releases), extracts the
`flint` binary, and installs it to `~/.local/bin` (Linux/macOS) or
`%LOCALAPPDATA%\Flint\bin` (Windows). Set `FLINT_VERSION` to install a
specific release tag, or `FLINT_INSTALL_DIR` to change the install location.

### Manual Download

Alternatively, download the archive for your platform directly:

| Platform | Archive |
|---|---|
| Linux x86_64 | `flint-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `flint-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Intel | `flint-x86_64-apple-darwin.tar.gz` |
| macOS Apple Silicon | `flint-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `flint-x86_64-pc-windows-msvc.zip` |

Extract it and put the `flint` binary on your `PATH`.

### Uninstall

On Linux and macOS:

```sh
curl -fsSL https://flint.devlayer.app/uninstall.sh | sh
```

On Windows (PowerShell):

```powershell
irm https://flint.devlayer.app/uninstall.ps1 | iex
```

This removes the binary from the install directory (and, on Windows, the
`PATH` entry the install script added). Set `FLINT_INSTALL_DIR` if you
installed to a custom location.

## Build from Source

Alternatively, build the CLI locally with Rust.

### Requirements

Install Rust from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Check that the toolchain is available:

```sh
rustc --version
cargo --version
```

### Build the CLI

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

### Verify the Repository

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
├── app/
│   └── index.flint.ui
└── api/
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
| `flint new <name> --template static` | Create a UI-only static site project. |
| `flint serve [dir]` | Start the development HTTP server. |
| `flint run [dir]` | Alias for `flint serve`. |
| `flint build [dir]` | Compile portable bytecode into `dist/`. |
| `flint build --static [dir]` | Export UI pages to static HTML in `dist/`. |
| `flint version` | Print the CLI version. |

## Editor Support

Install the [Flint Language](https://marketplace.visualstudio.com/items?itemName=mateusam.flint-vscode)
extension for VS Code to get syntax highlighting, snippets, and language
configuration for `.fl` route/service/repository files and `.flint.ui` pages.

Next: [First API](/guide/first-api).

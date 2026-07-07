# opit

A terminal UI for viewing and playing with OpenAPI documents: browse operations across
multiple keyboard-navigable panes, and send live HTTP requests with auth support.

## Prerequisites

- Rust toolchain (`cargo`, `rustc`) — install via [rustup](https://rustup.rs) if you don't have one.

## Build

```bash
cargo build --release
```

The binary lands at `target/release/opit`.

## Install (local)

To have `opit` available as a regular command from any directory:

```bash
cargo install --path .
```

This builds and copies the binary into `~/.cargo/bin/opit`, which is on `PATH` if you
installed Rust via rustup. To update after making changes, rerun the same command. To
remove it: `cargo uninstall opit`.

## Usage

```bash
opit path/to/your-spec.json
opit path/to/your-spec.yaml
```

Optional auth flags, applied to every request the app sends:

```bash
opit spec.json --bearer-token YOUR_TOKEN
opit spec.json --header "X-API-Key=secret123" --header "X-Custom=value"
```

### Keybindings

| Key             | Action                                                |
|-----------------|--------------------------------------------------------|
| `Tab`           | Cycle focus forward through panes                     |
| `Shift+Tab`     | Cycle focus backward through panes                    |
| `Up` / `Down`   | Move selection in the Endpoints pane                  |
| `Enter`         | Send the selected operation as a live HTTP request    |
| `q` / `Esc`     | Quit                                                    |

### Panes

- **Endpoints** — list of operations (`METHOD /path`) from the loaded spec
- **Request Builder** — parameters (path/query/header) for the selected operation
- **Auth Config** — security schemes declared in the spec's `components.securitySchemes`
- **Response Viewer** — status and body of the last request sent

The base URL used for requests is the first entry in the spec's top-level `servers` array.

## Known limitations

- There's no interactive way yet to type parameter *values* into the Request Builder
  pane — requests go out with empty params, so an endpoint requiring a path parameter
  (e.g. `/pets/{petId}`) will hit the literal `{petId}` in the URL.
- OAuth2 client_credentials auth is implemented and unit-tested (`auth::oauth2::TokenCache`)
  but not yet wired into the live-send path — only `--bearer-token` and `--header` are
  currently applied to outgoing requests.

## Development

This project follows strict TDD: tests live under `tests/` as integration tests, one file
per concern; production code lives under `src/`.

```bash
cargo test
```

## Cross-compiling for Linux

Requires [Docker](https://www.docker.com) (or a compatible engine like OrbStack) and the
[`cross`](https://github.com/cross-rs/cross) tool. The crates.io release of `cross` is
stale — install from git:

```bash
cargo install cross --git https://github.com/cross-rs/cross --locked
rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
```

Binaries land at:
- `target/x86_64-unknown-linux-gnu/release/opit`
- `target/aarch64-unknown-linux-gnu/release/opit`

**Glibc note**: these binaries dynamically link against a fairly recent glibc (2.39 at
time of writing) and need a target machine running Ubuntu 24.04+/Debian 13+ or newer —
they will fail with a `GLIBC_x.xx not found` error on older distros (e.g. Ubuntu 22.04,
Debian 12). If you need binaries that run on any Linux regardless of glibc version
(older servers, Alpine, minimal containers), build the musl targets instead
(`x86_64-unknown-linux-musl` / `aarch64-unknown-linux-musl`) for fully static binaries.

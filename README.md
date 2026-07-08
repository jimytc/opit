# opit

A terminal UI for viewing and playing with OpenAPI documents: browse operations across
multiple keyboard-navigable panes, and send live HTTP requests with auth support.

## Install

### macOS — Homebrew (recommended)

```bash
brew install jimytc/opit/opit
```

### macOS — DMG

Download the `.dmg` from the [latest release](https://github.com/jimytc/opit/releases/latest),
open it, and copy `opit` from the mounted volume into a directory on your `PATH` (e.g.
`/usr/local/bin` or `~/.local/bin`). Apple Silicon (arm64) only.

### Linux — prebuilt tarball

Download the tarball for your architecture from the
[latest release](https://github.com/jimytc/opit/releases/latest)
(`opit-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` or `-aarch64-unknown-linux-gnu.tar.gz`),
extract it, and move the `opit` binary into a directory on your `PATH`:

```bash
tar -xzf opit-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz
sudo mv opit-vX.Y.Z-x86_64-unknown-linux-gnu/opit /usr/local/bin/
```

**Glibc note**: these binaries need a fairly recent glibc (2.39+, i.e. Ubuntu 24.04+/Debian
13+) — see the [cross-compiling section](#cross-compiling-for-linux) below for details.

### Build from source

Requires the Rust toolchain (`cargo`, `rustc`) — install via [rustup](https://rustup.rs)
if you don't have one.

```bash
cargo build --release
```

The binary lands at `target/release/opit`. To install it as a regular command on `PATH`:

```bash
cargo install --path .
```

This copies the binary into `~/.cargo/bin/opit`, which is on `PATH` if you installed Rust
via rustup. To update after making changes, rerun the same command. To remove it:
`cargo uninstall opit`.

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

| Key             | Action                                                                 |
|-----------------|-------------------------------------------------------------------------|
| `Tab`           | Cycle focus forward through panes (disabled while editing a field)     |
| `Shift+Tab`     | Cycle focus backward through panes (disabled while editing a field)    |
| `Up` / `Down`   | Move selection: endpoint in Endpoints, row in Request Builder/Auth Config |
| `Enter`         | In Endpoints: send the selected operation as a live HTTP request. In Request Builder/Auth Config: start editing the selected row, or commit the in-progress value if already editing |
| `Esc`           | Cancel an in-progress edit (or quit, if not currently editing)          |
| Any character / `Backspace` | While editing a row, types into / erases from that row's value  |
| `q`             | Quit (only when not currently editing a field)                        |

### Panes

- **Endpoints** — list of operations (`METHOD /path`) from the loaded spec; the selected
  row is highlighted
- **Request Builder** — one row per parameter (path/query/header) for the selected
  operation, plus a trailing "Body" row if the operation accepts a request body; select a
  row and press `Enter` to type its value. Below the rows, a live `curl`-equivalent
  preview shows exactly what would be sent — it updates as you type, even before
  committing a field with `Enter`, reflecting in-progress edits in both this pane and
  Auth Config
- **Auth Config** — one row per security scheme declared in the spec's
  `components.securitySchemes`; select a row and press `Enter` to type its credential.
  API Key (header, query, or cookie location) and HTTP Bearer schemes take a single
  value; HTTP Basic and OAuth2 client_credentials (when the spec declares a
  `clientCredentials` flow) take `user:pass`/`client_id:client_secret` respectively
  (hinted in the row text) — OAuth2's token is fetched and cached automatically at
  send time. OpenID Connect and other OAuth2 flows are shown but marked
  "(not editable yet)" — see Known limitations
- **Response Viewer** — status, headers, and body of the last request sent. JSON bodies
  (response, and the request body shown in the curl preview) are pretty-printed; long
  lines soft-wrap within the pane instead of being cut off

The base URL used for requests is the first entry in the spec's top-level `servers` array.
Switching the selected operation in Endpoints clears any in-progress Request Builder
values (they're per-operation); Auth Config values persist across operation switches
(credentials are spec-wide). Credentials/params entered interactively are combined with
any `--bearer-token`/`--header` CLI flags when sending.

## Known limitations

- OAuth2 flows other than `clientCredentials` (authorization code, implicit, password)
  and OpenID Connect discovery are not supported — those schemes are shown in Auth
  Config but marked "(not editable yet)".
- The curl preview does not shell-escape header/body values — a value containing a
  single quote (`'`) will produce a command that isn't directly copy-paste-safe.
- The curl preview can't show OAuth2 client_credentials auth before you send — fetching
  a real token requires a network call, which only happens once you press `Enter` on
  Endpoints, not on every keystroke.

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

## Releasing

Pushing a `vX.Y.Z` tag triggers `.github/workflows/release.yml`, which builds and
publishes a GitHub Release with:
- `opit-vX.Y.Z-aarch64-apple-darwin.tar.gz` and `.dmg` (macOS)
- `opit-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` and `opit-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz` (Linux)

After a release, update the Homebrew tap
([jimytc/homebrew-opit](https://github.com/jimytc/homebrew-opit)) — see that repo's
README for the steps (bump `url`/`sha256`/`version` in `Formula/opit.rb`).

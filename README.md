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
| `Ctrl+Alt+1`-`5` | Jump focus directly to a pane: `1` Endpoints, `2` Auth Config, `3` Request Builder, `4` Curl Preview, `5` Response Viewer (`Alt` is `Option` on macOS keyboards; disabled while editing a field or filtering) |
| `Up` / `Down`   | Move selection: endpoint in Endpoints, row in Request Builder/Auth Config (auto-scrolls to keep the selection visible). In Curl Preview/Response Viewer: scroll the content up/down one line at a time |
| `[` / `]`       | In Request Builder (not while editing): switch between the Header/Parameters/Payload sub-tabs, backward/forward with wraparound |
| `s`             | In Endpoints (not while filtering): cycle the active server, when the spec declares more than one |
| `/`             | In Endpoints: start typing a filter that narrows the endpoint list live |
| `Enter`         | In Endpoints: send the selected operation as a live HTTP request, or (while filtering) keep the current filter text and stop typing. In Request Builder/Auth Config: start editing the selected row, or commit the in-progress value if already editing — except on the Body row (see below), where Enter inserts a newline instead |
| `Ctrl+S`        | While editing any Request Builder/Auth Config row, commit the in-progress value (the only way to commit the Body row, since Enter there inserts a newline) |
| `Esc`           | Cancel an in-progress edit, or (while filtering) clear the filter and stop typing — otherwise quit |
| Any character / `Backspace` / paste | While editing a row or typing a filter, types into / erases from / pastes into that value. Paste is inserted as a whole chunk, embedded newlines included |
| `q`             | Quit (only when not currently editing a field or filtering)           |

### Panes

Three columns: the left column is a single full-height **Endpoints** pane; the middle
column stacks **Auth Config** (top) and **Request Builder** (bottom) in a 2:3 height
ratio; the right column stacks **Curl Preview** (top) and **Response Viewer** (bottom),
also in a 2:3 height ratio. `Tab`/`Shift+Tab` cycle focus in that same order (Endpoints →
Auth Config → Request Builder → Curl Preview → Response Viewer, wrapping), and
`Ctrl+Alt+1`-`5` jump directly to any of them.

- **Endpoints** — list of operations from the loaded spec, grouped under a header row
  per first tag (or "Untagged" if an operation has none), in first-appearance order; the
  selected row is highlighted. Each operation shows `METHOD /path`, plus an indented
  summary line underneath when the spec provides a `summary` (or falls back to
  `description`). The pane title shows the spec's title/version, the active server (when
  the spec declares more than one) and the current filter text (when set). Press `/` to
  filter the list live (case-insensitive substring match against method, path, and
  summary); press `s` to cycle the active server
- **Auth Config** — one row per security scheme declared in the spec's
  `components.securitySchemes`; select a row and press `Enter` to type its credential.
  API Key (header, query, or cookie location) and HTTP Bearer schemes take a single
  value; HTTP Basic and OAuth2 client_credentials (when the spec declares a
  `clientCredentials` flow) take `user:pass`/`client_id:client_secret` respectively
  (hinted in the row text) — OAuth2's token is fetched and cached automatically at
  send time. OpenID Connect and other OAuth2 flows are shown but marked
  "(not editable yet)" — see Known limitations
- **Request Builder** — three sub-tabs, switched with `[`/`]` (shown in the pane title as
  e.g. `[Header] Parameters Payload`, with the active one bracketed):
  - **Header** — one row per `header`-location parameter declared for the selected
    operation, plus any custom headers you've added, plus a permanent `+ Add Header` row
    at the bottom.
  - **Parameters** — one row per `path`/`query`/`cookie`-location parameter, plus any
    custom query parameters you've added, plus a permanent `+ Add Parameter` row.
  - **Payload** — a single, always-present `Body` row — editable even when the spec
    declares no `requestBody` for the operation, so you can send a body the spec doesn't
    mention. Shows a generated example JSON hint (`Body — e.g. {...}`) until you commit a
    value. Unlike other rows, it supports multi-line editing: `Enter` inserts a newline
    instead of committing, pasted text (including embedded newlines) is inserted in one
    step, and `Ctrl+S` commits the value.

  Select any row and press `Enter` to type its value. Required parameters are labeled
  accordingly, and sending is blocked with a message in Response Viewer if any are left
  empty. To add a header or query parameter the spec doesn't declare: select the `+ Add`
  row, press `Enter`, type `name=value` (e.g. `X-Api-Version=2` or `region=us-west`), and
  commit with `Enter` or `Ctrl+S` — it becomes a real, re-editable row (labeled `custom`),
  and a fresh `+ Add` row appears below it for adding another. Path parameters aren't
  addable this way (they're fixed by the URL template). A malformed entry (no `=`, an
  empty name, or a name that duplicates an existing row) is silently discarded rather than
  added
- **Curl Preview** — a read-only pane showing exactly what would be sent as a
  `curl`-equivalent command; it updates as you type, even before committing a field with
  `Enter`, reflecting in-progress edits in both Request Builder and Auth Config
- **Response Viewer** — status, headers, and body of the last request sent. JSON bodies
  (response, and the request body shown in the curl preview) are pretty-printed; long
  lines soft-wrap within the pane instead of being cut off

Switching the selected operation in Endpoints clears any in-progress Request Builder
values, including custom headers/query parameters added via `+ Add` (they're per-operation,
tracked by method+path so a value never leaks onto the wrong operation even as filtering
reshuffles the list) — the active Request Builder sub-tab stays as you left it, though.
Auth Config values persist across operation switches (credentials are spec-wide).
Credentials/params entered interactively are combined with any `--bearer-token`/`--header`
CLI flags when sending.

## Known limitations

- `Ctrl+Alt+1`-`5` (pane jump) relies on your terminal supporting the
  [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) to
  reliably disambiguate the modifier combination; opit enables it automatically when
  supported. Terminals without support (and tmux sessions without `set -g extended-keys
  always` in `tmux.conf`, forwarding a terminal that itself supports the protocol) may
  not deliver some digits correctly — `Tab`/`Shift+Tab` cycling always works regardless.
- OAuth2 flows other than `clientCredentials` (authorization code, implicit, password)
  and OpenID Connect discovery are not supported — those schemes are shown in Auth
  Config but marked "(not editable yet)".
- The curl preview does not shell-escape header/body values — a value containing a
  single quote (`'`) will produce a command that isn't directly copy-paste-safe.
- The curl preview can't show OAuth2 client_credentials auth before you send — fetching
  a real token requires a network call, which only happens once you press `Enter` on
  Endpoints, not on every keystroke.
- The required-parameter check only covers path/query/header/cookie parameters marked
  `required` in the spec — it does not check whether a request body itself is required.
- Generated request body example hints only fill in properties declared inline in the
  schema; properties defined via `$ref` are skipped, and a request body whose own schema
  is a top-level `$ref` gets no example hint at all (no cross-schema resolution).
- Custom headers/query parameters added via `+ Add` can't be removed once added (only
  cleared entirely by switching to a different operation) — you can still edit their value
  in place by pressing `Enter` on the row.

## Development

This project follows strict TDD: tests live under `tests/` as integration tests, one file
per concern; production code lives under `src/`.

```bash
cargo test
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for how the codebase is put together
(C4 diagrams, sequence/state diagrams, module responsibilities, and the non-obvious
design decisions behind the Request Builder tabs and OAuth2 handling) — start there
before making non-trivial changes.

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

Create the tag with an annotated message summarizing what's new (`git tag -a vX.Y.Z -m "..."`)
— this becomes the release's notes, so write real content, not just the version number.
Pushing the tag triggers `.github/workflows/release.yml`, which builds and publishes a
GitHub Release with:
- `opit-vX.Y.Z-aarch64-apple-darwin.tar.gz` and `.dmg` (macOS)
- `opit-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` and `opit-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz` (Linux)

The release's notes combine the tag's annotated message with GitHub's auto-generated
"Full Changelog" compare link appended below it.

After a release, update the Homebrew tap
([jimytc/homebrew-opit](https://github.com/jimytc/homebrew-opit)) — see that repo's
README for the steps (bump `url`/`sha256`/`version` in `Formula/opit.rb`).

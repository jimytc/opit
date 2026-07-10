# Getting Started

A guided first hour with opit: build it, run it against a sample spec, poke every pane,
then make one trivial TDD change end to end. By the end you should have everything you
need to start a real change.

Prerequisites: the Rust toolchain (`cargo`, `rustc`) via [rustup](https://rustup.rs). No
other setup needed.

## 1. Build and run

```bash
git clone <your fork or the repo>
cd openapi-terminal-app
cargo build
cargo run -- examples/minimal.yaml
```

`examples/minimal.yaml` is a small bundled OpenAPI spec built specifically to exercise
every corner of the UI: two tags (`pets`, `orders`), a path param, a query param, a
header param, a request body, and two auth schemes (Bearer + OAuth2 client credentials).
You should see something like this (exact proportions vary with terminal size):

```text
┌─Endpoints────────────┬─Auth Config──────────┬─Curl Preview──────────┐
│ pets                  │ bearerAuth             │                        │
│  GET  /pets           │  (unset)               │                        │
│  POST /pets           │ oauth2ClientCreds      │                        │
│  GET  /pets/{petId}   │  (unset,               ├─Response Viewer───────┤
│ orders                │   client_id:secret)    │                        │
│  POST /orders         ├─Request Builder────────┤                        │
│                        │ [Header] Params ...    │                        │
│                        │  X-Request-Id          │                        │
│                        │                        │                        │
└────────────────────────┴────────────────────────┴────────────────────────┘

(left column: Endpoints, full height · middle column: Auth Config on top,
Request Builder below, 2:3 split · right column: Curl Preview on top,
Response Viewer below, 2:3 split — see the README's pane layout section)
```

Press `q` to quit at any time.

## 2. Tour the panes

Follow along in the running app (relaunch with the command above if you quit):

1. **Endpoints** is focused by default. Press `Down` a few times to move through
   `GET /pets`, `POST /pets`, `GET /pets/{petId}`, `POST /orders` (each row shows
   `METHOD /path`, plus an indented summary line underneath, grouped under its tag —
   `pets` then `orders`).
2. Press `/` and type `order` — the list narrows to just `POST /orders` live. Press `Esc`
   to clear the filter.
3. Press `Tab` to move focus to **Auth Config**. You'll see one row per security scheme
   the spec declares: `bearerAuth` and `oauth2ClientCreds`. Press `Enter` on
   `bearerAuth`, type `test-token-123`, press `Enter` again to commit.
4. Select `GET /pets/{petId}` in Endpoints (`Ctrl+Alt+1` to jump there directly if
   needed), then `Ctrl+Alt+3` to jump to **Request Builder**. The `Header` sub-tab
   (shown bracketed in the pane title) has its `X-Request-Id` header param; press `]` to
   switch to `Parameters` and you'll see its `petId` path param instead. Press `[`/`]`
   to switch back and forth between the Header/Parameters/Payload sub-tabs.
5. Press `Ctrl+Alt+4` to jump to **Curl Preview** — it already reflects the bearer token
   you typed in step 3, live, even though you haven't sent anything yet.
6. With `GET /pets/{petId}` still selected: jump back to Request Builder
   (`Ctrl+Alt+3`), switch to the `Parameters` tab (`]` if needed), select the `petId`
   row and fill it in (`Enter`, type a value, `Enter` to commit). Then jump to
   Endpoints (`Ctrl+Alt+1`) and press `Enter` on the `GET /pets/{petId}` row itself to
   actually send the request. Since `api.example.com` isn't a real server, check
   **Response Viewer** (`Ctrl+Alt+5`) — you should see a connection error, which
   confirms the request really was sent over the network, not faked.
7. Try it the other way: select `GET /pets/{petId}` again but leave `petId` empty
   (switch operations and back, or restart, to clear it), press `Enter` in Endpoints to
   send. Response Viewer should show a "Missing required parameter(s)" message instead —
   no network call happens.

At this point you've touched all five panes and seen the required-param check,
live curl preview, and auth wiring in action. See the
[README's Panes section](../README.md#panes) for the full reference once you're past
first exploration.

## 3. Read the architecture doc

Before writing any code, read [`ARCHITECTURE.md`](ARCHITECTURE.md) — at minimum the C4
diagrams (§2) and the module responsibility table (§2.3). Keep
[`GLOSSARY.md`](GLOSSARY.md) open alongside it for any unfamiliar term.

## 4. Make a trivial change, TDD-style

This walks through the full loop from `CONTRIBUTING.md §2` on something safe to throw
away afterward. Pick something small and observable, for example: *the Endpoints pane
title should show the operation count in parentheses.*

```text
   ┌─────────┐     ┌─────────┐     ┌──────────┐
   │  RED    │ --> │  GREEN  │ --> │  cleanup │
   │ (test)  │     │ (src/)  │     │ (revert) │
   └─────────┘     └─────────┘     └──────────┘
```

1. **Find the right test file.** Use the map in
   [`TESTING.md`](TESTING.md#module--test-file-map) — rendering the Endpoints pane is
   `src/ui/endpoint_list.rs`, tested by `tests/ui_endpoint_list.rs`.
2. **Write a failing test** in that file asserting the pane title contains the operation
   count, following the existing tests' pattern in that file (render into a
   `ratatui::buffer::Buffer`, assert on cell text).
3. **Confirm it fails**: `cargo test --test ui_endpoint_list`.
4. **Commit the failing test alone**: `git add tests/ui_endpoint_list.rs && git commit -m
   "test: endpoint list title shows operation count"`.
5. **Make it pass** with the smallest change to `src/ui/endpoint_list.rs`.
6. **Confirm it's green**: `cargo test --test ui_endpoint_list`, then `cargo test` for
   the full suite.
7. **Commit the fix alone**: `git add src/ui/endpoint_list.rs && git commit -m "feat:
   show operation count in endpoint list title"`.
8. This was a practice run — either open it as a real PR if you like the change, or
   reset it: `git reset --hard HEAD~2` (only if you don't want to keep it — check
   `git status` first if you're not sure what else is in your tree).

## 5. What's next

- [`CONTRIBUTING.md`](../CONTRIBUTING.md) — the full workflow, PR checklist, and commit
  conventions for a real change.
- [`ARCHITECTURE.md §8`](ARCHITECTURE.md#8-where-to-add-things) — "I want to add a
  ___" → which files to touch.
- [`TESTING.md`](TESTING.md) — the module → test file map, and the manual verification
  checklist for UI-visible changes.

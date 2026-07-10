# Testing opit

opit is built under strict TDD (see `CONTRIBUTING.md §2`). This doc is the practical
half: which test file covers which `src/` module, how the tests are structured (no
mocked terminal, no snapshot files), and the manual-verification checklist for
UI-visible changes. For the *why* behind this approach, see
[`ARCHITECTURE.md §6`](ARCHITECTURE.md#6-testing-philosophy-read-this-before-writing-any-code).

Run everything: `cargo test`. Run one file: `cargo test --test <file_stem>`, e.g.
`cargo test --test pane_editor`.

## Module → test file map

If you're changing a `src/` file, start by finding its row here — these are the tests
that will tell you if you broke something, and the tests you should extend for new
behavior.

| `src/` module | Test file(s) | What's exercised |
|---|---|---|
| `cli.rs` | `tests/cli_args.rs` | Flag parsing (`--bearer-token`, `--header`, positional spec path) |
| `spec/mod.rs` | `tests/spec_parsing.rs`, `tests/spec_load_from_path.rs`, `tests/spec_yaml_loading.rs`, `tests/spec_base_url.rs` | Loading/parsing a spec file (JSON + YAML), server/base-URL resolution |
| `spec/operation.rs` | `tests/spec_operations_all_methods.rs`, `tests/spec_operation_parameters.rs`, `tests/spec_operation_request_body.rs` | Turning `openapiv3` operations into opit's `Operation`/`Parameter`, example-body generation |
| `spec/security.rs` | `tests/spec_security_schemes.rs` | Parsing `components.securitySchemes` into `SecurityScheme`/`SecuritySchemeKind` |
| `app/mod.rs` (`AppState`) | `tests/app_focus.rs`, `tests/app_endpoint_selection.rs`, `tests/app_pane_editing.rs`, `tests/app_request_builder_custom_params.rs`, `tests/app_response_state.rs` | Pane focus cycling, endpoint selection/filtering, key handling, the `+ Add` row flow, response state |
| `app/pane_editor.rs` (`PaneEditor`) | `tests/pane_editor.rs` | The generic row-select/edit/commit/cancel state machine, in isolation from any pane |
| `ui/endpoint_list.rs` | `tests/ui_endpoint_list.rs`, `tests/ui_endpoint_list_filter.rs` | Rendered `List` contents/highlighting, live filtering |
| `ui/request_builder.rs` | `tests/ui_request_builder.rs` | Per-tab row layout, Add-row rendering, highlight styles |
| `ui/auth_config.rs` | `tests/ui_auth_config.rs` | Per-scheme row rendering, "(not editable yet)" hints |
| `ui/response_viewer.rs` | `tests/ui_response_viewer.rs` | Status/header/body rendering, JSON pretty-printing, line wrapping |
| `request/mod.rs` (build/preview) | `tests/request_building.rs`, `tests/request_build_preview.rs`, `tests/request_required_params.rs`, `tests/request_header_helpers.rs`, `tests/request_param_interpretation.rs`, `tests/request_body_interpretation.rs`, `tests/request_body_formatting.rs`, `tests/request_to_curl.rs` | Assembling an `HttpRequest`, required-param checking, curl rendering |
| `request/http_client.rs` | `tests/http_client.rs` | The `HttpClient` trait contract (a fake implementation, no real network) |
| `auth/mod.rs` | `tests/auth_credentials.rs`, `tests/auth_credential_interpretation.rs`, `tests/split_credential_pair.rs` | `Credential` construction/application, `user:pass` splitting |
| `auth/oauth2.rs` | `tests/oauth2_token_caches.rs`, `tests/oauth2_client_credentials.rs`, `tests/oauth2_resolve_credentials.rs` | Token cache expiry logic, client_credentials resolution (fake clock, no real network) |

No file in `src/` has a matching `#[cfg(test)] mod tests` block — every behavioral test
lives under `tests/` as a black-box integration test against the `openapi_terminal_app`
library crate.

## How the two hardest-to-test layers are tested without a real terminal or network

```text
   ui::* widgets                          app::AppState
   ─────────────                          ──────────────
   ratatui::buffer::Buffer  <-- render()  crossterm::event::KeyEvent
        │                                      │
        ▼                                      ▼
   assert on cell text/style              assert on state after handle_key()

   No real terminal. No snapshot files.   No real terminal. No timing.
```

- **`ui::*` widgets** are pure functions: `(state, spec data) -> ratatui widget`. Tests
  render into an in-memory `ratatui::buffer::Buffer` and assert on cell contents/styles
  directly — see `tests/ui_request_builder.rs` for the density this enables.
- **`app::AppState`** is tested by constructing one directly and driving it with
  synthetic `crossterm::event::KeyEvent`s, then asserting on its public getters — no
  terminal is ever opened.
- **`auth::oauth2`** tests use a fake `Clock` trait implementation instead of real time,
  and a fake `HttpClient` instead of a real network call — see
  `tests/oauth2_resolve_credentials.rs`.

## Writing a new test: the shape to copy

Pick the existing file closest to your change (see the table above) and follow its
pattern. Example shape for an `AppState` behavior test:

```rust
use openapi_terminal_app::app::{AppState, Pane};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn tab_from_endpoint_list_focuses_auth_config() {
    let mut state = AppState::new();
    state.handle_key(KeyEvent::from(KeyCode::Tab));
    assert_eq!(state.focused, Pane::AuthConfig);
}
```

Example shape for a `ui::*` rendering test:

```rust
use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};
use openapi_terminal_app::ui::endpoint_list;

#[test]
fn renders_selected_operation_highlighted() {
    let operations = vec![/* ... */];
    let filtered = endpoint_list::filtered_operations(&operations, "");
    let (list, _selected_visual_row) = endpoint_list::render(&filtered, 0);

    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| f.render_widget(list, f.area())).unwrap();
    let buffer: &Buffer = terminal.backend().buffer();
    // assert on buffer cell contents/styles
}
```

## Manual verification

Required as the *last* step of any UI-visible change (layout, keybindings, rendering) —
not a replacement for the automated tests above, and not something to commit as an
artifact.

1. Build and run against the bundled example spec:
   ```bash
   cargo run -- examples/minimal.yaml
   ```
2. Check the basics:
   - [ ] All five panes render with correct borders/titles, in the expected 3-column
         layout (see the README's [pane layout diagram](../README.md#panes)).
   - [ ] `Tab`/`Shift+Tab` cycle focus through all five panes, both directions, wrapping.
   - [ ] `Ctrl+Alt+1`-`5` jump directly to each pane (if your terminal supports the Kitty
         keyboard protocol — see `ARCHITECTURE.md §7.5`; skip this check if it doesn't).
   - [ ] In Endpoints: `/` filters live, `s` cycles servers (n/a for the single-server
         example spec — check with a multi-server spec if your change touches this).
   - [ ] In Request Builder: `[`/`]` switch sub-tabs; edit a row with `Enter`, commit
         with `Enter`/`Ctrl+S`, cancel with `Esc`.
   - [ ] Paste a multi-line value into the Payload row and confirm it's inserted as one
         chunk (embedded newlines included).
   - [ ] Select `getPet` (needs `petId` — a required path param) and press `Enter`
         without filling it in; confirm Response Viewer shows the missing-parameter
         message and no request is sent.
3. If your change specifically touches something not covered above (scrolling, a new
   pane, a new auth scheme), add a check for it to this list in the same PR.

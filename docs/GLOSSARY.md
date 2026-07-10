# Glossary

Domain and code terms used throughout opit's docs and codebase, alphabetical. Read this
before or alongside `docs/ARCHITECTURE.md` — that doc assumes you already know these.

**AppState** (`src/app/mod.rs`) — The single struct holding all UI state for one running
session: which pane is focused, which operation/server is selected, the endpoint filter
text, and the three sub-editors that make up the Request Builder. No rendering, no HTTP,
no spec parsing — see [`ARCHITECTURE.md §2.4`](ARCHITECTURE.md#24-level-4--code-the-core-state-types).

**Credential** (`src/auth/mod.rs`) — An enum representing a resolved, ready-to-apply
auth value (e.g. a Bearer token) derived from a `SecurityScheme` plus whatever the user
typed into Auth Config. Distinct from the raw string the user types — `auth::apply()`
turns a `Credential` into an actual header/query/cookie mutation on an `HttpRequest`.

**Curl Preview** — The read-only pane showing the `curl`-equivalent of the request that
*would* be sent right now, updated live as you type — before you commit any field with
`Enter`. See `ARCHITECTURE.md §7.3` for why OAuth2 tokens are the one thing it can't
show ahead of time.

**Custom header / custom query parameter** — A header or query parameter the user adds
via the `+ Add` row in Request Builder, as opposed to one declared by the spec. Tracked
separately (`RequestBuilderState::custom_headers` / `custom_query_params`) because it
has no corresponding `Parameter` in the spec. See `ARCHITECTURE.md §3.4` for the exact
control flow of adding one.

**HttpRequest** (`src/request/mod.rs`) — The fully-assembled, ready-to-send request:
method, URL, headers, query string, body. Built by `request::build`/`build_preview` from
an `Operation` plus a `RequestInputs`.

**Operation** (`src/spec/operation.rs`) — opit's own flattened representation of one
`(method, path)` entry from the spec — method, path, tag, summary/description,
parameters, and request body schema. This is what `spec::Spec::operations()` returns;
it's a simplification of `openapiv3`'s much more general types, built specifically for
what the UI needs to render and the request layer needs to send.

**Operation identity** — The `(method, path)` pair used to detect "did the user actually
select a *different* operation," independent of its numeric position in the (possibly
filtered) endpoint list. See `ARCHITECTURE.md §7.2`.

**Pane** (`src/app/mod.rs`, enum) — One of the five focusable regions of the screen:
`EndpointList`, `AuthConfig`, `RequestBuilder`, `CurlPreview`, `ResponseViewer`. `Tab` /
`Shift+Tab` cycle through them; `Ctrl+Alt+1`-`5` jump directly. See the pane layout
diagram in the [README](../README.md#panes).

**PaneEditor** (`src/app/pane_editor.rs`) — A small, reusable, list-of-editable-rows state
machine: which row is selected, which row (if any) is being edited, the in-progress edit
buffer, and the map of committed values. Used identically for Auth Config and for each of
the three Request Builder tabs — it has no idea what a "header" or "credential" is. See
`ARCHITECTURE.md §2.4` for why there are *three* of these inside `RequestBuilderState`
plus a fourth for Auth Config, rather than one shared instance.

**RequestBuilderState** (`src/app/mod.rs`) — Wraps the three `PaneEditor`s (`headers`,
`parameters`, `payload`) that back the Request Builder's sub-tabs, plus the
`custom_headers`/`custom_query_params` name lists.

**RequestBuilderTab** (`src/app/mod.rs`, enum) — Which of the Request Builder's three
sub-tabs is currently active: `Header`, `Parameters`, `Payload`. Switched with `[`/`]`;
sticky across operation switches (see `ARCHITECTURE.md §4.3`).

**RequestInputs** (`src/request/mod.rs`) — The seam between "where a value physically
lives" (which `PaneEditor`, which row index) and "what `request::build` needs" (values
keyed by parameter *name*, plus ad-hoc extras). Produced by
`request::gather_request_inputs`. See `ARCHITECTURE.md §7.4` for why this struct exists
at all.

**Row index convention** — The rule that the Payload editor always has exactly one row
(index 0) and the Header/Parameters editors' last row is always the `+ Add` row at index
`row_count() - 1`. Load-bearing for three different call sites — see
`ARCHITECTURE.md §7.1` before changing how rows are counted anywhere.

**SecurityScheme** / **SecuritySchemeKind** (`src/spec/security.rs`) — opit's
representation of one entry from the spec's `components.securitySchemes`: API Key
(header/query/cookie), HTTP Basic, HTTP Bearer, OAuth2 (with its flow, e.g.
`clientCredentials`), or OpenID Connect. Backs one row in the Auth Config pane — only API
Key, HTTP Basic/Bearer, and OAuth2 `clientCredentials` are currently editable there; the
rest show as "(not editable yet)" (see the README's Known limitations).

**Spec** (`src/spec/mod.rs`) — The top-level parsed representation of the whole OpenAPI
document: title, version, servers, operations, security schemes. Wraps `openapiv3`'s
parse result and exposes only what the rest of opit needs.

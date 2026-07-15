# opit — OpenAPI terminal app

Rust ratatui TUI for browsing OpenAPI specs and building/sending requests.
Binary: `opit` (`src/main.rs`); library crate: `openapi_terminal_app`
(`src/lib.rs`). GitHub repo: `jimytc/opit`.

## Commands

- `cargo run -- <spec.(yaml|json)>` starts the TUI — but it's a full-screen
  app, so **never** try to verify it by piping output. Use the `verify-tui`
  skill to drive it in tmux and capture panes.

## Skills

Project skills live in `.claude/skills/<name>/`, each bundling its script
under `scripts/`.

- `check` — fmt + build + test with terse, failures-only output. Use this
  instead of raw `cargo test` piped through ad-hoc filters.
- `verify-tui` — run/inspect the TUI in tmux (one call per interaction
  instead of manual new-session/send-keys/capture-pane sequences).
- `codex-task` — delegate a single-file task (usually a test file) to Codex
  CLI with the standard flags and "touch only this file" guardrail baked in.
- `release` — version bump + tag + CI watch. Release notes come from the
  annotated tag's message body; read the skill before tagging.

## Conventions

- Conventional commits (`feat:`, `fix:`, `test:`, `docs:`, `chore:`).
- Integration tests live in `tests/`, one file per behavior area.
- Fixture spec for quick manual checks: `examples/minimal.yaml`.
- CI (`ci.yml`) runs build + test on ubuntu and macos for every push to main.

# Contributing to opit

Welcome. This document is the practical companion to
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) (the *why*) and
[`docs/GETTING_STARTED.md`](docs/GETTING_STARTED.md) (your first hour). This one is the
*how*: environment setup, the day-to-day workflow, and what a PR needs before it's
mergeable.

If you haven't yet, read `docs/ARCHITECTURE.md` end to end before your first non-trivial
change — it explains module boundaries and a handful of invariants (§7) that aren't
obvious from the code alone.

## 1. Setup

You need the Rust toolchain (`cargo`, `rustc`) — install via [rustup](https://rustup.rs)
if you don't have one. No other system dependencies are required for `cargo build`/`cargo
test`; the [cross-compiling section of the README](README.md#cross-compiling-for-linux)
covers the extra `cross`/Docker setup needed only for building Linux release binaries on
another OS.

```bash
git clone <your fork or the repo>
cd openapi-terminal-app
cargo build            # compiles the opit binary + library
cargo test              # runs the full integration test suite
cargo run -- examples/minimal.yaml   # launches the TUI against a bundled sample spec
```

See `docs/GETTING_STARTED.md` for a guided first run.

## 2. Day-to-day workflow

```text
   ┌─────────┐     ┌─────────┐     ┌──────────┐     ┌────────┐
   │  branch │ --> │  RED    │ --> │  GREEN   │ --> │   PR   │
   │  off    │     │  commit │     │  commit  │     │        │
   │  main   │     │ (tests) │     │  (src/)  │     │        │
   └─────────┘     └─────────┘     └──────────┘     └────────┘
```

opit is built under **strict TDD**, enforced by convention (not tooling): a commit that
adds a failing test never also touches `src/`, and a commit that makes it pass never also
touches `tests/`. Run `git log --oneline` to see what this looks like in practice — it
reads as an alternating `test: ...` / `feat: ...` / `fix: ...` sequence.

1. **Write the test first**, under `tests/` (integration-style — this repo has no
   `#[cfg(test)]` unit tests inside `src/`; see `docs/TESTING.md` for which file a given
   change belongs in). Confirm it fails for the reason you expect:
   `cargo test --test <file_stem>`.
2. **Commit the failing test on its own** (`test: ...`).
3. **Write only the `src/` change needed to turn it green.** Resist the urge to also
   refactor or add unrelated tests in this step.
4. **Commit the fix on its own** (`feat: ...` / `fix: ...`).
5. Repeat until the change is complete.

**A note on Codex.** This project's maintainers sometimes delegate the RED step (writing
the test itself) to Codex CLI (`codex exec`), given a precise behavioral spec, then review
the resulting test for correctness/scope before treating it as ground truth and writing
the GREEN step by hand. **This is a maintainer workflow choice, not a project
requirement** — you're free to write your own tests directly. What *is* required
regardless of how the test was authored: the red/green split above, and that the test
actually fails before the fix and passes after.

## 3. Before opening a PR

Run these locally — CI (`.github/workflows/ci.yml`) currently runs `cargo build` and
`cargo test` on Ubuntu and macOS, plus a separate cross-compile job that builds (but
doesn't test) the Linux release targets via `cross`, so a change that breaks
cross-compilation can fail CI even if `cargo test` passes locally. Keeping the tree
`fmt`/`clippy`-clean is expected project hygiene even though it isn't yet a CI gate:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

For anything that changes rendered output or keyboard handling, also do a manual
terminal check — see `docs/TESTING.md#manual-verification` for the checklist. It's the
last step, not a replacement for the automated tests above.

## 4. PR checklist

- [ ] Commits follow the red/green split described in §2 (or the change is docs-only /
      genuinely untestable, in which case say so in the PR description).
- [ ] `cargo test` passes locally.
- [ ] `cargo fmt` and `cargo clippy --all-targets -- -D warnings` are clean.
- [ ] If the change alters a module boundary, a data flow, or an invariant, update
      `docs/ARCHITECTURE.md` in the same PR — see that doc's opening line: "an
      architecture doc that drifts from the code is worse than none."
- [ ] If the change is UI-visible, you've manually exercised it in a terminal (see
      `docs/TESTING.md#manual-verification`).
- [ ] PR description explains *why*, not just *what* — link an issue if one exists.

## 5. Commit messages

Prefix with a conventional-commit-style tag matching what actually happened:
`test:`, `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`. Keep the subject line short
and imperative ("add X", not "added X" or "adds X").

## 6. Where to start

New to the codebase? See `docs/GETTING_STARTED.md` for a guided first change, and
`docs/ARCHITECTURE.md#8-where-to-add-things` for a map of "I want to add a Y" →
"touch these files."

## 7. Releasing

Releasing (tagging, Homebrew tap updates) is a maintainer task — see the
[README's Releasing section](README.md#releasing) if you need the mechanics, but you
don't need this to contribute a PR.

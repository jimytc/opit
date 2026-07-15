---
name: check
description: Run this repo's standard fmt + build + test check with terse, failures-only output. Use whenever verifying the codebase is healthy — "run the tests", "do the checks pass", before any commit, after finishing a change, and as the preflight step of a release. Always prefer this over raw `cargo test`/`cargo build` piped through ad-hoc grep/tail filters.
---

# Checking the opit codebase

Run:

```bash
.claude/skills/check/scripts/check.sh
```

It runs, in order, from the repo root:

1. `cargo fmt -- --check` — reports files needing formatting (fix: `cargo fmt`)
2. `cargo build` — fails fast on errors, surfaces warnings
3. `cargo test` — on failure prints only the `failures:` section and summary

Output is one status line per stage (`FMT:`/`BUILD:`/`TEST:`) plus details
only for problems, so the full cargo logs stay out of context. Exit code is
nonzero if any stage fails.

There is no clippy stage because CI (`.github/workflows/ci.yml`) only runs
build + test; matching CI keeps a green check.sh meaning "CI will pass".

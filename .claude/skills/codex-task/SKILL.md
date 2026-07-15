---
name: codex-task
description: Delegate a self-contained single-file task (usually writing an integration test file in tests/) to OpenAI Codex CLI with this repo's standard flags and single-file guardrail baked in. Use whenever offloading work to Codex — "have codex write the tests", "delegate this to codex", "farm this out" — instead of composing a raw `codex exec` command by hand.
---

# Delegating single-file tasks to Codex

Run:

```bash
.claude/skills/codex-task/scripts/codex-task.sh tests/foo.rs "Write integration tests covering ..."
```

The prompt can also come from stdin (`< prompt.txt`), which is better for
long prompts with context excerpts.

The script bakes in the flags every past delegation used — `codex exec
--skip-git-repo-check --sandbox workspace-write --full-auto -m gpt-5-codex
--config model_reasoning_effort=medium -C <repo-root>` — and prepends a
guardrail telling Codex to write ONLY the target file and touch nothing
else. Options:

- `-m MODEL` — different model (e.g. `gpt-5`)
- `-r EFFORT` — reasoning effort (default `medium`)
- `-W` — drop the single-file guardrail for multi-file tasks (use sparingly;
  the guardrail is what makes delegations safe to run unattended)

## Writing good delegation prompts

Codex starts with zero context about this crate. Prompts that worked well in
past sessions included: the crate/module layout relevant to the task, the
public API signatures under test (paste them), existing test conventions to
imitate, and an explicit "when done, run `cargo test <file-stem>` and fix
failures" instruction.

After the run, always review the diff (`git diff --stat` then the file) and
run the `check` skill before committing — Codex output is a draft, not a
merge.

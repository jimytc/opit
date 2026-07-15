#!/usr/bin/env bash
# Delegate a single-file task to Codex CLI with this repo's standard flags
# and single-file guardrail baked in.
#
# Usage:
#   scripts/codex-task.sh tests/spec_parsing.rs "Write integration tests covering ..."
#   scripts/codex-task.sh tests/foo.rs < prompt.txt      # prompt from stdin
#
# Options:
#   -m MODEL   codex model (default gpt-5-codex)
#   -r EFFORT  model_reasoning_effort (default medium)
#   -W         drop the single-file guardrail (allow writes anywhere in the repo)
set -euo pipefail

MODEL="gpt-5-codex" EFFORT="medium" GUARD=1
while getopts "m:r:Wh" opt; do
  case $opt in
    m) MODEL=$OPTARG ;;
    r) EFFORT=$OPTARG ;;
    W) GUARD=0 ;;
    h) sed -n '2,13p' "$0"; exit 0 ;;
    *) exit 1 ;;
  esac
done
shift $((OPTIND - 1))

TARGET="${1:?usage: codex-task.sh [-m model] [-r effort] [-W] <target-file> [prompt]}"
shift
PROMPT="${*:-$(cat)}"

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"

if [[ $GUARD -eq 1 ]]; then
  PROMPT="Write ONLY the file at $TARGET in this repo. Do NOT create, modify, or touch any other file — not src/, not Cargo.toml, nothing except $TARGET.

$PROMPT"
fi

exec codex exec --skip-git-repo-check --sandbox workspace-write --full-auto \
  -m "$MODEL" --config model_reasoning_effort="$EFFORT" -C "$ROOT" "$PROMPT"

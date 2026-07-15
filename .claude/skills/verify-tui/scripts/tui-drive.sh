#!/usr/bin/env bash
# Drive the opit TUI in a throwaway tmux session and print the captured pane.
#
# One-shot (start app, send keys, print pane, kill session):
#   scripts/tui-drive.sh examples/minimal.yaml Tab Down Down Enter
#
# Multi-step (keep the session alive between calls):
#   scripts/tui-drive.sh -k examples/minimal.yaml Tab      # prints pane, keeps session
#   scripts/tui-drive.sh -t <session> Down Enter           # send more keys to it
#   scripts/tui-drive.sh -t <session> -K                   # capture then kill
#
# Keys use tmux send-keys names: Tab, Down, Up, Enter, Escape, C-s, C-M-2,
# or literal characters like "[" "]" "g". Each key is sent as its own
# send-keys call with a short delay so the app can redraw between inputs.
#
# Options:
#   -x COLS     pane width  (default 130)
#   -y ROWS     pane height (default 30)
#   -d SECS     delay between keys (default 0.2)
#   -s SECS     startup wait before first key (default 0.5)
#   -e          include ANSI escapes in the capture (for color assertions)
#   -k          keep the session alive after capturing; prints its name to stderr
#   -t NAME     reuse an existing session instead of starting the app
#   -K          kill the session after capturing (with -t)
set -euo pipefail

COLS=130 ROWS=30 KEY_DELAY=0.2 STARTUP=0.5 ANSI="" KEEP=0 KILL=0 SESSION=""
while getopts "x:y:d:s:ekKt:h" opt; do
  case $opt in
    x) COLS=$OPTARG ;;
    y) ROWS=$OPTARG ;;
    d) KEY_DELAY=$OPTARG ;;
    s) STARTUP=$OPTARG ;;
    e) ANSI="-e" ;;
    k) KEEP=1 ;;
    K) KILL=1 ;;
    t) SESSION=$OPTARG ;;
    h) sed -n '2,25p' "$0"; exit 0 ;;
    *) exit 1 ;;
  esac
done
shift $((OPTIND - 1))

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
BIN="$ROOT/target/debug/opit"

if [[ -z "$SESSION" ]]; then
  if [[ $# -lt 1 ]]; then
    echo "usage: tui-drive.sh [options] <spec-file> [keys...]  (or -t <session> [keys...])" >&2
    exit 1
  fi
  SPEC="$1"; shift
  if [[ ! -x "$BIN" || -n "$(find "$ROOT/src" -name '*.rs' -newer "$BIN" 2>/dev/null | head -1)" ]]; then
    (cd "$ROOT" && cargo build 2>&1 | tail -3) >&2
  fi
  SESSION="opitdrive-$$-$RANDOM"
  tmux new-session -d -s "$SESSION" -x "$COLS" -y "$ROWS" \
    "$(printf '%q %q' "$BIN" "$SPEC")"
  sleep "$STARTUP"
fi

for key in "$@"; do
  tmux send-keys -t "$SESSION" "$key"
  sleep "$KEY_DELAY"
done

tmux capture-pane -t "$SESSION" -p $ANSI

# Sessions we started die unless -k; sessions we reused (-t) live unless -K.
if [[ -n "${SPEC:-}" && $KEEP -eq 0 ]] || [[ $KILL -eq 1 ]]; then
  tmux kill-session -t "$SESSION" 2>/dev/null || true
elif [[ $KEEP -eq 1 || -z "${SPEC:-}" ]]; then
  echo "session kept: $SESSION  (continue: tui-drive.sh -t $SESSION <keys...>; kill: -t $SESSION -K)" >&2
fi

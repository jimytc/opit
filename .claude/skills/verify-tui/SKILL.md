---
name: verify-tui
description: Drive and visually verify the opit TUI (a ratatui app — it cannot be checked by piping output; it needs a real terminal). Use this whenever you need to run the app, see a change working on screen, verify UI behavior or layout, test keyboard interactions, take a pane snapshot, or reproduce a UI bug — even if the request is just "check that it works" or "run the app". Always prefer .claude/skills/verify-tui/scripts/tui-drive.sh over hand-rolling tmux new-session/send-keys/capture-pane sequences.
---

# Verifying the opit TUI

opit is a full-screen ratatui app, so the only way to observe it is inside a
real terminal. `.claude/skills/verify-tui/scripts/tui-drive.sh` wraps the whole tmux dance (new-session,
send-keys with redraw delays, capture-pane, kill-session) into one call —
use it instead of issuing tmux commands one keystroke at a time.

The script rebuilds `target/debug/opit` automatically when sources are newer
than the binary.

## One-shot check

```bash
.claude/skills/verify-tui/scripts/tui-drive.sh examples/minimal.yaml Tab Down Down Enter
```

Starts the app on the spec, sends the keys (0.2s apart), prints the final
pane, and kills the session. Default pane is 130x30; use `-x`/`-y` for other
sizes (100x20 is a good "small terminal" regression check).

## Multi-step interactions

When you need to inspect the screen between inputs, keep the session alive:

```bash
.claude/skills/verify-tui/scripts/tui-drive.sh -k examples/minimal.yaml Tab        # prints pane, keeps session (name on stderr)
.claude/skills/verify-tui/scripts/tui-drive.sh -t opitdrive-XXXX Down Enter        # send more keys, print pane again
.claude/skills/verify-tui/scripts/tui-drive.sh -t opitdrive-XXXX -K                # final capture + kill
```

## Keys

Keys are tmux `send-keys` names: `Tab`, `Down`, `Up`, `Enter`, `Escape`,
`C-s` (Ctrl+S), `C-M-2` (Ctrl+Alt+2), or literal characters quoted as needed
(`"["`, `"]"`, `"g"`). Typing into an input field: pass the string as one
argument, e.g. `"petId=42"`.

## Asserting on colors (focus, highlights)

Pass `-e` to keep ANSI escapes in the capture. The focused pane's border is
green (256-color 2), so the standard focus assertion is:

```bash
.claude/skills/verify-tui/scripts/tui-drive.sh -e examples/minimal.yaml Tab | grep -F $'\x1b[38;5;2m\xe2\x94\x8c'
```

(that is: escape sequence `[38;5;2m` immediately followed by `┌`). Exit code
tells you whether the expected pane holds focus.

## Fixture specs

- `examples/minimal.yaml` — small spec checked into the repo; default choice.
- `~/Downloads/als-oms-openapi.yaml` — large real-world spec used in past
  sessions for scrolling/filtering checks (may not exist on every machine).
- For targeted layouts, generate a purpose-built JSON spec into the scratchpad
  directory rather than editing fixtures in the repo.

## Tips

- Give the app extra startup time for large specs: `-s 1`.
- If output looks torn mid-redraw, re-capture: `.claude/skills/verify-tui/scripts/tui-drive.sh -t <session>`
  with no keys just captures again.
- Clean up stray sessions with `tmux ls` / `tmux kill-session -t <name>`
  (script-created sessions are named `opitdrive-*`).

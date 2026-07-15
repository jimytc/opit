---
name: release
description: Cut a release of opit — version bump, tag, CI watch, release verification. Use whenever the user asks to release, publish, ship, cut/tag a version, or bump the version, and also when verifying that a release finished. Encodes the repo's tag-message-driven release notes, so don't improvise a release flow without it.
---

# Releasing opit

Repo: `jimytc/opit`. Releases are driven by pushing an annotated tag
`vX.Y.Z` — the Release workflow (`.github/workflows/release.yml`) builds a
macOS aarch64 tarball and creates the GitHub release.

**Release notes come from the annotated tag's message body.** The workflow
takes the tag message (minus the first line) as the curated section and
appends GitHub's auto-generated notes below it. So write a real changelog
into the tag message — that IS the release description. Do not use
`gh release create` manually and do not rely on `--generate-notes`.

## Flow

1. **Preflight** — working tree clean, on `main`, checks pass:
   ```bash
   git status && .claude/skills/check/scripts/check.sh
   ```

2. **Bump version** in `Cargo.toml` (`version = "X.Y.Z"`), then
   `cargo build` so `Cargo.lock` picks it up. Commit both:
   ```
   chore: bump version to X.Y.Z
   ```

3. **Push main and wait for CI** (single blocking call — don't poll
   `gh run view` repeatedly):
   ```bash
   git push origin main
   gh run list --repo jimytc/opit --branch main --limit 1   # grab the run id
   gh run watch <run-id> --repo jimytc/opit --exit-status 2>&1 | tail -20
   ```

4. **Tag with a changelog body** (first line = title, rest = curated
   release notes):
   ```bash
   git tag -a vX.Y.Z -m "$(cat <<'EOF'
   vX.Y.Z: one-line summary

   ## Highlights
   - user-facing change 1
   - user-facing change 2
   EOF
   )"
   git push origin vX.Y.Z
   ```

5. **Watch the release run**, then verify the release exists and has the
   tarball asset and the curated notes:
   ```bash
   gh run list --repo jimytc/opit --limit 3        # find the Release run id
   gh run watch <run-id> --repo jimytc/opit --exit-status 2>&1 | tail -20
   gh release view vX.Y.Z --repo jimytc/opit
   ```

## If something goes wrong

- Release workflow failed after the tag was pushed: fix on main, then move
  the tag (`git tag -fa vX.Y.Z -m "..."`, `git push -f origin vX.Y.Z`) only
  if the release was never published; otherwise bump to a new patch version.
- To inspect a past tag's message: `git tag -l --format='%(contents)' vX.Y.Z`.

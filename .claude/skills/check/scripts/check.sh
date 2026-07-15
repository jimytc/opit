#!/usr/bin/env bash
# Terse pre-commit check: fmt + build + test, printing only problems.
# Exits nonzero if anything fails. Full logs stay out of the way unless needed.
set -uo pipefail
cd "$(dirname "$0")/../../../.."
fail=0

if cargo fmt -- --check >/dev/null 2>&1; then
  echo "FMT:   ok"
else
  echo "FMT:   needs formatting — run: cargo fmt"
  cargo fmt -- --check 2>&1 | head -20
  fail=1
fi

build_out="$(cargo build 2>&1)"
if [[ $? -ne 0 ]]; then
  echo "BUILD: FAILED"
  echo "$build_out" | tail -40
  exit 1
fi
if echo "$build_out" | grep -qi '^warning'; then
  echo "BUILD: ok, with warnings:"
  echo "$build_out" | grep -i -A2 '^warning' | head -30
else
  echo "BUILD: ok"
fi

test_out="$(cargo test 2>&1)"
if [[ $? -ne 0 ]]; then
  echo "TEST:  FAILED"
  echo "$test_out" | sed -n '/^failures:/,$p' | head -60
  echo "$test_out" | grep 'test result:' | grep -v '0 failed' || true
  fail=1
else
  echo "TEST:  ok ($(echo "$test_out" | grep -c 'test result: ok') suites)"
fi

exit $fail

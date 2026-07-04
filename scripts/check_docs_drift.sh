#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "Running docs drift guard..."

# Patterns that should not appear in current docs/API examples.
declare -a forbidden_patterns=(
  'to_json_english\('
  '\bbo4e::'
  'bo4e::v202501::bo::'
  'bo4e::v202401::bo::'
  '\.upgrade\('
  'schemars = "0\\.8"'
  '\bbo4e = \{ version = '
)

for pattern in "${forbidden_patterns[@]}"; do
  set +e
  rg -n -e "$pattern" docs src/lib.rs
  rc=$?
  set -e

  if [[ $rc -eq 0 ]]; then
    echo
    echo "Docs drift guard failed: found forbidden pattern '$pattern'"
    exit 1
  elif [[ $rc -gt 1 ]]; then
    echo
    echo "Docs drift guard failed: invalid pattern '$pattern'"
    exit 1
  fi
done

# Guard: library source must not use `crate::current::` — `current` is a
# user-facing moving alias and must not be depended upon inside the crate
# itself (it would create a version-skew self-reference on re-pointing).
echo "Checking for forbidden crate::current:: usage in src/..."
set +e
rg -n 'crate::current::' src/
rc=$?
set -e
if [[ $rc -eq 0 ]]; then
  echo
  echo "Docs drift guard failed: found 'crate::current::' in src/ — use explicit versioned paths instead."
  exit 1
elif [[ $rc -gt 1 ]]; then
  echo
  echo "Docs drift guard failed: rg error while checking crate::current:: (rc=$rc)"
  exit 1
fi

echo "Docs drift guard passed."

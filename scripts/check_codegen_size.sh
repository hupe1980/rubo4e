#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Guardrail defaults are intentionally above current baseline to allow normal
# schema evolution while still catching accidental generation explosions.
MAX_GENERATED_RS_FILES="${MAX_GENERATED_RS_FILES:-430}"
MAX_GENERATED_RS_LINES="${MAX_GENERATED_RS_LINES:-62000}"

count_files() {
  find src/generated/v202401 src/generated/v202501 -type f -name '*.rs' | wc -l | tr -d ' '
}

count_lines() {
  find src/generated/v202401 src/generated/v202501 -type f -name '*.rs' -print0 \
    | xargs -0 wc -l \
    | tail -n 1 \
    | awk '{print $1}'
}

files="$(count_files)"
lines="$(count_lines)"

echo "Generated Rust file count: $files (limit: $MAX_GENERATED_RS_FILES)"
echo "Generated Rust line count: $lines (limit: $MAX_GENERATED_RS_LINES)"

if ! awk -v n="$files" -v max="$MAX_GENERATED_RS_FILES" 'BEGIN { exit !(n <= max) }'; then
  echo "Codegen size guard failed: generated file count $files exceeds limit $MAX_GENERATED_RS_FILES"
  exit 1
fi

if ! awk -v n="$lines" -v max="$MAX_GENERATED_RS_LINES" 'BEGIN { exit !(n <= max) }'; then
  echo "Codegen size guard failed: generated line count $lines exceeds limit $MAX_GENERATED_RS_LINES"
  exit 1
fi

echo "Codegen size guard passed."

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MAX_SLOWDOWN="${MAX_SLOWDOWN:-1.80}"
MAX_SLOWDOWN_GERMAN_STR="${MAX_SLOWDOWN_GERMAN_STR:-1.20}"
MAX_SLOWDOWN_GERMAN_BYTES="${MAX_SLOWDOWN_GERMAN_BYTES:-1.20}"
MAX_SLOWDOWN_SNAKE_STR="${MAX_SLOWDOWN_SNAKE_STR:-1.20}"
MAX_SLOWDOWN_SNAKE_BYTES="${MAX_SLOWDOWN_SNAKE_BYTES:-1.20}"
MAX_SLOWDOWN_ADVERSARIAL_GERMAN_STR="${MAX_SLOWDOWN_ADVERSARIAL_GERMAN_STR:-1.10}"
MAX_SLOWDOWN_ADVERSARIAL_GERMAN_BYTES="${MAX_SLOWDOWN_ADVERSARIAL_GERMAN_BYTES:-1.10}"
MAX_SLOWDOWN_ADVERSARIAL_SNAKE_STR="${MAX_SLOWDOWN_ADVERSARIAL_SNAKE_STR:-1.10}"
MAX_SLOWDOWN_ADVERSARIAL_SNAKE_BYTES="${MAX_SLOWDOWN_ADVERSARIAL_SNAKE_BYTES:-1.10}"
MAX_SLOWDOWN_FILEBACKED_GERMAN_STR="${MAX_SLOWDOWN_FILEBACKED_GERMAN_STR:-1.15}"
MAX_SLOWDOWN_FILEBACKED_GERMAN_BYTES="${MAX_SLOWDOWN_FILEBACKED_GERMAN_BYTES:-1.15}"
MAX_SLOWDOWN_FILEBACKED_SNAKE_STR="${MAX_SLOWDOWN_FILEBACKED_SNAKE_STR:-1.15}"
MAX_SLOWDOWN_FILEBACKED_SNAKE_BYTES="${MAX_SLOWDOWN_FILEBACKED_SNAKE_BYTES:-1.15}"
MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_STR="${MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_STR:-1.15}"
MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_BYTES="${MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_BYTES:-1.15}"
MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_STR="${MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_STR:-1.15}"
MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_BYTES="${MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_BYTES:-1.20}"

BASE_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$BASE_DIR"
}
trap cleanup EXIT

echo "Running baseline quick benchmark (versioned,json)..."
cargo bench --bench json_perf --features versioned,json -- --quick

copy_baseline_estimate() {
  local bench="$1"
  local src="target/criterion/${bench}/new/estimates.json"
  local dst="$BASE_DIR/${bench}.json"

  mkdir -p "$(dirname "$dst")"
  if [[ ! -f "$src" ]]; then
    echo "Missing baseline estimate file: $src"
    exit 1
  fi
  cp "$src" "$dst"
}

# Focus on medium/typical workloads and cover both german/snake and str/bytes paths.
declare -a benches=(
  "deserialize_from_json_german/str/marktlokation_typical"
  "deserialize_from_json_german/str/vertrag_typical"
  "deserialize_from_json_german/bytes/marktlokation_typical"
  "deserialize_from_json_german/bytes/vertrag_typical"
  "deserialize_from_json_snake_case/str/marktlokation_typical"
  "deserialize_from_json_snake_case/str/vertrag_typical"
  "deserialize_from_json_snake_case/bytes/marktlokation_typical"
  "deserialize_from_json_snake_case/bytes/vertrag_typical"
  "deserialize_from_json_german/str/marktlokation_filebacked"
  "deserialize_from_json_german/str/vertrag_filebacked"
  "deserialize_from_json_german/bytes/marktlokation_filebacked"
  "deserialize_from_json_german/bytes/vertrag_filebacked"
  "deserialize_from_json_german/str/marktlokation_filebacked_large"
  "deserialize_from_json_german/str/vertrag_filebacked_large"
  "deserialize_from_json_german/bytes/marktlokation_filebacked_large"
  "deserialize_from_json_german/bytes/vertrag_filebacked_large"
  "deserialize_from_json_snake_case/str/marktlokation_filebacked"
  "deserialize_from_json_snake_case/str/vertrag_filebacked"
  "deserialize_from_json_snake_case/bytes/marktlokation_filebacked"
  "deserialize_from_json_snake_case/bytes/vertrag_filebacked"
  "deserialize_from_json_snake_case/str/marktlokation_filebacked_large"
  "deserialize_from_json_snake_case/str/vertrag_filebacked_large"
  "deserialize_from_json_snake_case/bytes/marktlokation_filebacked_large"
  "deserialize_from_json_snake_case/bytes/vertrag_filebacked_large"
  "deserialize_from_json_german/str/marktlokation_adversarial"
  "deserialize_from_json_german/str/vertrag_adversarial"
  "deserialize_from_json_german/bytes/marktlokation_adversarial"
  "deserialize_from_json_german/bytes/vertrag_adversarial"
  "deserialize_from_json_snake_case/str/marktlokation_adversarial"
  "deserialize_from_json_snake_case/str/vertrag_adversarial"
  "deserialize_from_json_snake_case/bytes/marktlokation_adversarial"
  "deserialize_from_json_snake_case/bytes/vertrag_adversarial"
)

for bench in "${benches[@]}"; do
  copy_baseline_estimate "$bench"
done

echo "Running simd quick benchmark (versioned,json,simd-json)..."
cargo bench --bench json_perf --features versioned,json,simd-json -- --quick

extract_point_estimate() {
  local json_file="$1"
  perl -ne 'if (/"median":\{"confidence_interval":\{[^}]*\},"point_estimate":([0-9.eE+-]+)/) { print "$1\n"; exit }' "$json_file"
}

assert_ratio() {
  local bench="$1"
  local base_ns="$2"
  local simd_ns="$3"
  local max_allowed="$4"

  if [[ -z "$base_ns" || -z "$simd_ns" ]]; then
    echo "Could not parse benchmark '$bench'"
    exit 1
  fi

  local ratio
  if ! awk -v b="$base_ns" 'BEGIN { exit !(b > 0) }'; then
    echo "Invalid baseline for '$bench': $base_ns"
    exit 1
  fi

  ratio="$(awk -v s="$simd_ns" -v b="$base_ns" 'BEGIN { printf "%.4f", s / b }')"

  echo "$bench"
  echo "  baseline: ${base_ns}ns"
  echo "  simd:     ${simd_ns}ns"
  echo "  ratio:    ${ratio}x"
  echo "  limit:    ${max_allowed}x"

  if ! awk -v r="$ratio" -v max="$max_allowed" 'BEGIN { exit !(r <= max) }'; then
    echo "Perf regression gate failed for '$bench': ${ratio}x > ${max_allowed}x"
    exit 1
  fi
}

threshold_for_bench() {
  local bench="$1"
  case "$bench" in
    deserialize_from_json_german/str/*filebacked_large*)
      echo "$MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_STR"
      ;;
    deserialize_from_json_german/bytes/*filebacked_large*)
      echo "$MAX_SLOWDOWN_FILEBACKED_LARGE_GERMAN_BYTES"
      ;;
    deserialize_from_json_snake_case/str/*filebacked_large*)
      echo "$MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_STR"
      ;;
    deserialize_from_json_snake_case/bytes/*filebacked_large*)
      echo "$MAX_SLOWDOWN_FILEBACKED_LARGE_SNAKE_BYTES"
      ;;
    deserialize_from_json_german/str/*filebacked*)
      echo "$MAX_SLOWDOWN_FILEBACKED_GERMAN_STR"
      ;;
    deserialize_from_json_german/bytes/*filebacked*)
      echo "$MAX_SLOWDOWN_FILEBACKED_GERMAN_BYTES"
      ;;
    deserialize_from_json_snake_case/str/*filebacked*)
      echo "$MAX_SLOWDOWN_FILEBACKED_SNAKE_STR"
      ;;
    deserialize_from_json_snake_case/bytes/*filebacked*)
      echo "$MAX_SLOWDOWN_FILEBACKED_SNAKE_BYTES"
      ;;
    deserialize_from_json_german/str/*adversarial*)
      echo "$MAX_SLOWDOWN_ADVERSARIAL_GERMAN_STR"
      ;;
    deserialize_from_json_german/bytes/*adversarial*)
      echo "$MAX_SLOWDOWN_ADVERSARIAL_GERMAN_BYTES"
      ;;
    deserialize_from_json_snake_case/str/*adversarial*)
      echo "$MAX_SLOWDOWN_ADVERSARIAL_SNAKE_STR"
      ;;
    deserialize_from_json_snake_case/bytes/*adversarial*)
      echo "$MAX_SLOWDOWN_ADVERSARIAL_SNAKE_BYTES"
      ;;
    deserialize_from_json_german/str/*)
      echo "$MAX_SLOWDOWN_GERMAN_STR"
      ;;
    deserialize_from_json_german/bytes/*)
      echo "$MAX_SLOWDOWN_GERMAN_BYTES"
      ;;
    deserialize_from_json_snake_case/str/*)
      echo "$MAX_SLOWDOWN_SNAKE_STR"
      ;;
    deserialize_from_json_snake_case/bytes/*)
      echo "$MAX_SLOWDOWN_SNAKE_BYTES"
      ;;
    *)
      echo "$MAX_SLOWDOWN"
      ;;
  esac
}

echo "Comparing selected benchmark medians with mode-specific slowdown limits..."
for bench in "${benches[@]}"; do
  baseline_file="$BASE_DIR/${bench}.json"
  simd_file="target/criterion/${bench}/new/estimates.json"

  if [[ ! -f "$simd_file" ]]; then
    echo "Missing simd estimate file: $simd_file"
    exit 1
  fi

  base_ns="$(extract_point_estimate "$baseline_file")"
  simd_ns="$(extract_point_estimate "$simd_file")"
  max_allowed="$(threshold_for_bench "$bench")"
  assert_ratio "$bench" "$base_ns" "$simd_ns" "$max_allowed"
done

echo "JSON perf regression gate passed."

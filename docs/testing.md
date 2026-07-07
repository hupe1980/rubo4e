# Testing Strategy

`rubo4e` uses five distinct testing layers. Each layer has its own purpose, test corpus,
and command to run.

---

## Test Layer Summary

| Layer | Purpose | Feature flags | Location | Approx. runtime |
|-------|---------|---------------|----------|-----------------|
| 1. Golden | Wire compatibility | `json`, `versioned` | `tests/golden/` | < 5 s |
| 2. Snapshot | Serialization stability | `schemars` | `tests/schemars_snapshots/` | < 5 s |
| 3. Property | Identifier invariants | (dev dep only) | `tests/proptest_roundtrips.rs` | 30–60 s |
| 4. Fuzz | Panic safety | nightly + `json` | `fuzz/` | minutes (CI: 1M runs) |
| 5. Compat | Cross-impl interop | `json`, `versioned` | `tests/compat/` | < 10 s |

---

## Layer 1 — Golden Schema Tests

Deserialize official BO4E JSON payloads and re-serialize; compare field values.

**Run:**
```bash
cargo test --features json,versioned --test golden
```

**Corpus location:** `tests/golden/`

```
tests/golden/
├── vertrag_minimal.json        # only _typ + _version
├── vertrag_typical.json        # common fields populated
├── marktlokation_minimal.json
├── marktlokation_typical.json
├── messlokation_minimal.json
├── messlokation_typical.json
├── netzlokation_minimal.json
├── netzlokation_typical.json
├── rechnung_minimal.json
└── rechnung_typical.json
```

Files are **not** nested in a version subdirectory — all live directly under
`tests/golden/`. The schema version is encoded in each file’s `"_version"` field.

**What the test checks:**
- Deserialization does not return an error
- Re-serialized output with `to_json_german()` deserializes back to a value equal
  to the original (field values identical; key ordering not required to match)
- Unknown fields in the payload are preserved in `_additional` and survive the round-trip

---

## Layer 2 — Snapshot Serialization Tests

Verify that canonical and German serialization output does not change unexpectedly.
Uses `insta` for snapshot management.

**Run:**
```bash
cargo test --features schemars --test schemars_snapshots
```

**Update snapshots** after intentional changes:
```bash
cargo insta review
```

Snapshots are committed to the repository. A changed snapshot in CI is a CI failure
that requires explicit review and acceptance with `cargo insta accept`.

---

## Layer 3 — Property-Based Tests

Verify identifier round-trip invariants, serde correctness for date types, and
enum `Display`/`FromStr` for all generated variants.

**Run:**
```bash
cargo test --all-features --test proptest_roundtrips
```

> **Note:** there is no `testing` feature flag. `proptest` is a plain dev-dependency.
> `Arbitrary` impls for identifier types are compiled `#[cfg(test)]` only and
> are not available to external crates.

**Properties covered:**

```rust
// Identifier: Display ↔ FromStr round-trip
proptest! {
    fn malo_id_display_from_str_roundtrip(s in valid_11digit()) {
        let id = MaloId::new(&s).unwrap();
        prop_assert_eq!(id.to_string().parse::<MaloId>().unwrap(), id);
    }
}

// Serde round-trip for required time::Date
proptest! {
    fn required_date_serde_roundtrip(date in any_date()) {
        // serializes as "YYYY-MM-DD", deserializes back to the same Date
    }
}

// Enum: Display ↔ FromStr round-trip over all known variants (strum)
proptest! {
    fn sparte_display_from_str_roundtrip(variant in any_sparte()) { … }
}
```

Also covered: `opt_date_serde` `None`/`Some` round-trips, JSON null → `None` deserialization,
ISO 8601 wire-format assertion (`"YYYY-MM-DD"`).

---

## Layer 4 — Fuzz Testing

Feed arbitrary bytes to the deserialization path and verify no panics occur.
Requires nightly Rust.

**Setup:**
```bash
cargo install cargo-fuzz
```

**Run (CI — 1 million iterations):**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag -- -runs=1000000
```

**Run (continuous — local development):**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag
```

**Targets:**
```
fuzz/fuzz_targets/
└── fuzz_deserialize_vertrag.rs   — serde_json::from_slice::<Vertrag>(...)
```

**What constitutes a fuzz failure:**
- Any panic (including `unwrap`, `expect`, index out of bounds)
- Stack overflow
- Memory safety violation

An `Err` return from `from_slice` is **not** a failure — malformed input is expected
to return an error, not panic.

**Reproducing a crash:**
```bash
cargo +nightly fuzz run fuzz_deserialize_vertrag fuzz/artifacts/fuzz_deserialize_vertrag/<id>
```

---

## Layer 5 — Cross-Implementation Compatibility

Verify that `rubo4e` correctly deserializes payloads produced by the Python and Go
reference implementations.

**Run:**
```bash
cargo test --features json,versioned --test compat
```

**Corpus location:**
```
tests/compat/
├── README.md           — how to regenerate vectors
├── python/
│   ├── Marktlokation.json
│   ├── Messlokation.json
│   ├── Vertrag.json
│   └── Rechnung.json
└── go/
    ├── Marktlokation.json
    ├── Messlokation.json
    ├── Vertrag.json
    └── Rechnung.json
```

**What the test checks:**
- Deserialization does not error
- Specific field values are asserted (not just "no error") — at least 3 fields per payload

**Regenerating vectors:**
See `tests/compat/README.md` for instructions on how to regenerate when either reference
implementation releases a new version.

---

## CI Safety Notes

When piping test output through `tee` in CI scripts, enable `set -o pipefail`
(or check `PIPESTATUS`) to prevent a failing test command from appearing to succeed:

```bash
set -o pipefail
cargo test --features json,versioned --test golden 2>&1 | tee test-output.log
```

Without `pipefail`, a non-zero exit from `cargo test` is masked by `tee`'s success.

---

## Running the Full Suite

```bash
# All unit and integration tests (default features)
cargo test --workspace

# All tests with all features
cargo test --workspace --all-features

# Just golden corpus tests
cargo test --features json,versioned --test golden

# Identifier + serde + enum property tests (no extra feature flag needed)
cargo test --all-features --test proptest_roundtrips

# Cross-impl compatibility
cargo test --features json,versioned --test compat

# schemars snapshot tests
cargo test --features schemars --test schemars_snapshots

# Validation integration tests
cargo test --all-features --test validation

# Fuzz (nightly, 1M iterations)
cargo +nightly fuzz run fuzz_deserialize_vertrag -- -runs=1000000
```

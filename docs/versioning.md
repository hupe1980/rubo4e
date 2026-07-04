# Schema Versioning

`rubo4e` exposes a single stable BO4E schema series (`v202501`), compiled
conditionally behind the `versioned` feature flag.

---

## Version Module Layout

With the `versioned` feature enabled:

```rust
rubo4e::v202501::Vertrag       // v202501 series (current stable)
rubo4e::v202501::Adresse
rubo4e::v202501::Sparte

rubo4e::current::Vertrag       // moving alias — always the latest stable series
```

Without the `versioned` feature, none of these module paths exist.  The default
feature set (`serde` only) does not include versioned types.

---

## Feature Gate

```toml
# Enable version modules (no external deps; controls conditional compilation only)
rubo4e = { version = "...", features = ["versioned"] }
```

This is a **pure conditional-compilation flag**.  Enabling `versioned` makes the
compiler include the `v202501` module.  No external dependencies are added.

---

## Known Schema Series

| Series  | Pinned tag    | Status                              |
|---------|---------------|-------------------------------------|
| v202501 | v202501.0.0   | Current stable; released April 2025 |

### Versioning Scheme Explained

BO4E uses `vYYYYMM.minor.patch`.  The `MM` component is always `01` — it
represents the release cycle, not the calendar month.  Module names use the
`vYYYYMM` prefix only:

```
v202501.0.0  →  module: v202501
```

Within a series, minor/patch bumps (e.g. `v202501.0.0` → `v202501.1.0`) are
additive.  The generator pins the full semver tag for reproducibility but exposes
only the series prefix in the public API.

---

## `rubo4e::current` — Moving Alias

`rubo4e::current` is a type alias that always points to the latest stable schema
series.  Use it when you always want the newest types and do not need to pin to a
specific version.

```rust
use rubo4e::current::Vertrag;   // equivalent to rubo4e::v202501::Vertrag today
```

Pin to a concrete module (`rubo4e::v202501`) if you need version-stability across
crate updates:

```rust
use rubo4e::v202501::Vertrag;   // stable even if rubo4e::current advances
```

---

## Adding a New Schema Version

When a new BO4E schema release arrives (e.g. `v202601.0.0`):

1. Download the schema snapshot:
   ```
   just download-schemas v202601.0.0
   ```
2. Run the generator:
   ```
   just generate v202601.0.0
   ```
3. The generator writes `src/generated/v202601/` and updates `src/generated/mod.rs`.
4. Add a `pub mod v202601` re-export in `src/lib.rs` and update the `current` alias.
5. Update this document with the new series row.

---

## COM and Enum Versioning

COM and enum types live inside the versioned module alongside BO types.  They
follow exactly the same conditional-compilation rules.

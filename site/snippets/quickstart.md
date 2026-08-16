```rust
use rubo4e::current::Marktlokation;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::Bo4eStrict;

// Parse with size, depth, and extension-data budgets enforced while reading.
let malo = Marktlokation::from_json_german_hardened(
    &body,
    JsonParseLimits::untrusted_defaults(),
)?;

// The serde path is deliberately lenient so a newer schema does not break this
// build. Opt into strictness where bad data must not get through.
malo.ensure_known_enums()?;

// From here it is ordinary Rust: the ID already verified its own check digit.
if let Some(id) = &malo.marktlokations_id {
    println!("{id}");
}
```

#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Lastgang contains nested `Zeitreihenwert` arrays (potentially large) and
    // `rust_decimal::Decimal` values — good coverage for depth/budget limits and
    // numeric edge cases (NaN-like strings, overflow, etc.).
    let _ = serde_json::from_slice::<rubo4e::v202501::Lastgang>(data);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rubo4e::v202501::Lastgang::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

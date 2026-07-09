#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Bilanzierung contains nested temporal ranges and decimal values; it also has
    // cross-field ordering invariants (bilanzierungsbeginn ≤ bilanzierungsende)
    // exercised at the deserialization + validation boundary.
    let _ = serde_json::from_slice::<rubo4e::v202607::Bilanzierung>(data);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rubo4e::v202607::Bilanzierung::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

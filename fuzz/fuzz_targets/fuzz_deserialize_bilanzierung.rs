#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Bilanzierung contains nested temporal ranges and decimal values; it also has
    // cross-field ordering invariants (bilanzierungsbeginn ≤ bilanzierungsende)
    // exercised at the deserialization + validation boundary.
    // Validation runs `Decimal` arithmetic and ordering comparisons over values
    // that came straight off the wire, so it is fuzzed alongside the decode
    // rather than after it. A validator that panics on a decodable payload is
    // as exploitable as a deserializer that does.
    if let Ok(v) = serde_json::from_slice::<rubo4e::v202607::Bilanzierung>(data) {
        let _ = rubo4e::prelude::Validate::validate(&v);
    }

    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(v) = rubo4e::v202607::Bilanzierung::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        ) {
            let _ = rubo4e::prelude::Validate::validate(&v);
        }
        // The snake_case reader is a second parser over the same bytes, with its
        // own key-transform and depth wrappers around the deserializer.
        let _ = rubo4e::v202607::Bilanzierung::from_json_snake_case_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

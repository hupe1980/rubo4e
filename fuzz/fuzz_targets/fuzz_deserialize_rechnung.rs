#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Exercise both the plain serde path and the hardened path with all limit checks.
    // Rechnung has complex arithmetic invariants (multi-Betrag cross-field checks), making
    // it a high-value target for fuzzing: malformed or edge-case decimal values should
    // never panic — only produce Err.
    // Validation runs `Decimal` arithmetic and ordering comparisons over values
    // that came straight off the wire, so it is fuzzed alongside the decode
    // rather than after it. A validator that panics on a decodable payload is
    // as exploitable as a deserializer that does.
    if let Ok(v) = serde_json::from_slice::<rubo4e::v202607::Rechnung>(data) {
        let _ = rubo4e::prelude::Validate::validate(&v);
    }

    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(v) = rubo4e::v202607::Rechnung::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        ) {
            let _ = rubo4e::prelude::Validate::validate(&v);
        }
        // The snake_case reader is a second parser over the same bytes, with its
        // own key-transform and depth wrappers around the deserializer.
        let _ = rubo4e::v202607::Rechnung::from_json_snake_case_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

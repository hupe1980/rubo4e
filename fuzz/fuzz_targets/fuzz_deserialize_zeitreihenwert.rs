#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Zeitreihenwert carries energy quantity time-series data — the most
    // performance-critical type in batch market-data processing.  Exercise both
    // the plain path and the hardened path (payload/depth/extension limits).
    // Validation runs `Decimal` arithmetic and ordering comparisons over values
    // that came straight off the wire, so it is fuzzed alongside the decode
    // rather than after it. A validator that panics on a decodable payload is
    // as exploitable as a deserializer that does.
    if let Ok(v) = serde_json::from_slice::<rubo4e::v202607::Zeitreihenwert>(data) {
        let _ = rubo4e::prelude::Validate::validate(&v);
    }

    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(v) = rubo4e::v202607::Zeitreihenwert::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        ) {
            let _ = rubo4e::prelude::Validate::validate(&v);
        }
        // The snake_case reader is a second parser over the same bytes, with its
        // own key-transform and depth wrappers around the deserializer.
        let _ = rubo4e::v202607::Zeitreihenwert::from_json_snake_case_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

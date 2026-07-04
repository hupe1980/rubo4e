#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Zeitreihenwert carries energy quantity time-series data — the most
    // performance-critical type in batch market-data processing.  Exercise both
    // the plain path and the hardened path (payload/depth/extension limits).
    let _ = serde_json::from_slice::<rubo4e::v202501::Zeitreihenwert>(data);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rubo4e::v202501::Zeitreihenwert::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

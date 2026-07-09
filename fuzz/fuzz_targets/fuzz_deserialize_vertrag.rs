#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Test both the plain deserializer and the hardened path (which enforces
    // payload, nesting-depth, and extension-budget limits).  Any panic here
    // is a bug; errors (invalid JSON / schema mismatches / limit hits) are
    // expected and OK.
    let _ = serde_json::from_slice::<rubo4e::v202607::Vertrag>(data);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rubo4e::v202607::Vertrag::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

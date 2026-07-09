#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

fuzz_target!(|data: &[u8]| {
    // Exercise both the plain serde path and the hardened path with all limit checks.
    // Rechnung has complex arithmetic invariants (multi-Betrag cross-field checks), making
    // it a high-value target for fuzzing: malformed or edge-case decimal values should
    // never panic — only produce Err.
    let _ = serde_json::from_slice::<rubo4e::v202607::Rechnung>(data);

    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rubo4e::v202607::Rechnung::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

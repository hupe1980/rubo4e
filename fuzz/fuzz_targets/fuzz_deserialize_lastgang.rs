#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::timeseries::Bo4eTimeSeries;

/// The timeline walk over attacker-controlled intervals.
///
/// `audit` parses every `startuhrzeit` / `enduhrzeit` off the wire, joins each
/// with its date, sorts the results and does `OffsetDateTime` arithmetic over
/// them — three places a payload can steer into an overflow. `time::Duration`
/// addition **panics** rather than saturating, so the accumulation is fuzzed
/// rather than reasoned about. `integrate` runs `Decimal` arithmetic over the
/// same values.
fn audit(v: &rubo4e::v202607::Lastgang) {
    let report = v.audit();
    let _ = report.coverage_ratio();
    let _ = report.missing();
    let _ = report.is_usable();
    let _ = v.integrate();
    let _ = v.sum();
    // …and against an explicit reference, which is the clipping path.
    if let Some(span) = v.span() {
        let _ = v.audit_over(span);
    }
}

fuzz_target!(|data: &[u8]| {
    // Lastgang contains nested `Zeitreihenwert` arrays (potentially large) and
    // `rust_decimal::Decimal` values — good coverage for depth/budget limits and
    // numeric edge cases (NaN-like strings, overflow, etc.).
    // Validation runs `Decimal` arithmetic and ordering comparisons over values
    // that came straight off the wire, so it is fuzzed alongside the decode
    // rather than after it. A validator that panics on a decodable payload is
    // as exploitable as a deserializer that does.
    if let Ok(v) = serde_json::from_slice::<rubo4e::v202607::Lastgang>(data) {
        let _ = rubo4e::prelude::Validate::validate(&v);
        audit(&v);
    }

    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(v) = rubo4e::v202607::Lastgang::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        ) {
            let _ = rubo4e::prelude::Validate::validate(&v);
            audit(&v);
        }
        // The snake_case reader is a second parser over the same bytes, with its
        // own key-transform and depth wrappers around the deserializer.
        let _ = rubo4e::v202607::Lastgang::from_json_snake_case_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

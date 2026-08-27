#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::json::{Bo4eExtensions, Bo4eJsonExt, JsonParseLimits};

/// The recursive walks over an attacker-shaped tree.
///
/// `collect_extension_paths` builds a `String` per undefined key from a key the
/// payload chose, at a depth the payload chose — the one place a path is
/// assembled from bytes rather than from the schema. `Kosten` is also the type
/// the reported decode-to-check trap was found on, so the check that closes it
/// is fuzzed on the same input as the decode that does not.
fn walk(v: &rubo4e::v202607::Kosten) {
    let _ = v.extension_paths();
    let _ = v.ensure_no_extension_data();
    let _ = rubo4e::Bo4eStrict::unknown_enum_paths(v);
}

fuzz_target!(|data: &[u8]| {
    // `Kosten` is the shortest route to `Kostenposition`, whose validator
    // multiplies two wire decimals and then derives a tolerance from the scale
    // of a third. All three are attacker-controlled, and `rust_decimal` panics
    // rather than erroring on several of its constructors — so this target
    // exists to run that arithmetic, not just to decode the type.
    // Validation runs `Decimal` arithmetic and ordering comparisons over values
    // that came straight off the wire, so it is fuzzed alongside the decode
    // rather than after it. A validator that panics on a decodable payload is
    // as exploitable as a deserializer that does.
    if let Ok(v) = serde_json::from_slice::<rubo4e::v202607::Kosten>(data) {
        let _ = rubo4e::prelude::Validate::validate(&v);
        walk(&v);
    }

    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(v) = rubo4e::v202607::Kosten::from_json_german_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        ) {
            let _ = rubo4e::prelude::Validate::validate(&v);
            walk(&v);
        }

        // Decoding an already-parsed `Value` is the third reader, and the one a
        // producer that assembled a document with `json!` actually calls.
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(s) {
            let closed =
                JsonParseLimits::unlimited().with_max_extension_field_count(Some(0));
            let _ = rubo4e::v202607::Kosten::from_json_value_hardened(value.clone(), closed);
            if let Ok(v) = rubo4e::v202607::Kosten::from_json_value(value) {
                walk(&v);
            }
        }

        // The snake_case reader is a second parser over the same bytes, with its
        // own key-transform and depth wrappers around the deserializer.
        let _ = rubo4e::v202607::Kosten::from_json_snake_case_hardened(
            s,
            JsonParseLimits::untrusted_defaults(),
        );
    }
});

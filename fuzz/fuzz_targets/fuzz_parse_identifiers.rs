#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Identifier-parsing functions accept arbitrary untrusted strings from JSON
    // payloads or user input and enforce structural / checksum constraints.
    // Panics anywhere in these parsers (overflow, unwrap, slice index) are bugs.
    if let Ok(s) = std::str::from_utf8(data) {
        // All identifiers: parse via FromStr — errors are expected and OK.
        let _ = s.parse::<rubo4e::identifiers::MaloId>();
        let _ = s.parse::<rubo4e::identifiers::MeloId>();
        let _ = s.parse::<rubo4e::identifiers::NeloId>();
        let _ = s.parse::<rubo4e::identifiers::EicCode>();
        let _ = s.parse::<rubo4e::identifiers::ObisCode>();
        let _ = s.parse::<rubo4e::identifiers::MarktpartnerId>();
        let _ = s.parse::<rubo4e::identifiers::SrId>();
        let _ = s.parse::<rubo4e::identifiers::TrId>();

        // Deserialize identifiers from a JSON string (most common untrusted path).
        let json_str = format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
        let _ = serde_json::from_str::<rubo4e::identifiers::MaloId>(&json_str);
        let _ = serde_json::from_str::<rubo4e::identifiers::EicCode>(&json_str);
        let _ = serde_json::from_str::<rubo4e::identifiers::ObisCode>(&json_str);
    }
});

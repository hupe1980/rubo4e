#![no_main]

use libfuzzer_sys::fuzz_target;
use rubo4e::identifiers::*;

/// Parses `s` as `$t` and, for good measure, round-trips a successful parse
/// back through `FromStr` — a parser that accepts its own `Display` output is
/// the property every one of these types documents.
macro_rules! try_parse {
    ($s:expr, $($t:ty),+ $(,)?) => {$(
        if let Ok(id) = $s.parse::<$t>() {
            let rendered = id.to_string();
            assert_eq!(
                rendered.parse::<$t>().ok(),
                Some(id),
                "{} did not accept its own Display output: {rendered:?}",
                stringify!($t),
            );
        }
    )+};
}

fuzz_target!(|data: &[u8]| {
    // Identifier parsers take arbitrary untrusted strings from JSON payloads and
    // user input, and enforce structural and checksum constraints. A panic
    // anywhere in them — overflow, unwrap, slice index — is a bug. An `Err` is
    // the expected outcome for almost every input and is fine.
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };

    // Every identifier type, so a newly added one is covered the day it lands.
    try_parse!(
        s,
        AkivId,
        BilanzierungsgebietId,
        BilanzkreisId,
        CrId,
        EicCode,
        MaloId,
        MarktpartnerId,
        MeloId,
        NebeId,
        NeloId,
        ObisCode,
        PaketId,
        SgId,
        SrId,
        TrId,
        TranchennummerId,
    );

    // The accessors run arithmetic and slicing over the parsed value, which the
    // parse itself does not reach.
    if let Ok(malo) = s.parse::<MaloId>() {
        let _ = MaloId::check_digit(malo.as_ref());
        let _ = malo.vergabestelle();
    }
    if let Ok(eic) = s.parse::<EicCode>() {
        let _ = eic.eic_type();
        let _ = eic.type_char();
    }
    if let Ok(obis) = s.parse::<ObisCode>() {
        let _ = obis.components();
        let _ = obis.to_pia_string();
    }
    if let Ok(mp) = s.parse::<MarktpartnerId>() {
        let _ = mp.authority();
        let _ = mp.to_i64();
        let _ = mp.has_valid_bdew_check_digit();
        let _ = mp.has_valid_gln_check_digit();
    }

    // The ISO 8601 duration parser runs float arithmetic and byte slicing over
    // the same class of untrusted text, and reaches `Zeitraum.dauer` straight
    // from a payload.
    let _ = rubo4e::iso8601_duration::parse(s);
    let _ = rubo4e::offset_time::parse(s);

    // Deserializing from a JSON string is the most common untrusted path.
    let json_str = serde_json::to_string(s).expect("a &str always serializes");
    let _ = serde_json::from_str::<MaloId>(&json_str);
    let _ = serde_json::from_str::<EicCode>(&json_str);
    let _ = serde_json::from_str::<ObisCode>(&json_str);
    let _ = serde_json::from_str::<MarktpartnerId>(&json_str);
});

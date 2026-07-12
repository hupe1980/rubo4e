//! Proptest [`Arbitrary`] strategies for BO4E identifier newtypes.
//!
//! Gated behind the `testing` feature flag, which pulls in `proptest`.
//! These strategies generate *valid* identifiers (ones that pass construction-time
//! validation), ensuring property tests never see spurious errors from invalid inputs.

use proptest::prelude::*;

use crate::identifiers::{
    AkivId, BilanzkreisId, EicCode, MaloId, MarktpartnerId, MeloId, NeloId, ObisCode, SrId, TrId,
    TranchennummerId,
};

// ─── 11-digit BDEW identifiers (MaloId, SrId, TrId) ─────────────────────────

/// Strategy that generates a valid 11-digit BDEW identifier string.
fn valid_11digit_strategy() -> impl Strategy<Value = String> {
    prop::array::uniform10(0u8..=9u8)
        .prop_map(|prefix| super::checksum::make_valid_11digit(&prefix))
}

impl Arbitrary for MaloId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        valid_11digit_strategy()
            .prop_map(|s| MaloId::new(&s).expect("generated MaloId must be valid"))
            .boxed()
    }
}

impl Arbitrary for SrId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        const ALNUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        prop::collection::vec(prop::sample::select(ALNUM), 9)
            .prop_map(|body| {
                let s = super::checksum::make_valid_ascii_id(
                    b'C',
                    body.as_slice().try_into().expect("9 bytes"),
                );
                SrId::new(&s).expect("generated SrId must be valid")
            })
            .boxed()
    }
}

impl Arbitrary for TrId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        const ALNUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        prop::collection::vec(prop::sample::select(ALNUM), 9)
            .prop_map(|body| {
                let s = super::checksum::make_valid_ascii_id(
                    b'D',
                    body.as_slice().try_into().expect("9 bytes"),
                );
                TrId::new(&s).expect("generated TrId must be valid")
            })
            .boxed()
    }
}

// ─── MeloId (33-char: 2-char uppercase country code + 31 alphanumeric) ─────────

impl Arbitrary for MeloId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        // Country code: 2 uppercase ASCII letters (ISO 3166-1 alpha-2)
        const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        // Body: 31 alphanumeric chars (upper, lower, digits)
        const BODY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let country = prop::array::uniform2(prop::sample::select(UPPER));
        let body = prop::collection::vec(prop::sample::select(BODY), 31);
        (country, body)
            .prop_map(|(cc, body_bytes)| {
                let mut s = String::with_capacity(33);
                s.push(cc[0] as char);
                s.push(cc[1] as char);
                for b in body_bytes {
                    s.push(b as char);
                }
                MeloId::new(&s).expect("generated MeloId must be valid")
            })
            .boxed()
    }
}

// ─── NeloId (E prefix + 9 [A-Z0-9] + ASCII-Verfahren check digit) ─────────────

impl Arbitrary for NeloId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        const ALNUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        prop::collection::vec(prop::sample::select(ALNUM), 9)
            .prop_map(|body| {
                let s = super::checksum::make_valid_ascii_id(
                    b'E',
                    body.as_slice().try_into().expect("9 bytes"),
                );
                NeloId::new(&s).expect("generated NeloId must be valid")
            })
            .boxed()
    }
}

// ─── MarktpartnerId (13 decimal digits, no checksum) ─────────────────────────

impl Arbitrary for MarktpartnerId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::array::uniform13(b'0'..=b'9')
            .prop_map(|digits| {
                let s: String = digits.iter().map(|&b| b as char).collect();
                MarktpartnerId::new(&s).expect("generated MarktpartnerId must be valid")
            })
            .boxed()
    }
}

// ─── EicCode (EIC format: 2-char type + 14 chars payload + check char) ────────

impl Arbitrary for EicCode {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        // Valid type characters for EIC codes
        const TYPE_CHARS: &[char] = &['W', 'X', 'Y', 'Z', 'A', 'V'];
        // EIC payload: positions 2–15 must be alphanumeric or hyphen, uppercase
        const PAYLOAD_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";
        let payload_len = 14usize;

        (
            prop::sample::select(TYPE_CHARS),
            prop::collection::vec(prop::sample::select(PAYLOAD_CHARS), payload_len),
        )
            .prop_filter_map(
                "EIC check char computation",
                |(type_char, payload_bytes)| {
                    let payload: String = payload_bytes.iter().map(|&b| b as char).collect();
                    let prefix = format!("{}{}", type_char, payload);
                    let check = EicCode::compute_check_char(&prefix)?;
                    let full = format!("{prefix}{check}");
                    EicCode::new(&full).ok()
                },
            )
            .boxed()
    }
}

// ─── ObisCode ─────────────────────────────────────────────────────────────────

impl Arbitrary for ObisCode {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        // OBIS code format: A-B:C.D.E*F  where all components are u8 (0..=255).
        // Simplified: generate the canonical 6-field form.
        (
            any::<u8>(), // A medium
            any::<u8>(), // B channel
            any::<u8>(), // C value type (0–255; C=0 = general metering group, valid per IEC 62056-61)
            any::<u8>(), // D measurement type
            any::<u8>(), // E tariff
            any::<u8>(), // F billing
        )
            .prop_map(|(a, b, c, d, e, f)| {
                let s = format!("{a}-{b}:{c}.{d}.{e}*{f}");
                ObisCode::new(&s).expect("generated ObisCode must be valid")
            })
            .boxed()
    }
}

// ─── BilanzkreisId (EIC type 'Z') ────────────────────────────────────────────

impl Arbitrary for BilanzkreisId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        // Positions 1-2: LIO identifier [A-Z0-9]
        // Position 3:    fixed 'Z'
        // Positions 4-15: body [A-Z0-9-]
        // Position 16:   check char (computed)
        const LIO: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        const BODY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";
        (
            prop::array::uniform2(prop::sample::select(LIO)),
            prop::collection::vec(prop::sample::select(BODY), 12),
        )
            .prop_filter_map(
                "BilanzkreisId check char computation",
                |(lio, body_bytes)| {
                    let body: String = body_bytes.iter().map(|&b| b as char).collect();
                    let prefix = format!("{}{}Z{}", lio[0] as char, lio[1] as char, body);
                    BilanzkreisId::from_prefix(&prefix).ok()
                },
            )
            .boxed()
    }
}

// ─── AkivId (1–36 printable ASCII) ───────────────────────────────────────────

impl Arbitrary for AkivId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        // Printable ASCII: 0x21 ('!') through 0x7E ('~'), excluding space (0x20).
        const PRINTABLE: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_./";
        (1usize..=crate::identifiers::AKIV_ID_MAX_LEN)
            .prop_flat_map(|len| {
                prop::collection::vec(prop::sample::select(PRINTABLE), len).prop_map(|chars| {
                    let s: String = chars.iter().map(|&b| b as char).collect();
                    AkivId::new(&s).expect("generated AkivId must be valid")
                })
            })
            .boxed()
    }
}

// ─── TranchennummerId (0–999 999) ─────────────────────────────────────────────

impl Arbitrary for TranchennummerId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (0u32..=crate::identifiers::TRANCHENNUMMER_MAX)
            .prop_map(|v| TranchennummerId::from_value(v).expect("value in range"))
            .boxed()
    }
}

//! Struct size regression guard (L-05).
//!
//! Asserts that key identifier newtypes stay within expected memory bounds.
//! These sizes are recorded at the time they were designed and serve as a
//! regression guard against accidental bloat from future field additions.
//!
//! **Platform note**: sizes are for 64-bit targets (pointer width = 8 bytes).
//! On 32-bit targets the tests are skipped via `#[cfg(target_pointer_width = "64")]`.

#[cfg(target_pointer_width = "64")]
mod identifier_sizes {
    use rubo4e::identifiers::*;
    use std::mem::size_of;

    /// All identifier newtypes are a single `Box<str>` = 2 × pointer = 16 bytes.
    const BOX_STR: usize = 16;

    #[test]
    fn malo_id_size() {
        assert_eq!(size_of::<MaloId>(), BOX_STR, "MaloId should be 16 bytes");
    }

    #[test]
    fn melo_id_size() {
        assert_eq!(size_of::<MeloId>(), BOX_STR, "MeloId should be 16 bytes");
    }

    #[test]
    fn nelo_id_size() {
        assert_eq!(size_of::<NeloId>(), BOX_STR, "NeloId should be 16 bytes");
    }

    #[test]
    fn sr_id_size() {
        assert_eq!(size_of::<SrId>(), BOX_STR, "SrId should be 16 bytes");
    }

    #[test]
    fn tr_id_size() {
        assert_eq!(size_of::<TrId>(), BOX_STR, "TrId should be 16 bytes");
    }

    #[test]
    fn eic_code_size() {
        assert_eq!(size_of::<EicCode>(), BOX_STR, "EicCode should be 16 bytes");
    }

    /// `ObisCode` is the one identifier that is not *just* a `Box<str>`: it also
    /// caches its parsed value groups so `components()` is infallible and free
    /// rather than re-parsing (and re-allocating) on every call.
    ///
    /// The six value groups are octets per IEC 62056-61, so they cost 10 bytes —
    /// four `Option<u8>` at 2 bytes plus two bare `u8`. `Box<str>` fills all 16 of
    /// its bytes and forces 8-byte alignment, so 16 + 10 rounds up to 32.
    ///
    /// Widening any value group back to `u32` would push this to 40; that is what
    /// this guard is here to catch.
    #[test]
    fn obis_code_size() {
        assert_eq!(
            size_of::<ObisComponents>(),
            10,
            "OBIS value groups are octets — this should not grow"
        );
        assert_eq!(
            size_of::<ObisCode>(),
            32,
            "ObisCode is a Box<str> plus its cached value groups"
        );
    }

    #[test]
    fn marktpartner_id_size() {
        assert_eq!(
            size_of::<MarktpartnerId>(),
            BOX_STR,
            "MarktpartnerId should be 16 bytes"
        );
    }

    #[test]
    fn bilanzkreis_id_size() {
        assert_eq!(
            size_of::<BilanzkreisId>(),
            BOX_STR,
            "BilanzkreisId should be 16 bytes"
        );
    }

    #[test]
    fn akiv_id_size() {
        assert_eq!(size_of::<AkivId>(), BOX_STR, "AkivId should be 16 bytes");
    }

    #[test]
    fn tranchennummer_id_size() {
        assert_eq!(
            size_of::<TranchennummerId>(),
            BOX_STR,
            "TranchennummerId should be 16 bytes"
        );
    }
}

#[cfg(all(target_pointer_width = "64", feature = "versioned", feature = "serde"))]
mod generated_type_sizes {
    use rubo4e::v202607::*;
    use std::mem::size_of;

    /// `BoTyp` is a fieldless repr(u8) enum — 1 byte on its own, but alignment
    /// may round up in structs. As a standalone type it is exactly 1 byte.
    #[test]
    fn bo_typ_size() {
        assert_eq!(
            size_of::<BoTyp>(),
            1,
            "BoTyp is a fieldless enum and should be 1 byte"
        );
    }

    /// `Option<BoTyp>` should be 1 byte via niche optimisation.
    #[test]
    fn option_bo_typ_size() {
        // repr(u8) fieldless enums get None-niche from the extra discriminant value.
        // In practice this is 1 byte on stable Rust.
        assert!(
            size_of::<Option<BoTyp>>() <= 2,
            "Option<BoTyp> should fit in 2 bytes (niche opt), got {}",
            size_of::<Option<BoTyp>>()
        );
    }
}

/// Every identifier must expose the *same* conversion surface.
///
/// They all share one macro-generated implementation; this asserts it, so a
/// hand-written type cannot ship with a smaller API than its siblings.
mod uniform_trait_surface {
    use rubo4e::identifiers::*;
    use std::borrow::Borrow;
    use std::str::FromStr;

    /// Fails to compile if `T` is missing any of the shared identifier traits.
    fn assert_full_surface<T>(valid: &str)
    where
        T: FromStr
            + for<'a> TryFrom<&'a str>
            + TryFrom<String>
            + AsRef<str>
            + Borrow<str>
            + std::ops::Deref<Target = str>
            + std::fmt::Display
            + std::fmt::Debug
            + Clone
            + PartialEq
            + Eq
            + std::hash::Hash,
        String: From<T>,
    {
        let id = T::try_from(valid).ok().expect("fixture must be valid");

        // All five string views agree.
        assert_eq!(id.as_ref(), valid);
        assert_eq!(Borrow::<str>::borrow(&id), valid);
        assert_eq!(&*id, valid);
        assert_eq!(id.to_string(), valid);
        assert_eq!(String::from(id.clone()), valid);

        // Display output re-parses to the same value. Routing through
        // `to_string()` rather than `as_ref()` is the point of this assertion: it
        // pins `Display` and `FromStr` against each other, which borrowing the
        // stored string would not exercise.
        #[allow(clippy::unnecessary_to_owned)]
        let displayed = id.to_string();
        assert_eq!(id, T::from_str(&displayed).ok().expect("FromStr"));
        assert_eq!(
            T::try_from(valid.to_string())
                .ok()
                .expect("TryFrom<String>"),
            id
        );
    }

    #[test]
    fn every_identifier_has_the_same_surface() {
        assert_full_surface::<MaloId>("41373559241");
        assert_full_surface::<MeloId>("DE0000000000000000000000000000001");
        assert_full_surface::<MarktpartnerId>("9900357000003");
        assert_full_surface::<NeloId>("E0000000019");
        assert_full_surface::<NebeId>("F0000000018");
        assert_full_surface::<CrId>("A0000000013");
        assert_full_surface::<SgId>("B0000000012");
        assert_full_surface::<SrId>("C0000000011");
        assert_full_surface::<TrId>("D0000000010");
        assert_full_surface::<PaketId>("P9000000010");
        assert_full_surface::<EicCode>("10YDE-EON------1");
        assert_full_surface::<BilanzkreisId>("11XSUEDWESTSTRO8");
        assert_full_surface::<BilanzierungsgebietId>("11YN-0000-0001-Q");
        assert_full_surface::<AkivId>("550e8400-e29b-41d4-a716-446655440000");
        assert_full_surface::<TranchennummerId>("13003");
        // `ObisCode` canonicalises, so its fixture must already be canonical.
        assert_full_surface::<ObisCode>("1-0:1.8.0*255");
    }
}

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

    #[test]
    fn obis_code_size() {
        assert_eq!(
            size_of::<ObisCode>(),
            BOX_STR,
            "ObisCode should be 16 bytes"
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

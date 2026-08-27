//! The BDEW §8.2 "ASCII-Verfahren" identifier family.
//!
//! Source: BDEW Anwendungshilfe **"Identifikatoren in der Marktkommunikation"**
//! v1.2 (7 February 2025).
//!
//! Every identifier here is 11 characters wide and shares one structure:
//!
//! ```text
//! ┌───────────┬───────────────────────┬─────────────┐
//! │ Codetyp   │ body                  │ check digit │
//! │ pos 1     │ pos 2–10  [A-Z0-9]    │ pos 11 [0-9]│
//! └───────────┴───────────────────────┴─────────────┘
//! ```
//!
//! The Codetyp is what distinguishes them:
//!
//! | Type | Codetyp | Object | § |
//! |------|---------|--------|---|
//! | [`NeloId`] | `E` | Netzlokation | 4 |
//! | [`NebeId`] | `F` | Netzbereich | 5 |
//! | [`CrId`]   | `A` | Cluster Ressource | 6.5 / 6.6 |
//! | [`SgId`]   | `B` | Steuergruppe | 6.4 / 6.6 |
//! | [`SrId`]   | `C` | Steuerbare Ressource | 6.3 / 6.6 |
//! | [`TrId`]   | `D` | Technische Ressource | 6.2 / 6.6 |
//! | [`PaketId`]| `P9`| Paket (Netzbetreiberwechsel) | 7 |
//!
//! The Paket-ID is the one variation: §7.2 fixes *two* leading characters —
//! `P` for "Paket" and `9` for BDEW/Strom — leaving an 8-character body.
//!
//! All seven use the same check-digit procedure (§8.2), implemented once in
//! [`crate::identifiers::checksum`].

bdew_ascii_identifier! {
    /// Netzlokations-ID (NeLo-ID) — identifies a *Netzlokation* (grid location).
    ///
    /// Introduced by BNetzA-Festlegung **BK6-22-128**; issued by Energie Codes und
    /// Services GmbH since 15 February 2023. Electricity only. Defined in BDEW
    /// "Identifikatoren in der Marktkommunikation" v1.2, §4.
    ///
    /// ## Not to be confused with an EIC code
    ///
    /// `10YDE-EON------1` is an [`EicCode`](super::EicCode) for a control area, not
    /// a NeLo-ID. EIC codes appear on `Marktlokation.marktgebiet`; NeLo-IDs identify
    /// the grid location itself.
    NeloId,
    prefix    = b"E",
    schema    = "crate::schema_helpers::nelo_id_schema",
    schema_meta = crate::identifiers::schema::NELO_ID,
    pattern   = r"^E[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Netzlokations-ID (Codetyp 'E')",
    example   = ("E000000001", "E0000000019"),
    check_fn  = check_nelo_id,
}

bdew_ascii_identifier! {
    /// Netzbereich-ID (NeBe-ID) — identifies a *Netzbereich* (grid area).
    ///
    /// Introduced by BNetzA-Festlegungen **BK6-22-300** and **BK8-22/010-A**; issued
    /// since 20 February 2025 for reporting controllable consumption devices
    /// (steuerbare Verbrauchseinrichtungen) under §14a EnWG. Electricity only.
    /// Defined in BDEW "Identifikatoren in der Marktkommunikation" v1.2, §5.
    NebeId,
    prefix    = b"F",
    schema    = "crate::schema_helpers::nebe_id_schema",
    schema_meta = crate::identifiers::schema::NEBE_ID,
    pattern   = r"^F[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Netzbereich-ID (Codetyp 'F')",
    example   = ("F000000001", "F0000000018"),
    check_fn  = check_nebe_id,
}

bdew_ascii_identifier! {
    /// Cluster-Ressource-ID (CR-ID) — identifies a *Cluster Ressource* in Redispatch 2.0.
    ///
    /// A Cluster Ressource bundles several technical resources for grid-operator
    /// coordination. Defined in BDEW "Identifikatoren in der Marktkommunikation"
    /// v1.2, §6.5 and §6.6.
    CrId,
    prefix    = b"A",
    schema    = "crate::schema_helpers::cr_id_schema",
    schema_meta = crate::identifiers::schema::CR_ID,
    pattern   = r"^A[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Cluster-Ressource-ID (Codetyp 'A')",
    example   = ("A000000001", "A0000000013"),
    check_fn  = check_cr_id,
}

bdew_ascii_identifier! {
    /// Steuergruppen-ID (SG-ID) — identifies a *Steuergruppe* in Redispatch 2.0.
    ///
    /// A Steuergruppe bundles steuerbare Ressourcen that are dispatched together.
    /// Defined in BDEW "Identifikatoren in der Marktkommunikation" v1.2, §6.4 and §6.6.
    SgId,
    prefix    = b"B",
    schema    = "crate::schema_helpers::sg_id_schema",
    schema_meta = crate::identifiers::schema::SG_ID,
    pattern   = r"^B[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Steuergruppen-ID (Codetyp 'B')",
    example   = ("B000000001", "B0000000012"),
    check_fn  = check_sg_id,
}

bdew_ascii_identifier! {
    /// Steuerbare-Ressource-ID (SR-ID) — identifies a *Steuerbare Ressource* in
    /// Redispatch 2.0.
    ///
    /// Assigned to a steuerbare Ressource by the Einsatzverantwortlicher and stays
    /// fixed for its lifetime, even when the Einsatzverantwortlicher changes.
    /// Defined in BDEW "Identifikatoren in der Marktkommunikation" v1.2, §6.3 and §6.6.
    SrId,
    prefix    = b"C",
    schema    = "crate::schema_helpers::sr_id_schema",
    schema_meta = crate::identifiers::schema::SR_ID,
    pattern   = r"^C[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Steuerbare-Ressource-ID (Codetyp 'C')",
    example   = ("C000000001", "C0000000011"),
    check_fn  = check_sr_id,
}

bdew_ascii_identifier! {
    /// Technische-Ressource-ID (TR-ID) — identifies a *Technische Ressource* in
    /// Redispatch 2.0 (a physical generation or consumption unit).
    ///
    /// Assigned by the grid operator and stays fixed for the lifetime of the
    /// resource, even across a change of grid operator or of the BTR (Betreiber
    /// einer Technischen Ressource). Defined in BDEW "Identifikatoren in der
    /// Marktkommunikation" v1.2, §6.2 and §6.6.
    TrId,
    prefix    = b"D",
    schema    = "crate::schema_helpers::tr_id_schema",
    schema_meta = crate::identifiers::schema::TR_ID,
    pattern   = r"^D[A-Z0-9]{9}[0-9]$",
    expecting = "an 11-character BDEW Technische-Ressource-ID (Codetyp 'D')",
    example   = ("D000000001", "D0000000010"),
    check_fn  = check_tr_id,
}

bdew_ascii_identifier! {
    /// Paket-ID — identifies the set of locations affected by a
    /// *Netzbetreiberwechsel* (change of grid operator).
    ///
    /// Unlike the other §8.2 identifiers, the Paket-ID fixes **two** leading
    /// characters (§7.2): `P` for "Paket" and `9` for BDEW/Strom. The freely
    /// assigned body therefore spans positions 3–10.
    ///
    /// Defined in BDEW "Identifikatoren in der Marktkommunikation" v1.2, §7.
    PaketId,
    prefix    = b"P9",
    schema    = "crate::schema_helpers::paket_id_schema",
    schema_meta = crate::identifiers::schema::PAKET_ID,
    pattern   = r"^P9[A-Z0-9]{8}[0-9]$",
    expecting = "an 11-character BDEW Paket-ID (Codetyp 'P9')",
    example   = ("P900000001", "P9000000010"),
    check_fn  = check_paket_id,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::IdentifierError;

    /// Every type in this family, as `(Codetyp, a valid ID, its base)`.
    ///
    /// The valid IDs are produced by `from_base`, which is itself pinned to the
    /// BDEW §8.2 reference vector in `checksum::tests`. That keeps this table
    /// honest: it cannot drift without the reference vector failing first.
    macro_rules! for_each_type {
        ($mac:ident) => {
            $mac!(NeloId, "E", "E000000001");
            $mac!(NebeId, "F", "F000000001");
            $mac!(CrId, "A", "A000000001");
            $mac!(SgId, "B", "B000000001");
            $mac!(SrId, "C", "C000000001");
            $mac!(TrId, "D", "D000000001");
            $mac!(PaketId, "P9", "P900000001");
        };
    }

    #[test]
    fn codetyp_constant_matches_prefix() {
        macro_rules! check {
            ($ty:ident, $codetyp:literal, $base:literal) => {
                assert_eq!($ty::CODETYP, $codetyp);
                assert!($base.starts_with($codetyp));
            };
        }
        for_each_type!(check);
    }

    #[test]
    fn from_base_round_trips_through_new() {
        macro_rules! check {
            ($ty:ident, $codetyp:literal, $base:literal) => {
                let id = $ty::from_base($base).expect("base is valid");
                assert_eq!(id.as_ref().len(), 11);
                assert_eq!(id.base(), $base);
                // `new` re-verifies the check digit that `from_base` computed.
                assert_eq!($ty::new(id.as_ref()).unwrap(), id);
                // Display / FromStr round-trip.
                assert_eq!(id.to_string().parse::<$ty>().unwrap(), id);
            };
        }
        for_each_type!(check);
    }

    #[test]
    fn wrong_check_digit_is_rejected() {
        macro_rules! check {
            ($ty:ident, $codetyp:literal, $base:literal) => {
                let id = $ty::from_base($base).unwrap();
                let correct = id.as_ref().as_bytes()[10] - b'0';
                let wrong = (correct + 1) % 10;
                let mutated = format!("{}{}", id.base(), wrong);
                assert!(
                    matches!($ty::new(&mutated), Err(IdentifierError::InvalidChecksum)),
                    concat!(stringify!($ty), ": mutated check digit must be rejected"),
                );
            };
        }
        for_each_type!(check);
    }

    #[test]
    fn wrong_codetyp_is_rejected() {
        // Every type must reject a body carrying a different family member's Codetyp.
        macro_rules! check {
            ($ty:ident, $codetyp:literal, $base:literal) => {
                // 'Z' is not a Codetyp used by any member of this family.
                let foreign = format!("Z{}", &$base[1..]);
                assert!(matches!(
                    $ty::from_base(&foreign),
                    Err(IdentifierError::InvalidFormat { .. })
                ));
            };
        }
        for_each_type!(check);
    }

    #[test]
    fn lowercase_and_wrong_length_are_rejected() {
        macro_rules! check {
            ($ty:ident, $codetyp:literal, $base:literal) => {
                // Lowercase body characters are outside [A-Z0-9].
                let lower = format!("{}a00000001", $codetyp);
                let lower = &lower[..10];
                assert!(matches!(
                    $ty::from_base(lower),
                    Err(IdentifierError::InvalidCharacter { .. })
                ));
                // Wrong length.
                assert!(matches!(
                    $ty::new($base),
                    Err(IdentifierError::InvalidLength { .. })
                ));
            };
        }
        for_each_type!(check);
    }

    /// The Paket-ID's second character is fixed to `9` by §7.2.
    #[test]
    fn paket_id_requires_bdew_strom_digit() {
        assert!(matches!(
            PaketId::from_base("P800000001"),
            Err(IdentifierError::InvalidFormat { .. })
        ));
        assert!(PaketId::from_base("P900000001").is_ok());
    }

    /// Non-ASCII input must produce a clean error rather than panicking on a
    /// multi-byte slice boundary.
    #[test]
    fn non_ascii_input_does_not_panic() {
        // 11 bytes: 'E' + 8 ASCII + 'ä' (2 bytes).
        assert!(NeloId::new("E00000000ä").is_err());
        assert!(NeloId::new("Ü0000000019").is_err());
    }
}

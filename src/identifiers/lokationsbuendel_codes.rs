//! The two 13-digit BDEW codes that carry a Lokationsbündelstruktur.
//!
//! EDI@Energy's **"Codeliste der Lokationsbündelstrukturen"** (BDEW, version 1.0,
//! published 31 March 2023, to be applied from 1 October 2024) describes a
//! Netzanschluss as a *bundle*: which Marktlokationen, Messlokationen,
//! Netzlokationen and technische Ressourcen sit behind it, on which level, and in
//! which energy-flow direction. Two codes carry that:
//!
//! | Code | Says | BO4E field |
//! |---|---|---|
//! | [`Lokationsbuendelcode`] | *which* structure this bundle has | `Lokationszuordnung.lokationsbuendelcode` |
//! | [`LokationsbuendelObjektcode`] | *where in it* one object sits | `<BO>.lokationsbuendelObjektcode` |
//!
//! Both are 13-digit BDEW Codenummern ending in a §8.1 check digit — the same
//! arithmetic as a [`MaloId`](super::MaloId). Every one of the 15 structure codes
//! and 27 object codes the codelist publishes verifies under it, and this module
//! enforces it; [`MarktpartnerId`](super::MarktpartnerId) is the sibling that
//! cannot, because an MP-ID may carry a GS1 check digit instead.
//!
//! [`crate::lokationsbuendel`] is what turns a validated code into its meaning.
//!
//! # Spelling
//!
//! `buendel`, not `bündel`, and `Objektcode`, not `ObjektCode`: BO4E transliterates
//! umlauts in every wire key and writes the compound as one word
//! (`lokationsbuendelcode`, `lokationsbuendelObjektcode`). A type named after a
//! BO4E compound keeps BO4E's word boundaries, so it reads the same as the field
//! it validates. A name this crate coins itself follows Rust convention instead —
//! which is why [`BilanzierungsgebietId`](super::BilanzierungsgebietId) has a
//! capital `I`: BO4E has no such compound.
//!
//! # Example
//!
//! ```
//! use rubo4e::identifiers::{Lokationsbuendelcode, LokationsbuendelObjektcode};
//!
//! // "Verbrauch mit einer Messlokation (Standard)" — the codelist prints codes
//! // with spaces for readability; the wire never has them.
//! let struktur = Lokationsbuendelcode::new("9992000000026").unwrap();
//! assert_eq!(struktur.grouped(), "9992 00000 002 6");
//!
//! // The Marktlokation (Verbrauch) on level 1 of that structure.
//! let objekt = LokationsbuendelObjektcode::new("9992000001016").unwrap();
//!
//! // A transposed digit fails the check digit.
//! assert!(Lokationsbuendelcode::new("9992000000062").is_err());
//! ```

use super::checksum::{compute_numeric_id_from_base, validate_numeric_id};
use crate::error::IdentifierError;

/// Length of a Lokationsbündel code, including the check digit.
const LEN: usize = 13;

/// The published codes all begin `9992`, but the codelist covers electricity
/// only, so no leading digit is required beyond "is a digit".
const MIN_FIRST_DIGIT: u8 = 0;

/// Defines one of the two 13-digit Lokationsbündel codes.
///
/// The two differ only in what they name — a structure or an object's place in
/// one — so everything except the documentation is generated here. They are
/// separate types precisely because they must never be swapped: a
/// `Lokationszuordnung` carrying an object code where its structure code belongs
/// describes a bundle that does not exist.
macro_rules! lokationsbuendel_code {
    (
        $ty:ident,
        $schema_fn:literal,
        $schema_meta:expr,
        $pattern:literal,
        $expecting:literal,
        $example_base:literal,
        $example_full:literal,
        $(#[$doc:meta])*
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "validate", derive(garde::Validate))]
        #[cfg_attr(feature = "validate", garde(allow_unvalidated))]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schemars", schemars(schema_with = $schema_fn))]
        #[cfg_attr(feature = "schemars", schemars(description = $schema_meta.description))]
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[cfg_attr(feature = "utoipa", schema(
            value_type = String,
            pattern = $pattern,
            example = $example_full,
            description = $schema_meta.description
        ))]
        pub struct $ty(
            #[cfg_attr(feature = "validate", garde(custom(validate_code)))] Box<str>,
        );

        impl $ty {
            /// Creates the code, validating the 13-digit form and the §8.1 check
            /// digit.
            ///
            /// # Errors
            /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 13 characters.
            /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
            /// - [`IdentifierError::InvalidChecksum`] if the 13th digit does not match.
            #[must_use = "the validated code is returned; ignoring it discards the result"]
            pub fn new(s: &str) -> Result<Self, IdentifierError> {
                validate_numeric_id(s, LEN, MIN_FIRST_DIGIT)?;
                Ok(Self(Box::from(s)))
            }

            /// Builds the code from its 12-digit base by computing and appending
            /// the §8.1 check digit.
            ///
            /// # Errors
            /// - [`IdentifierError::InvalidLength`] if `base` is not exactly 12 characters.
            /// - [`IdentifierError::InvalidCharacter`] if any character is not a decimal digit.
            ///
            /// # Examples
            /// ```
            #[doc = concat!("use rubo4e::identifiers::", stringify!($ty), ";")]
            ///
            #[doc = concat!("let code = ", stringify!($ty), "::from_base(\"", $example_base, "\").unwrap();")]
            #[doc = concat!("assert_eq!(code.as_ref(), \"", $example_full, "\");")]
            /// ```
            #[must_use = "the validated code is returned; ignoring it discards the result"]
            pub fn from_base(base: &str) -> Result<Self, IdentifierError> {
                compute_numeric_id_from_base(base, LEN, MIN_FIRST_DIGIT).map(|s| Self(s.into_boxed_str()))
            }

            /// Returns the §8.1 check digit for a 12-digit base, without building
            /// the code.
            ///
            /// # Errors
            /// Same as [`from_base`](Self::from_base).
            pub fn check_digit(base: &str) -> Result<u8, IdentifierError> {
                let full = compute_numeric_id_from_base(base, LEN, MIN_FIRST_DIGIT)?;
                Ok(full.as_bytes()[LEN - 1] - b'0')
            }

            /// Returns the code in the codelist's own printed grouping —
            /// `"9992 00000 002 6"`.
            ///
            /// The document groups the digits `4-5-3-1` purely for legibility and
            /// says so; the wire form never carries the spaces. Use this for a
            /// human-facing report, never for output.
            #[must_use]
            pub fn grouped(&self) -> String {
                let s: &str = &self.0;
                format!("{} {} {} {}", &s[0..4], &s[4..9], &s[9..12], &s[12..13])
            }

            /// The validated 13-digit wire form.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl_identifier_traits!($ty, $expecting);
    };
}

#[cfg(feature = "validate")]
fn validate_code(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_numeric_id(value, LEN, MIN_FIRST_DIGIT).map_err(garde::Error::from)
}

lokationsbuendel_code!(
    Lokationsbuendelcode,
    "crate::schema_helpers::lokationsbuendel_code_schema",
    crate::identifiers::schema::LOKATIONSBUENDEL_CODE,
    r"^[0-9]{13}$",
    "a 13-digit Lokationsbündelstruktur code with a valid BDEW check digit",
    "999200000002",
    "9992000000026",
    /// Code der **Lokationsbündelstruktur** — *which* bundle structure a
    /// Netzanschluss has.
    ///
    /// Carried by `Lokationszuordnung.lokationsbuendelcode`. The codelist
    /// publishes 15 of them, from "Verbrauch ohne Messlokation (Pauschal)" to
    /// "Verbrauchskaskade mit ungemessenem Verbrauch und gemessener Erzeugung";
    /// [`Lokationsbuendelstruktur::from_code`] resolves one to its objects,
    /// their levels, directions and cardinalities.
    ///
    /// A code outside the published list still constructs, as long as its check
    /// digit is right: the codelist's own introduction says complex or special
    /// structures are agreed bilaterally rather than coded, and BDEW may extend
    /// the list. [`Lokationsbuendelstruktur::from_code`] returns `None` for one.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::Lokationsbuendelcode;
    /// use rubo4e::lokationsbuendel::Lokationsbuendelstruktur;
    ///
    /// let code = Lokationsbuendelcode::new("9992000000026").unwrap();
    /// let struktur = Lokationsbuendelstruktur::from_code(&code).unwrap();
    /// assert_eq!(struktur.bezeichnung, "Verbrauch mit einer Messlokation (Standard)");
    /// ```
    ///
    /// [`Lokationsbuendelstruktur::from_code`]: crate::lokationsbuendel::Lokationsbuendelstruktur::from_code
);

lokationsbuendel_code!(
    LokationsbuendelObjektcode,
    "crate::schema_helpers::lokationsbuendel_objektcode_schema",
    crate::identifiers::schema::LOKATIONSBUENDEL_OBJEKTCODE,
    r"^[0-9]{13}$",
    "a 13-digit Lokationsbündel object code with a valid BDEW check digit",
    "999200000101",
    "9992000001016",
    /// **Objekt-Code** — *where in* a Lokationsbündelstruktur one object sits.
    ///
    /// Carried by `lokationsbuendelObjektcode` on `Marktlokation`,
    /// `Messlokation`, `Netzlokation`, `SteuerbareRessource` and
    /// `TechnischeRessource`. The code pins three facts at once: the object type
    /// (MaLo / MeLo / NeLo / TR), the energy-flow direction, and the level
    /// (1–3) — so `9992 00000 101 6` is *the* consumption Marktlokation on level 1
    /// and nothing else.
    ///
    /// [`Objektrolle::from_code`] resolves those facts.
    ///
    /// # Examples
    /// ```
    /// use rubo4e::identifiers::LokationsbuendelObjektcode;
    /// use rubo4e::lokationsbuendel::{Objektrolle, Objekttyp, Flussrichtung};
    ///
    /// let code = LokationsbuendelObjektcode::new("9992000001016").unwrap();
    /// let rolle = Objektrolle::from_code(&code).unwrap();
    /// assert_eq!(rolle.objekttyp, Objekttyp::Marktlokation);
    /// assert_eq!(rolle.richtung, Some(Flussrichtung::Verbrauch));
    /// assert_eq!(rolle.ebene, 1);
    /// ```
    ///
    /// [`Objektrolle::from_code`]: crate::lokationsbuendel::Objektrolle::from_code
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_structure_codes_all_verify() {
        for code in crate::lokationsbuendel::STRUKTUREN {
            assert!(
                Lokationsbuendelcode::new(code.code).is_ok(),
                "{} must satisfy the BDEW §8.1 check digit",
                code.code
            );
        }
    }

    #[test]
    fn published_object_codes_all_verify() {
        for rolle in crate::lokationsbuendel::OBJEKTROLLEN {
            assert!(
                LokationsbuendelObjektcode::new(rolle.code).is_ok(),
                "{} must satisfy the BDEW §8.1 check digit",
                rolle.code
            );
        }
    }

    #[test]
    fn grouped_matches_the_codelist_printing() {
        let c = Lokationsbuendelcode::new("9992000000026").unwrap();
        assert_eq!(c.grouped(), "9992 00000 002 6");
        let o = LokationsbuendelObjektcode::new("9992000001090").unwrap();
        assert_eq!(o.grouped(), "9992 00000 109 0");
    }

    #[test]
    fn wrong_length_and_checksum_are_rejected() {
        assert!(matches!(
            Lokationsbuendelcode::new("999200000002"),
            Err(IdentifierError::InvalidLength { .. })
        ));
        assert!(matches!(
            Lokationsbuendelcode::new("9992000000027"),
            Err(IdentifierError::InvalidChecksum)
        ));
        assert!(matches!(
            LokationsbuendelObjektcode::new("999200000101X"),
            Err(IdentifierError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn from_base_appends_the_check_digit() {
        assert_eq!(
            LokationsbuendelObjektcode::from_base("999200000101").unwrap(),
            LokationsbuendelObjektcode::new("9992000001016").unwrap()
        );
        assert_eq!(
            LokationsbuendelObjektcode::check_digit("999200000101").unwrap(),
            6
        );
    }

    /// The two codes share an arithmetic but must not share a type: a structure
    /// code where an object code belongs describes a bundle that does not exist.
    #[test]
    fn the_two_codes_are_distinct_types() {
        let s = Lokationsbuendelcode::new("9992000000026").unwrap();
        let o = LokationsbuendelObjektcode::new("9992000000026").unwrap();
        assert_eq!(s.as_str(), o.as_str());
        // `s == o` does not compile — that is the point of the two types.
    }
}

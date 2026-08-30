//! The 33-character Zählpunktbezeichnung, and the two things it can be.
//!
//! BO4E calls `Messlokation.messlokationsId` *"Die Messlokations-Identifikation;
//! Das ist die frühere Zählpunktbezeichnung"* — one grammar, and BO4E assumes one
//! meaning. MaBiS does not: a Zählpunktbezeichnung also names points that are not
//! Messlokationen at all, and the BDEW Anwendungshilfe to BK6-20-160 says so
//! outright for the e-mobility case (§ 1.6.2):
//!
//! > Für den Zählpunkt (eMob) wird eine ID (Zählpunktbezeichnung) vergeben.
//! > **Hinweis: Für den Zählpunkt (eMob) wird nicht die ID der Messlokation
//! > (Zählpunktbezeichnung) verwendet.**
//!
//! So [`MeloId`](super::MeloId) and [`Zaehlpunktbezeichnung`] validate identically
//! and are deliberately different types: assigning a Zählpunkt (eMob) to
//! `Messlokation.messlokationsId` is exactly the mistake the standard warns
//! against, and only a type can stop it.
//!
//! [`Zaehlpunktart`] is what says *which* kind of Zählpunkt one is — and
//! [`Zaehlpunkt::as_melo_id`] is the narrowing that refuses for every kind that
//! is not one.
//!
//! ```
//! use rubo4e::identifiers::Zaehlpunktbezeichnung;
//! use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart};
//!
//! let zpb = Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap();
//! let zp = Zaehlpunkt::new(Zaehlpunktart::NetzgangzeitreiheEmob, zpb);
//!
//! assert!(!zp.is_messlokation());
//! assert_eq!(zp.country_code(), "DE");
//! ```

use crate::error::{IdentifierError, LengthExpectation};

/// Length of a Zählpunktbezeichnung, in characters.
pub(super) const ZAEHLPUNKT_LEN: usize = 33;

/// Validates the 33-character Zählpunktbezeichnung grammar.
///
/// Shared by [`MeloId`](super::MeloId) and [`Zaehlpunktbezeichnung`]: the two
/// carry the same grammar and differ only in what they mean, so the rule is
/// written once here rather than twice and left to drift.
///
/// - Positions 1–2: ISO 3166-1 alpha-2 country code, uppercase ASCII.
/// - Positions 3–33: `[A-Za-z0-9]`.
pub(super) fn validate_zaehlpunktbezeichnung(s: &str) -> Result<(), IdentifierError> {
    if s.len() != ZAEHLPUNKT_LEN {
        return Err(IdentifierError::InvalidLength {
            expected: LengthExpectation::Exact(ZAEHLPUNKT_LEN),
            actual: s.len(),
        });
    }
    for c in s.chars().take(2) {
        if !c.is_ascii_uppercase() {
            return Err(IdentifierError::InvalidFormat {
                description:
                    "first two characters must be uppercase ISO 3166-1 country code (e.g. \"DE\")"
                        .into(),
            });
        }
    }
    for (i, c) in s.chars().enumerate().skip(2) {
        if !c.is_ascii_alphanumeric() {
            return Err(IdentifierError::InvalidCharacter {
                position: i,
                character: c,
            });
        }
    }
    Ok(())
}

/// A 33-character **Zählpunktbezeichnung** that is not (necessarily) a
/// Messlokations-ID.
///
/// Same grammar as [`MeloId`](super::MeloId) — 2-character ISO 3166-1 country
/// code plus 31 alphanumerics — and deliberately a different type. MaBiS names
/// several points with a Zählpunktbezeichnung that are not Messlokationen:
///
/// | Kind | Carries |
/// |---|---|
/// | Zählpunkt (Netzübergabe) | the Netzgangzeitreihe between two Bilanzierungsgebiete of VNB |
/// | Zählpunkt (eMob) | the NGZ (eMob) between the BG of the NB (VNB) and the BG of the NB (LPB) |
/// | MaBiS-Zählpunkt für NZR (eMob) | the summed NZR (eMob) between those two BG |
///
/// [`Zaehlpunktart`] enumerates them, and [`Zaehlpunkt`] pairs one with an ID.
///
/// # Examples
/// ```
/// use rubo4e::identifiers::{MeloId, Zaehlpunktbezeichnung};
///
/// let zpb = Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap();
/// assert_eq!(zpb.country_code(), "DE");
/// assert!(zpb.is_german());
///
/// // The grammar is the same, so a MeLo-ID converts — explicitly, never by
/// // accident, because the two are different types.
/// let melo = MeloId::new("DE0000000000000000000000000000001").unwrap();
/// let as_zpb = Zaehlpunktbezeichnung::from(melo.clone());
/// assert_eq!(as_zpb.as_ref(), melo.as_ref());
///
/// // A 33-character rule, enforced the same way.
/// assert!(Zaehlpunktbezeichnung::new("DE123").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "validate", derive(garde::Validate))]
#[cfg_attr(feature = "validate", garde(allow_unvalidated))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schemars",
    schemars(schema_with = "crate::schema_helpers::zaehlpunktbezeichnung_schema")
)]
#[cfg_attr(
    feature = "schemars",
    schemars(description = crate::identifiers::schema::ZAEHLPUNKTBEZEICHNUNG.description)
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(
    value_type = String,
    pattern = r"^[A-Z]{2}[A-Za-z0-9]{31}$",
    example = "DE0000000000000000000000000000042",
    description = crate::identifiers::schema::ZAEHLPUNKTBEZEICHNUNG.description
))]
pub struct Zaehlpunktbezeichnung(
    #[cfg_attr(feature = "validate", garde(custom(check_zaehlpunktbezeichnung)))] Box<str>,
);

#[cfg(feature = "validate")]
fn check_zaehlpunktbezeichnung(value: &str, _: &()) -> Result<(), garde::Error> {
    validate_zaehlpunktbezeichnung(value).map_err(garde::Error::from)
}

impl Zaehlpunktbezeichnung {
    /// Creates the Zählpunktbezeichnung, validating the 33-character grammar.
    ///
    /// # Errors
    /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 33 characters.
    /// - [`IdentifierError::InvalidFormat`] if the first two characters are not
    ///   an uppercase country code.
    /// - [`IdentifierError::InvalidCharacter`] if the body is not alphanumeric.
    #[must_use = "the validated Zählpunktbezeichnung is returned; ignoring it discards the result"]
    pub fn new(s: &str) -> Result<Self, IdentifierError> {
        validate_zaehlpunktbezeichnung(s)?;
        Ok(Self(Box::from(s)))
    }

    /// The ISO 3166-1 alpha-2 country code — the first two characters.
    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[..2]
    }

    /// `true` if the country code is `"DE"`.
    #[must_use]
    pub fn is_german(&self) -> bool {
        self.country_code() == "DE"
    }

    /// The validated 33-character wire form.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Reads this Zählpunktbezeichnung as a Messlokations-ID.
    ///
    /// Infallible — the grammars are identical — but deliberately explicit: a
    /// Zählpunkt (eMob) is *not* a Messlokation, and the standard says so. Call
    /// this only where you know the point is one.
    #[must_use]
    pub fn into_melo_id(self) -> super::MeloId {
        super::MeloId::new(&self.0).expect("the two grammars are identical")
    }
}

impl From<super::MeloId> for Zaehlpunktbezeichnung {
    /// Every Messlokations-ID *is* a Zählpunktbezeichnung — BO4E says so on the
    /// field itself. The reverse is [`into_melo_id`](Zaehlpunktbezeichnung::into_melo_id),
    /// which is a claim rather than a fact.
    fn from(id: super::MeloId) -> Self {
        Self(Box::from(id.as_ref()))
    }
}

impl_identifier_traits!(
    Zaehlpunktbezeichnung,
    "a 33-character Zählpunktbezeichnung: ISO 3166-1 country code + 31 alphanumeric characters"
);

// ─── What a Zählpunktbezeichnung names ───────────────────────────────────────

/// Which kind of point a [`Zaehlpunktbezeichnung`] names.
///
/// BO4E has one field for a Zählpunktbezeichnung — `Messlokation.messlokationsId`,
/// *"Das ist die frühere Zählpunktbezeichnung"* — and so assumes every one names
/// a Messlokation. MaBiS names four kinds, and the BDEW Anwendungshilfe to
/// BK6-20-160 (§ 1.6.2) is explicit that one of them is **not** a Messlokation:
///
/// > Für den Zählpunkt (eMob) wird eine ID (Zählpunktbezeichnung) vergeben.
/// > Hinweis: Für den Zählpunkt (eMob) wird **nicht** die ID der Messlokation
/// > (Zählpunktbezeichnung) verwendet.
///
/// # Extrinsic, unlike [`EicType`](super::EicType)
///
/// [`EicType`](super::EicType), [`MaloVergabestelle`](super::MaloVergabestelle) and
/// [`MpIdAuthority`](super::MpIdAuthority) are all *read out of* their identifier —
/// position 3 of an EIC, digit 1 of a MaLo-ID. A Zählpunktart cannot be: a
/// Zählpunkt (eMob) and a MeLo-ID are indistinguishable as strings. That is
/// precisely why it has to be carried alongside, and why
/// [`well_known::ZAEHLPUNKT`](crate::zusatz_attribut::well_known::ZAEHLPUNKT)
/// exists.
///
/// `#[non_exhaustive]`: unlike the closed BDEW codelists this crate ships, MaBiS
/// adds Zählpunktarten as processes are added — the eMob pair arrived with
/// BK6-20-160 itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum Zaehlpunktart {
    /// An ordinary Messlokation — the one kind BO4E models, on
    /// `Messlokation.messlokationsId`.
    Messlokation,
    /// **Zählpunkt (Netzübergabe)** — carries the Netzgangzeitreihe between two
    /// Bilanzierungsgebiete of Verteilnetzbetreiber.
    Netzuebergabe,
    /// **Zählpunkt (eMob)** — carries the NGZ (eMob) between the
    /// Bilanzierungsgebiet of the NB (VNB) and that of the NB (LPB).
    ///
    /// The one the standard warns is not a MeLo-ID.
    NetzgangzeitreiheEmob,
    /// **MaBiS-Zählpunkt für NZR (eMob)** — carries the summed Netzzeitreihe
    /// (eMob) between those same two Bilanzierungsgebiete.
    NetzzeitreiheEmob,
    /// A MaBiS-Zählpunkt that is none of the above — `LOC+Z15` on the wire.
    MabisZaehlpunkt,
}

impl Zaehlpunktart {
    /// `true` for the two kinds BK6-20-160 Modell 2 introduces.
    #[must_use]
    pub const fn is_emobilitaet(self) -> bool {
        matches!(self, Self::NetzgangzeitreiheEmob | Self::NetzzeitreiheEmob)
    }

    /// `true` if a Zählpunkt of this kind is a BO4E
    /// [`Messlokation`](crate::current::Messlokation).
    ///
    /// Only [`Messlokation`](Zaehlpunktart::Messlokation) is. This is the guard
    /// that stops a Zählpunkt (eMob) being written to `messlokationsId`.
    #[must_use]
    pub const fn is_messlokation(self) -> bool {
        matches!(self, Self::Messlokation)
    }

    /// The German label MaBiS and the BDEW Anwendungshilfe use.
    #[must_use]
    pub const fn bezeichnung(self) -> &'static str {
        match self {
            Self::Messlokation => "Messlokation",
            Self::Netzuebergabe => "Zählpunkt (Netzübergabe)",
            Self::NetzgangzeitreiheEmob => "Zählpunkt (eMob)",
            Self::NetzzeitreiheEmob => "MaBiS-Zählpunkt für NZR (eMob)",
            Self::MabisZaehlpunkt => "MaBiS-Zählpunkt",
        }
    }
}

impl std::fmt::Display for Zaehlpunktart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.bezeichnung())
    }
}

/// A Zählpunktbezeichnung together with what it names.
///
/// ```
/// use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart, Zaehlpunktbezeichnung};
///
/// let zp = Zaehlpunkt::new(
///     Zaehlpunktart::NetzgangzeitreiheEmob,
///     Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap(),
/// );
///
/// assert!(zp.is_emobilitaet());
/// assert!(!zp.is_messlokation());
/// // …so it must not become a `Messlokation.messlokationsId`.
/// assert_eq!(zp.as_melo_id(), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Zaehlpunkt {
    /// What this Zählpunkt names.
    pub art: Zaehlpunktart,
    /// The 33-character Zählpunktbezeichnung.
    pub bezeichnung: Zaehlpunktbezeichnung,
}

impl Zaehlpunkt {
    /// Pairs a Zählpunktbezeichnung with what it names.
    #[must_use]
    pub const fn new(art: Zaehlpunktart, bezeichnung: Zaehlpunktbezeichnung) -> Self {
        Self { art, bezeichnung }
    }

    /// See [`Zaehlpunktart::is_emobilitaet`].
    #[must_use]
    pub const fn is_emobilitaet(&self) -> bool {
        self.art.is_emobilitaet()
    }

    /// See [`Zaehlpunktart::is_messlokation`].
    #[must_use]
    pub const fn is_messlokation(&self) -> bool {
        self.art.is_messlokation()
    }

    /// The ID as a [`MeloId`](super::MeloId) — **only** where this
    /// Zählpunkt really is a Messlokation.
    ///
    /// `None` for every other kind: the grammars are identical, so nothing but
    /// this check stops a Zählpunkt (eMob) being filed as a Messlokation.
    #[must_use]
    pub fn as_melo_id(&self) -> Option<super::MeloId> {
        self.is_messlokation()
            .then(|| self.bezeichnung.clone().into_melo_id())
    }

    /// The country code of the Zählpunktbezeichnung.
    #[must_use]
    pub fn country_code(&self) -> &str {
        self.bezeichnung.country_code()
    }
}

impl std::fmt::Display for Zaehlpunkt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.art, self.bezeichnung)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::MeloId;

    #[test]
    fn a_zaehlpunkt_emob_is_not_a_messlokation() {
        let zpb = Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap();
        let emob = Zaehlpunkt::new(Zaehlpunktart::NetzgangzeitreiheEmob, zpb.clone());
        assert!(emob.is_emobilitaet());
        assert!(!emob.is_messlokation());
        assert_eq!(emob.as_melo_id(), None);

        let melo = Zaehlpunkt::new(Zaehlpunktart::Messlokation, zpb);
        assert!(!melo.is_emobilitaet());
        assert_eq!(
            melo.as_melo_id(),
            Some(MeloId::new("DE0000000000000000000000000000042").unwrap())
        );
    }

    #[test]
    fn every_zaehlpunktart_has_a_label() {
        for art in [
            Zaehlpunktart::Messlokation,
            Zaehlpunktart::Netzuebergabe,
            Zaehlpunktart::NetzgangzeitreiheEmob,
            Zaehlpunktart::NetzzeitreiheEmob,
            Zaehlpunktart::MabisZaehlpunkt,
        ] {
            assert!(!art.bezeichnung().is_empty());
            assert_eq!(art.is_messlokation(), art == Zaehlpunktart::Messlokation);
        }
    }

    #[test]
    fn accepts_the_same_grammar_as_a_melo_id() {
        let s = "DE0000000000000000000000000000042";
        assert!(Zaehlpunktbezeichnung::new(s).is_ok());
        assert!(MeloId::new(s).is_ok());
    }

    #[test]
    fn rejects_what_a_melo_id_rejects() {
        for bad in [
            "DE123",                             // too short
            "de0000000000000000000000000000042", // lower-case country code
            "DE000000000000000000000000000004-", // non-alphanumeric body
        ] {
            assert!(Zaehlpunktbezeichnung::new(bad).is_err(), "{bad}");
            assert!(MeloId::new(bad).is_err(), "{bad}");
        }
    }

    /// The conversion exists in both directions, and neither happens implicitly.
    #[test]
    fn conversions_are_explicit_in_both_directions() {
        let melo = MeloId::new("DE0000000000000000000000000000001").unwrap();
        let zpb = Zaehlpunktbezeichnung::from(melo.clone());
        assert_eq!(zpb.as_str(), melo.as_ref());
        assert_eq!(zpb.into_melo_id(), melo);
    }

    #[test]
    fn country_helpers_read_the_prefix() {
        let zpb = Zaehlpunktbezeichnung::new("AT0000000000000000000000000000042").unwrap();
        assert_eq!(zpb.country_code(), "AT");
        assert!(!zpb.is_german());
    }
}

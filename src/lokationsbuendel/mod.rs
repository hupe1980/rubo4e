//! The Lokationsbündelstruktur: what sits behind one Netzanschluss, and where.
//!
//! # There is no `Lokationsbuendel` Geschäftsobjekt
//!
//! BO4E `v202607.1.0`, the release this crate is generated from, defines **no**
//! `Lokationsbuendel` BO, and `BoTyp` has no `LOKATIONSBUENDEL` member. The bundle is modelled instead as one BO plus one
//! field on each participant:
//!
//! | Carries | Where | Meaning |
//! |---|---|---|
//! | the bundle itself | [`Lokationszuordnung`] | the lists of MaLo / MeLo / NeLo / SR / TR that share a Netzanschluss |
//! | *which* structure | `Lokationszuordnung.lokationsbuendelcode` | a [`Lokationsbuendelcode`] |
//! | *where in it* | `<BO>.lokationsbuendelObjektcode` | a [`LokationsbuendelObjektcode`] |
//!
//! The upstream schema says so itself: `Lokationszuordnung` is documented as
//! *"Modell für die Abbildung der Referenz auf die Lokationsbündelstruktur"*.
//! Adding a `Lokationsbuendel` type here would invent a wire format no other BO4E
//! implementation writes, so this module gives the bundle a **name and an API**
//! without giving it a new schema: [`Lokationsbuendel`] is a borrowed *view* over
//! a `Lokationszuordnung`, and it serialises as nothing at all.
//!
//! # What the codes mean
//!
//! Both codes are 13-digit BDEW Codenummern with a §8.1 check digit, published in
//! EDI@Energy's **"Codeliste der Lokationsbündelstrukturen"** (BDEW, version 1.0,
//! 31 March 2023, applicable from 1 October 2024). This module ships the whole
//! list — 15 structures and 27 object codes — as static data:
//!
//! ```
//! use rubo4e::identifiers::{Lokationsbuendelcode, LokationsbuendelObjektcode};
//! use rubo4e::lokationsbuendel::{Flussrichtung, Lokationsbuendelstruktur, Objektrolle, Objekttyp};
//!
//! let code = Lokationsbuendelcode::new("9992000000026").unwrap();
//! let struktur = Lokationsbuendelstruktur::from_code(&code).unwrap();
//! assert_eq!(struktur.bezeichnung, "Verbrauch mit einer Messlokation (Standard)");
//!
//! // Exactly one consumption Marktlokation on level 1, and it is not optional.
//! let malo = struktur.objekt("9992000001016").unwrap();
//! assert!(malo.is_mandatory());
//! assert_eq!(malo.rolle().objekttyp, Objekttyp::Marktlokation);
//! assert_eq!(malo.rolle().richtung, Some(Flussrichtung::Verbrauch));
//!
//! // …and any number of technische Ressourcen hanging off it.
//! let tr = struktur.objekt("9992000001024").unwrap();
//! assert_eq!((tr.min, tr.max), (0, None));
//! ```
//!
//! An object code pins three facts at once — object type, energy-flow direction
//! and level — which is what makes the § 14a EnWG case readable: the *steuerbare
//! Verbrauchseinrichtung* is a `TechnischeRessource` whose code says
//! [`Flussrichtung::Verbrauch`], and the Marktlokation it is billed through is the
//! one on the same level.
//!
//! # Auditing a bundle
//!
//! With the `versioned` feature, [`Lokationszuordnung::audit_buendel`] checks a
//! decoded `Lokationszuordnung` against the codelist: unknown codes, an object
//! filed under the wrong type, a code that is not part of the declared structure,
//! and every cardinality the structure states.
//!
//! ```
//! # #[cfg(feature = "versioned")] {
//! use rubo4e::current::{Lokationszuordnung, Marktlokation};
//! use rubo4e::lokationsbuendel::{Befund, LokationsbuendelExt};
//!
//! let zuordnung = Lokationszuordnung {
//!     lokationsbuendelcode: Some("9992000000026".into()),
//!     // The structure requires exactly one MaLo, one MeLo — and here is neither.
//!     marktlokationen: Some(vec![Box::new(Marktlokation::default())]),
//!     ..Default::default()
//! };
//!
//! let report = zuordnung.audit_buendel();
//! assert!(!report.is_conformant());
//! assert!(report.befunde.iter().any(|b| matches!(b, Befund::ObjektcodeFehlt { .. })));
//! # }
//! ```
//!
//! # Scope: `SteuerbareRessource` is not in the codelist
//!
//! BO4E puts `lokationsbuendelObjektcode` on `SteuerbareRessource` too, but the
//! BDEW codelist covers only NeLo, MeLo, MaLo and TR — chapter 2.1 lists exactly
//! those four. There is therefore no object code that *means* "steuerbare
//! Ressource", and [`audit_buendel`](LokationsbuendelExt::audit_buendel) leaves
//! the `steuerbareRessourcen` list alone rather than reporting every entry as
//! unknown. [`Lokationsbuendel::steuerbare_ressourcen`] still lists them.
//!
//! [`Lokationszuordnung`]: crate::current::Lokationszuordnung
//! [`Lokationszuordnung::audit_buendel`]: LokationsbuendelExt::audit_buendel
//! [`Lokationsbuendelcode`]: crate::identifiers::Lokationsbuendelcode
//! [`LokationsbuendelObjektcode`]: crate::identifiers::LokationsbuendelObjektcode

mod codelist;

#[cfg(feature = "versioned")]
mod bo4e;

pub use codelist::{OBJEKTROLLEN, STRUKTUREN};

#[cfg(feature = "versioned")]
#[cfg_attr(docsrs, doc(cfg(feature = "versioned")))]
pub use bo4e::{
    Befund, Buendelaudit, Lokationsbuendel, LokationsbuendelExt, LokationsbuendelObjekt,
};

use crate::identifiers::{LokationsbuendelObjektcode, Lokationsbuendelcode};

// ─── The four object types a bundle is built from ────────────────────────────

/// The kind of object an [`Objektrolle`] describes.
///
/// Chapter 2.1 of the codelist: a Lokationsbündelstruktur is built from exactly
/// these four. Wandler, Trafo and Tranche are named there as *not* part of it —
/// they travel in UTILMD/UTILTS and do not change the structure.
///
/// Closed: chapter 2.1 enumerates exactly these four and names what is *not*
/// part of a structure (Wandler, Trafo, Tranche). A fifth would be a change to the
/// codelist itself, and an exhaustive `match` here should break when one arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Objekttyp {
    /// Marktlokation (MaLo).
    Marktlokation,
    /// Messlokation (MeLo).
    Messlokation,
    /// Netzlokation (NeLo).
    Netzlokation,
    /// Technische Ressource (TR).
    TechnischeRessource,
}

impl Objekttyp {
    /// The abbreviation the codelist prints — `"MaLo"`, `"MeLo"`, `"NeLo"`, `"TR"`.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Marktlokation => "MaLo",
            Self::Messlokation => "MeLo",
            Self::Netzlokation => "NeLo",
            Self::TechnischeRessource => "TR",
        }
    }
}

impl std::fmt::Display for Objekttyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.abbreviation())
    }
}

/// The energy-flow direction an object code pins.
///
/// Not [`Energierichtung`](crate::current::Energierichtung): that BO4E enum is
/// `EINSP` / `AUSSP` seen from the *grid*, whereas the codelist names the
/// direction from the customer's side and needs a third value for a Messlokation
/// that records both. [`Flussrichtung::as_energierichtung`] converts where the
/// two do line up.
///
/// Closed: the codelist's legend defines exactly these three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flussrichtung {
    /// Verbrauch — the object draws energy.
    Verbrauch,
    /// Erzeugung — the object feeds energy in.
    Erzeugung,
    /// Verbrauch & Erzeugung — a Messlokation recording both, or a storage TR.
    VerbrauchUndErzeugung,
}

/// A qualifier the codelist prints in parentheses beside an object.
///
/// Three of them describe *where a Messlokation measures*, and are what the
/// graphical structures mark `(N)`, `(H)` and `(D)`. The fourth marks a technische
/// Ressource that is a store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Objektfunktion {
    /// `(N)` — Messung an der Netzverbindung (Netzübergabe).
    Netzuebergabe,
    /// `(H)` — Hinterschaltung: measured behind another Messlokation.
    Hinterschaltung,
    /// `(D)` — Differenzmessung.
    Differenzmessung,
    /// `(Speicher)` — a technische Ressource that stores rather than consumes or
    /// generates.
    Speicher,
}

/// Whether a structure fixes the number of objects carrying a code.
///
/// Codelist chapter 2.3: *"Starr"* means the count is prescribed, *"Flexibel"*
/// that it varies from 0 to n — and that the sender must transmit the MeLo → MaLo
/// (and where present MeLo → NeLo) reference, because without it the structure
/// cannot be rebuilt.
///
/// Closed: the column takes exactly these two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flexibilitaet {
    /// The number of objects is prescribed.
    Starr,
    /// The number of objects varies; references are required to resolve them.
    Flexibel,
}

// ─── The object-code catalogue ───────────────────────────────────────────────

/// What one object code *means*, independent of the structure it appears in.
///
/// The codelist assigns object codes "in Abhängigkeit vom Objekt (NeLo, MeLo,
/// MaLo, TR), der Ebene (1, 2, 3), der Richtung" — so a code is a complete
/// coordinate, and `9992 00000 101 6` is the consumption Marktlokation on level 1
/// in every structure that uses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Objektrolle {
    /// The 13-digit object code, in wire form (no spaces).
    pub code: &'static str,
    /// Which of the four object types this code stands for.
    pub objekttyp: Objekttyp,
    /// The energy-flow direction — `None` for a Netzlokation, which the codelist
    /// prints as `--`.
    pub richtung: Option<Flussrichtung>,
    /// The level in the structure: 1, 2 or 3.
    pub ebene: u8,
    /// The parenthesised qualifier, where the codelist prints one.
    pub funktion: Option<Objektfunktion>,
}

impl Objektrolle {
    /// Looks up a validated object code in the catalogue.
    ///
    /// Returns `None` for a code that is well-formed but not published — the
    /// codelist's introduction says complex or special structures are exchanged
    /// bilaterally rather than coded.
    #[must_use]
    pub fn from_code(code: &LokationsbuendelObjektcode) -> Option<&'static Self> {
        Self::from_wire(code.as_str())
    }

    /// Looks up an object code given as a bare string.
    ///
    /// Does **not** check the check digit; [`from_code`](Self::from_code) is the
    /// entry point that does. A string that is not in the catalogue yields `None`
    /// either way.
    #[must_use]
    pub fn from_wire(code: &str) -> Option<&'static Self> {
        OBJEKTROLLEN
            .binary_search_by_key(&code, |r| r.code)
            .ok()
            .map(|i| &OBJEKTROLLEN[i])
    }

    /// The code as a validated [`LokationsbuendelObjektcode`].
    ///
    /// Infallible in practice — every catalogued code satisfies the check digit,
    /// and `tests` pins that — but it allocates, so prefer [`code`](Self::code)
    /// when a `&str` will do.
    ///
    /// # Panics
    /// Never, for a value obtained from this module.
    #[must_use]
    pub fn as_objektcode(&self) -> LokationsbuendelObjektcode {
        LokationsbuendelObjektcode::new(self.code)
            .expect("catalogued object codes carry a valid BDEW check digit")
    }

    /// `true` if this code stands for a technische Ressource that draws energy —
    /// the shape a § 14a EnWG *steuerbare Verbrauchseinrichtung* takes in a
    /// bundle.
    #[must_use]
    pub const fn is_verbrauchs_tr(&self) -> bool {
        matches!(self.objekttyp, Objekttyp::TechnischeRessource)
            && matches!(self.richtung, Some(Flussrichtung::Verbrauch))
    }
}

// ─── One row of one structure ────────────────────────────────────────────────

/// One row of a structure's table: an object code, and how often it may appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct StrukturObjekt {
    /// The object code this row constrains.
    pub code: &'static str,
    /// Whether the structure fixes the count.
    pub flexibilitaet: Flexibilitaet,
    /// Smallest permitted number of objects carrying this code.
    pub min: u32,
    /// Largest permitted number, or `None` for the codelist's `N`.
    pub max: Option<u32>,
    /// Object codes of the Messlokation(en) a flexible object must reference.
    ///
    /// Empty where the codelist prints `--`. More than one entry means the
    /// reference may point at either, as in "Summenmessung mit mindestens einer
    /// ungemessenen Erzeugung", where the single MaLo references both the
    /// consumption and the bidirectional Messlokation.
    pub referenz_messlokation: &'static [&'static str],
    /// Object code of the Netzlokation a flexible object must reference.
    ///
    /// Only the three Summenmessung structures use this column.
    pub referenz_netzlokation: &'static [&'static str],
}

impl StrukturObjekt {
    /// What the code means — object type, direction, level.
    ///
    /// # Panics
    /// Never: every structure row names a catalogued code, and the tests in this
    /// module pin that.
    #[must_use]
    pub fn rolle(&self) -> &'static Objektrolle {
        Objektrolle::from_wire(self.code)
            .expect("every structure row names a catalogued object code")
    }

    /// `true` if the structure requires at least one object with this code.
    #[must_use]
    pub const fn is_mandatory(&self) -> bool {
        self.min > 0
    }

    /// `true` if `count` objects with this code satisfy the structure.
    #[must_use]
    pub const fn permits(&self, count: u32) -> bool {
        count >= self.min
            && match self.max {
                Some(max) => count <= max,
                None => true,
            }
    }

    /// The cardinality in the codelist's own notation — `"1"`, `"0-1"`, `"0-N"`,
    /// `"≥1"`.
    #[must_use]
    pub fn cardinality(&self) -> String {
        match (self.min, self.max) {
            (min, Some(max)) if min == max => min.to_string(),
            (min, Some(max)) => format!("{min}-{max}"),
            (0, None) => "0-N".to_string(),
            (min, None) => format!("≥{min}"),
        }
    }
}

// ─── One structure ───────────────────────────────────────────────────────────

/// One published Lokationsbündelstruktur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Lokationsbuendelstruktur {
    /// The 13-digit structure code, in wire form (no spaces).
    pub code: &'static str,
    /// The German name the codelist gives it.
    pub bezeichnung: &'static str,
    /// The objects it is built from, in the codelist's own row order.
    pub objekte: &'static [StrukturObjekt],
}

impl Lokationsbuendelstruktur {
    /// Looks up a validated structure code.
    ///
    /// Returns `None` for a well-formed code outside the published list.
    #[must_use]
    pub fn from_code(code: &Lokationsbuendelcode) -> Option<&'static Self> {
        Self::from_wire(code.as_str())
    }

    /// Looks up a structure code given as a bare string. Does not check the
    /// check digit.
    #[must_use]
    pub fn from_wire(code: &str) -> Option<&'static Self> {
        STRUKTUREN
            .binary_search_by_key(&code, |s| s.code)
            .ok()
            .map(|i| &STRUKTUREN[i])
    }

    /// The code as a validated [`Lokationsbuendelcode`].
    ///
    /// # Panics
    /// Never, for a value obtained from this module.
    #[must_use]
    pub fn as_code(&self) -> Lokationsbuendelcode {
        Lokationsbuendelcode::new(self.code)
            .expect("catalogued structure codes carry a valid BDEW check digit")
    }

    /// The row for `code`, or `None` if this structure does not use it.
    #[must_use]
    pub fn objekt(&self, code: &str) -> Option<&'static StrukturObjekt> {
        self.objekte.iter().find(|o| o.code == code)
    }

    /// Every row whose object code stands for `typ`.
    pub fn objekte_of(&self, typ: Objekttyp) -> impl Iterator<Item = &'static StrukturObjekt> {
        self.objekte
            .iter()
            .filter(move |o| o.rolle().objekttyp == typ)
    }

    /// The deepest level any of this structure's objects sits on — 1, 2 or 3.
    #[must_use]
    pub fn max_ebene(&self) -> u8 {
        self.objekte
            .iter()
            .map(|o| o.rolle().ebene)
            .max()
            .unwrap_or(1)
    }
}

impl std::fmt::Display for Lokationsbuendelstruktur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.bezeichnung, self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogues_are_sorted_for_binary_search() {
        assert!(OBJEKTROLLEN.windows(2).all(|w| w[0].code < w[1].code));
        assert!(STRUKTUREN.windows(2).all(|w| w[0].code < w[1].code));
    }

    #[test]
    fn the_codelist_ships_complete() {
        assert_eq!(
            STRUKTUREN.len(),
            15,
            "codelist v1.0 publishes 15 structures"
        );
        assert_eq!(
            OBJEKTROLLEN.len(),
            27,
            "codelist v1.0 publishes 27 object codes"
        );
    }

    /// Every row of every structure names a code the catalogue defines, and every
    /// catalogued code is used by at least one structure.
    #[test]
    fn structures_and_object_codes_agree() {
        let mut used = std::collections::BTreeSet::new();
        for s in STRUKTUREN {
            for o in s.objekte {
                assert!(
                    Objektrolle::from_wire(o.code).is_some(),
                    "{} references uncatalogued object code {}",
                    s.code,
                    o.code
                );
                used.insert(o.code);
            }
        }
        for r in OBJEKTROLLEN {
            assert!(
                used.contains(&r.code),
                "{} is catalogued but unused",
                r.code
            );
        }
    }

    /// A structure's reference columns may only name object codes that structure
    /// itself contains.
    #[test]
    fn references_stay_inside_their_structure() {
        for s in STRUKTUREN {
            for o in s.objekte {
                for r in o
                    .referenz_messlokation
                    .iter()
                    .chain(o.referenz_netzlokation)
                {
                    let target = s.objekt(r).unwrap_or_else(|| {
                        panic!(
                            "{}: {} references {r}, which is not in the structure",
                            s.code, o.code
                        )
                    });
                    let expected = if o.referenz_messlokation.contains(r) {
                        Objekttyp::Messlokation
                    } else {
                        Objekttyp::Netzlokation
                    };
                    assert_eq!(target.rolle().objekttyp, expected, "{}: {r}", s.code);
                }
            }
        }
    }

    /// The round trip a caller actually makes: wire string → validated code →
    /// meaning.
    #[test]
    fn resolves_the_standard_structure() {
        let code = Lokationsbuendelcode::new("9992000000026").unwrap();
        let s = Lokationsbuendelstruktur::from_code(&code).unwrap();
        assert_eq!(s.bezeichnung, "Verbrauch mit einer Messlokation (Standard)");
        assert_eq!(s.max_ebene(), 1);

        let melo = s.objekt("9992000001032").unwrap();
        assert_eq!(melo.flexibilitaet, Flexibilitaet::Starr);
        assert_eq!(melo.cardinality(), "1");
        assert_eq!(melo.rolle().funktion, Some(Objektfunktion::Netzuebergabe));

        let nelo = s.objekt("9992000001256").unwrap();
        assert_eq!(nelo.cardinality(), "0-1");
        assert_eq!(nelo.rolle().richtung, None);

        let tr = s.objekt("9992000001024").unwrap();
        assert_eq!(tr.cardinality(), "0-N");
        assert!(tr.rolle().is_verbrauchs_tr());
        assert!(!tr.is_mandatory());
        assert!(tr.permits(0) && tr.permits(9_999));
    }

    /// The cascade structures are the ones with a level 3, and the ones a heat-pump
    /// cascade under § 14a is exchanged as.
    #[test]
    fn cascade_structure_reaches_level_three() {
        let s = Lokationsbuendelstruktur::from_wire("9992000000183").unwrap();
        assert_eq!(s.max_ebene(), 3);
        assert_eq!(
            s.objekte_of(Objekttyp::Marktlokation).count(),
            4,
            "levels 1, 2 and 3 (consumption) plus level 3 generation"
        );
    }

    #[test]
    fn unpublished_codes_resolve_to_none() {
        // Valid check digit, not in the list.
        let code = Lokationsbuendelcode::from_base("999200000999").unwrap();
        assert!(Lokationsbuendelstruktur::from_code(&code).is_none());
    }

    #[test]
    fn cardinality_spellings_match_the_codelist() {
        let s = Lokationsbuendelstruktur::from_wire("9992000000109").unwrap();
        // "Summenmessung Verbrauch ohne Erzeugung": ≥2 Messlokationen.
        assert_eq!(s.objekt("9992000001032").unwrap().cardinality(), "≥2");
        assert_eq!(
            s.objekt("9992000001032").unwrap().referenz_netzlokation,
            ["9992000001256"]
        );
    }
}

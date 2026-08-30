//! The bridge from the codelist onto the generated BO4E types.
//!
//! Everything here is gated on `versioned`, because it reads
//! [`Lokationszuordnung`] and the five Geschäftsobjekte that carry a
//! `lokationsbuendelObjektcode`.

use super::{Flussrichtung, Lokationsbuendelstruktur, Objektrolle, Objekttyp};
use crate::current::{
    Energierichtung, Lokationszuordnung, Marktlokation, Messlokation, Netzlokation,
    SteuerbareRessource, TechnischeRessource,
};
use crate::error::IdentifierError;
use crate::identifiers::{LokationsbuendelObjektcode, Lokationsbuendelcode};

// ─── The codelist's direction, as BO4E's ─────────────────────────────────────

impl Flussrichtung {
    /// This direction as BO4E's [`Energierichtung`], where the two line up.
    ///
    /// They are not the same enum, and not for a cosmetic reason.
    /// `Energierichtung` is stated **from the grid's side** — `AUSSP`
    /// (Ausspeisung) is energy leaving the grid, which is the customer consuming
    /// it — and has exactly two values. The codelist names the direction from the
    /// customer's side and needs a third, because a Messlokation at the
    /// Netzübergabe records both.
    ///
    /// So the mapping is total in one direction only:
    ///
    /// | `Flussrichtung` | `Energierichtung` |
    /// |---|---|
    /// | `Verbrauch` | `Ausspeisung` — out of the grid |
    /// | `Erzeugung` | `Einspeisung` — into the grid |
    /// | `VerbrauchUndErzeugung` | `None` — BO4E has no value for both |
    ///
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::current::Energierichtung;
    /// use rubo4e::lokationsbuendel::Flussrichtung;
    ///
    /// assert_eq!(Flussrichtung::Verbrauch.as_energierichtung(), Some(Energierichtung::Aussp));
    /// assert_eq!(Flussrichtung::Erzeugung.as_energierichtung(), Some(Energierichtung::Einsp));
    /// assert_eq!(Flussrichtung::VerbrauchUndErzeugung.as_energierichtung(), None);
    /// # }
    /// ```
    #[must_use]
    pub const fn as_energierichtung(self) -> Option<Energierichtung> {
        match self {
            Self::Verbrauch => Some(Energierichtung::Aussp),
            Self::Erzeugung => Some(Energierichtung::Einsp),
            Self::VerbrauchUndErzeugung => None,
        }
    }
}

// ─── One participant in a bundle ─────────────────────────────────────────────

/// Implemented by the five Geschäftsobjekte that carry a
/// `lokationsbuendelObjektcode`.
///
/// The field is a plain `String` on the generated struct — deliberately. A
/// newtype there would make one mistyped code fail the deserialization of the
/// whole `Marktlokation` it sits on, which is the same trade
/// [`Zahlungsinformation::iban_checked`] settles the same way. These accessors
/// run the check on demand instead, and cost the caller one `Result`.
///
/// ```
/// # #[cfg(feature = "versioned")] {
/// use rubo4e::current::TechnischeRessource;
/// use rubo4e::lokationsbuendel::{Flussrichtung, LokationsbuendelObjekt, Objekttyp};
///
/// let tr = TechnischeRessource {
///     lokationsbuendel_objektcode: Some("9992000001024".into()),
///     ..Default::default()
/// };
///
/// let rolle = tr.objektrolle().unwrap();
/// assert_eq!(rolle.objekttyp, Objekttyp::TechnischeRessource);
/// assert_eq!(rolle.richtung, Some(Flussrichtung::Verbrauch));
/// assert_eq!(rolle.ebene, 1);
/// # }
/// ```
///
/// [`Zahlungsinformation::iban_checked`]: crate::current::Zahlungsinformation::iban_checked
pub trait LokationsbuendelObjekt {
    /// The raw `lokationsbuendelObjektcode` field.
    fn lokationsbuendel_objektcode_raw(&self) -> Option<&str>;

    /// Which of the four codelist object types this Rust type *is*.
    ///
    /// `None` for [`SteuerbareRessource`]: the BDEW codelist covers NeLo, MeLo,
    /// MaLo and TR only, so there is no object code that means "steuerbare
    /// Ressource". See the module docs.
    fn codelist_objekttyp() -> Option<Objekttyp>
    where
        Self: Sized;

    /// The object code, validated.
    ///
    /// `None` when the field is absent; `Some(Err(_))` when it is present but not
    /// a 13-digit BDEW code with a correct §8.1 check digit.
    fn lokationsbuendel_objektcode(
        &self,
    ) -> Option<Result<LokationsbuendelObjektcode, IdentifierError>> {
        self.lokationsbuendel_objektcode_raw()
            .map(LokationsbuendelObjektcode::new)
    }

    /// What the object code means — object type, direction, level.
    ///
    /// `None` when the field is absent, malformed, or holds a code outside the
    /// published catalogue.
    fn objektrolle(&self) -> Option<&'static Objektrolle> {
        Objektrolle::from_wire(self.lokationsbuendel_objektcode_raw()?)
    }

    /// The level (1–3) this object sits on, per its object code.
    fn ebene(&self) -> Option<u8> {
        self.objektrolle().map(|r| r.ebene)
    }

    /// The energy-flow direction its object code pins.
    fn flussrichtung(&self) -> Option<Flussrichtung> {
        self.objektrolle()?.richtung
    }
}

macro_rules! impl_lokationsbuendel_objekt {
    ($ty:ty, $typ:expr) => {
        impl LokationsbuendelObjekt for $ty {
            fn lokationsbuendel_objektcode_raw(&self) -> Option<&str> {
                self.lokationsbuendel_objektcode.as_deref()
            }
            fn codelist_objekttyp() -> Option<Objekttyp> {
                $typ
            }
        }
    };
}

impl_lokationsbuendel_objekt!(Marktlokation, Some(Objekttyp::Marktlokation));
impl_lokationsbuendel_objekt!(Messlokation, Some(Objekttyp::Messlokation));
impl_lokationsbuendel_objekt!(Netzlokation, Some(Objekttyp::Netzlokation));
impl_lokationsbuendel_objekt!(TechnischeRessource, Some(Objekttyp::TechnischeRessource));
impl_lokationsbuendel_objekt!(SteuerbareRessource, None);

// ─── The bundle view ─────────────────────────────────────────────────────────

/// A borrowed view of the Lokationsbündel a [`Lokationszuordnung`] describes.
///
/// Not a Geschäftsobjekt and not serialisable: BO4E defines no `Lokationsbuendel`
/// schema, and inventing one here would produce a payload no other implementation
/// reads. This is the name for the thing the `Lokationszuordnung` already is.
#[derive(Debug, Clone, Copy)]
pub struct Lokationsbuendel<'a> {
    zuordnung: &'a Lokationszuordnung,
}

impl<'a> Lokationsbuendel<'a> {
    /// The `Lokationszuordnung` this view reads.
    #[must_use]
    pub const fn zuordnung(&self) -> &'a Lokationszuordnung {
        self.zuordnung
    }

    /// The declared structure code, validated.
    ///
    /// `None` when `lokationsbuendelcode` is absent.
    #[must_use]
    pub fn code(&self) -> Option<Result<Lokationsbuendelcode, IdentifierError>> {
        self.zuordnung
            .lokationsbuendelcode
            .as_deref()
            .map(Lokationsbuendelcode::new)
    }

    /// The structure the declared code names, if it is one the codelist publishes.
    #[must_use]
    pub fn struktur(&self) -> Option<&'static Lokationsbuendelstruktur> {
        Lokationsbuendelstruktur::from_wire(self.zuordnung.lokationsbuendelcode.as_deref()?)
    }

    /// The Marktlokationen in the bundle.
    pub fn marktlokationen(&self) -> impl Iterator<Item = &'a Marktlokation> {
        flatten(self.zuordnung.marktlokationen.as_deref())
    }

    /// The Messlokationen in the bundle.
    pub fn messlokationen(&self) -> impl Iterator<Item = &'a Messlokation> {
        flatten(self.zuordnung.messlokationen.as_deref())
    }

    /// The Netzlokationen in the bundle.
    pub fn netzlokationen(&self) -> impl Iterator<Item = &'a Netzlokation> {
        flatten(self.zuordnung.netzlokationen.as_deref())
    }

    /// The technische Ressourcen in the bundle.
    pub fn technische_ressourcen(&self) -> impl Iterator<Item = &'a TechnischeRessource> {
        flatten(self.zuordnung.technische_ressourcen.as_deref())
    }

    /// The steuerbare Ressourcen in the bundle.
    ///
    /// Listed, but outside every codelist check — see the module docs.
    pub fn steuerbare_ressourcen(&self) -> impl Iterator<Item = &'a SteuerbareRessource> {
        flatten(self.zuordnung.steuerbare_ressourcen.as_deref())
    }

    /// How many objects the bundle holds, across all five lists.
    #[must_use]
    pub fn len(&self) -> usize {
        count(self.zuordnung.marktlokationen.as_deref())
            + count(self.zuordnung.messlokationen.as_deref())
            + count(self.zuordnung.netzlokationen.as_deref())
            + count(self.zuordnung.technische_ressourcen.as_deref())
            + count(self.zuordnung.steuerbare_ressourcen.as_deref())
    }

    /// `true` if the bundle holds no objects at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The technische Ressourcen whose object code marks them as consuming — the
    /// shape a § 14a EnWG *steuerbare Verbrauchseinrichtung* takes in a bundle.
    ///
    /// A heat pump, a wallbox and a storage heater all land here; a PV inverter
    /// does not, and a battery (`Verbrauch & Erzeugung (Speicher)`) does not
    /// either. Pair it with `TechnischeRessource::technische_ressource_nutzung`
    /// and `zugeordnete_steuerbare_ressource_id` to reach the SR that steers it.
    pub fn verbrauchs_ressourcen(&self) -> impl Iterator<Item = &'a TechnischeRessource> {
        self.technische_ressourcen()
            .filter(|tr| tr.objektrolle().is_some_and(Objektrolle::is_verbrauchs_tr))
    }

    /// Every object in the bundle that sits on `ebene`, as `(Objekttyp, code)`.
    ///
    /// Objects with no object code, or one outside the catalogue, are skipped —
    /// their level is not knowable.
    pub fn objekte_auf_ebene(&self, ebene: u8) -> Vec<(Objekttyp, &'a str)> {
        let mut out = Vec::new();
        collect_ebene(self.marktlokationen(), ebene, &mut out);
        collect_ebene(self.messlokationen(), ebene, &mut out);
        collect_ebene(self.netzlokationen(), ebene, &mut out);
        collect_ebene(self.technische_ressourcen(), ebene, &mut out);
        out
    }
}

fn collect_ebene<'a, T: LokationsbuendelObjekt + 'a>(
    items: impl Iterator<Item = &'a T>,
    ebene: u8,
    out: &mut Vec<(Objekttyp, &'a str)>,
) {
    for item in items {
        if let (Some(rolle), Some(code)) =
            (item.objektrolle(), item.lokationsbuendel_objektcode_raw())
        {
            if rolle.ebene == ebene {
                out.push((rolle.objekttyp, code));
            }
        }
    }
}

fn flatten<T>(items: Option<&[Box<T>]>) -> impl Iterator<Item = &T> {
    items.unwrap_or(&[]).iter().map(std::convert::AsRef::as_ref)
}

fn count<T>(items: Option<&[Box<T>]>) -> usize {
    items.map_or(0, <[Box<T>]>::len)
}

// ─── The audit ───────────────────────────────────────────────────────────────

/// One way a `Lokationszuordnung` departs from the structure it declares.
///
/// A data-quality finding, not a schema violation: BO4E requires none of this, so
/// nothing here is wired into `.validate()` — the same line
/// [`timeseries`](crate::timeseries) draws for a gappy Lastgang.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Befund {
    /// `lokationsbuendelcode` is absent, so nothing can be checked against it.
    StrukturcodeFehlt,
    /// `lokationsbuendelcode` is not a 13-digit BDEW code with a valid check
    /// digit.
    StrukturcodeUngueltig {
        /// The value as it stood in the payload.
        code: String,
        /// Why it failed.
        fehler: IdentifierError,
    },
    /// The code is well-formed but not one of the 15 published structures.
    StrukturUnbekannt {
        /// The declared structure code.
        code: String,
    },
    /// An object carries no `lokationsbuendelObjektcode`, so its place in the
    /// structure is unstated.
    ObjektcodeFehlt {
        /// Which list the object came from.
        objekttyp: Objekttyp,
        /// Its index in that list.
        index: usize,
    },
    /// An object's code is not a 13-digit BDEW code with a valid check digit.
    ObjektcodeUngueltig {
        /// Which list the object came from.
        objekttyp: Objekttyp,
        /// Its index in that list.
        index: usize,
        /// The value as it stood in the payload.
        code: String,
        /// Why it failed.
        fehler: IdentifierError,
    },
    /// An object's code is well-formed but not in the published catalogue.
    ObjektcodeUnbekannt {
        /// Which list the object came from.
        objekttyp: Objekttyp,
        /// Its index in that list.
        index: usize,
        /// The declared object code.
        code: String,
    },
    /// An object's code belongs to a different object type than the list it was
    /// filed under — a Marktlokation carrying a Messlokation's code.
    ObjekttypWiderspruch {
        /// The list the object was filed under.
        gefunden: Objekttyp,
        /// Its index in that list.
        index: usize,
        /// The object type the code stands for.
        erwartet: Objekttyp,
        /// The declared object code.
        code: String,
    },
    /// An object's code is catalogued, but the declared structure does not use it.
    ObjektcodeNichtInStruktur {
        /// Which list the object came from.
        objekttyp: Objekttyp,
        /// Its index in that list.
        index: usize,
        /// The declared object code.
        code: String,
    },
    /// The number of objects carrying one code is outside what the structure
    /// permits — including zero, where the structure requires at least one.
    AnzahlVerletzt {
        /// The object code whose count is wrong.
        code: String,
        /// What the code stands for.
        objekttyp: Objekttyp,
        /// How many the payload carried.
        gefunden: u32,
        /// The cardinality the structure states, in its own notation.
        erwartet: String,
    },
}

impl std::fmt::Display for Befund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrukturcodeFehlt => f.write_str("lokationsbuendelcode is absent"),
            Self::StrukturcodeUngueltig { code, fehler } => {
                write!(f, "lokationsbuendelcode \"{code}\" is invalid: {fehler}")
            }
            Self::StrukturUnbekannt { code } => {
                write!(
                    f,
                    "lokationsbuendelcode {code} is not a published structure"
                )
            }
            Self::ObjektcodeFehlt { objekttyp, index } => {
                write!(
                    f,
                    "{objekttyp}[{index}] carries no lokationsbuendelObjektcode"
                )
            }
            Self::ObjektcodeUngueltig {
                objekttyp,
                index,
                code,
                fehler,
            } => write!(
                f,
                "{objekttyp}[{index}] object code \"{code}\" is invalid: {fehler}"
            ),
            Self::ObjektcodeUnbekannt {
                objekttyp,
                index,
                code,
            } => write!(
                f,
                "{objekttyp}[{index}] object code {code} is not catalogued"
            ),
            Self::ObjekttypWiderspruch {
                gefunden,
                index,
                erwartet,
                code,
            } => write!(
                f,
                "{gefunden}[{index}] carries {code}, which is a {erwartet} object code"
            ),
            Self::ObjektcodeNichtInStruktur {
                objekttyp,
                index,
                code,
            } => write!(
                f,
                "{objekttyp}[{index}] object code {code} is not part of the declared structure"
            ),
            Self::AnzahlVerletzt {
                code,
                objekttyp,
                gefunden,
                erwartet,
            } => write!(
                f,
                "{objekttyp} {code}: found {gefunden}, structure states {erwartet}"
            ),
        }
    }
}

/// What [`audit_buendel`](LokationsbuendelExt::audit_buendel) found.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct Buendelaudit {
    /// The structure the payload declared, where it named a published one.
    pub struktur: Option<&'static Lokationsbuendelstruktur>,
    /// Every departure found, in the order the audit walks: structure code
    /// first, then the objects list by list, then the cardinalities.
    pub befunde: Vec<Befund>,
}

impl Buendelaudit {
    /// `true` if nothing was found.
    #[must_use]
    pub fn is_conformant(&self) -> bool {
        self.befunde.is_empty()
    }
}

impl std::fmt::Display for Buendelaudit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.struktur {
            Some(s) => writeln!(f, "{s}")?,
            None => writeln!(f, "<no published structure>")?,
        }
        if self.befunde.is_empty() {
            return f.write_str("conformant");
        }
        for (i, b) in self.befunde.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "- {b}")?;
        }
        Ok(())
    }
}

/// Reads a [`Lokationszuordnung`] as the Lokationsbündel it describes.
///
/// See the [module docs](crate::lokationsbuendel) for why this is an extension
/// trait rather than a `Lokationsbuendel` Geschäftsobjekt.
pub trait LokationsbuendelExt {
    /// A borrowed view of the bundle.
    fn buendel(&self) -> Lokationsbuendel<'_>;

    /// The declared structure code, validated.
    fn lokationsbuendelcode(&self) -> Option<Result<Lokationsbuendelcode, IdentifierError>>;

    /// The published structure the declared code names, if any.
    fn lokationsbuendelstruktur(&self) -> Option<&'static Lokationsbuendelstruktur>;

    /// Checks the bundle against the structure it declares.
    ///
    /// Walks the structure code, then every Marktlokation, Messlokation,
    /// Netzlokation and technische Ressource, then the cardinalities the
    /// structure states. `steuerbareRessourcen` are left alone — the BDEW
    /// codelist has no object codes for them.
    ///
    /// Where the structure code is missing or unpublished, the object-level
    /// checks that do not need it (a malformed code, a code filed under the wrong
    /// type) still run, so a report is never empty of everything it could say.
    fn audit_buendel(&self) -> Buendelaudit;
}

impl LokationsbuendelExt for Lokationszuordnung {
    fn buendel(&self) -> Lokationsbuendel<'_> {
        Lokationsbuendel { zuordnung: self }
    }

    fn lokationsbuendelcode(&self) -> Option<Result<Lokationsbuendelcode, IdentifierError>> {
        self.buendel().code()
    }

    fn lokationsbuendelstruktur(&self) -> Option<&'static Lokationsbuendelstruktur> {
        self.buendel().struktur()
    }

    fn audit_buendel(&self) -> Buendelaudit {
        let mut befunde = Vec::new();

        // ── The structure code ───────────────────────────────────────────────
        let struktur = match self.lokationsbuendelcode.as_deref() {
            None => {
                befunde.push(Befund::StrukturcodeFehlt);
                None
            }
            Some(raw) => match Lokationsbuendelcode::new(raw) {
                Err(fehler) => {
                    befunde.push(Befund::StrukturcodeUngueltig {
                        code: raw.to_owned(),
                        fehler,
                    });
                    None
                }
                Ok(code) => match Lokationsbuendelstruktur::from_code(&code) {
                    None => {
                        befunde.push(Befund::StrukturUnbekannt {
                            code: raw.to_owned(),
                        });
                        None
                    }
                    some => some,
                },
            },
        };

        // ── The objects, list by list ────────────────────────────────────────
        //
        // `counts` accumulates only codes that survived every per-object check,
        // so a malformed code is reported once rather than again as a shortfall.
        let mut counts: Vec<(&'static str, u32)> = Vec::new();

        macro_rules! walk {
            ($items:expr, $typ:expr) => {
                for (index, item) in $items.enumerate() {
                    audit_objekt(item, $typ, index, struktur, &mut counts, &mut befunde);
                }
            };
        }
        let b = self.buendel();
        walk!(b.marktlokationen(), Objekttyp::Marktlokation);
        walk!(b.messlokationen(), Objekttyp::Messlokation);
        walk!(b.netzlokationen(), Objekttyp::Netzlokation);
        walk!(b.technische_ressourcen(), Objekttyp::TechnischeRessource);

        // ── The cardinalities ────────────────────────────────────────────────
        if let Some(s) = struktur {
            for row in s.objekte {
                let found = counts
                    .iter()
                    .find(|(c, _)| *c == row.code)
                    .map_or(0, |(_, n)| *n);
                if !row.permits(found) {
                    befunde.push(Befund::AnzahlVerletzt {
                        code: row.code.to_owned(),
                        objekttyp: row.rolle().objekttyp,
                        gefunden: found,
                        erwartet: row.cardinality(),
                    });
                }
            }
        }

        Buendelaudit { struktur, befunde }
    }
}

/// Checks one object, recording its code for the cardinality pass if it passes.
fn audit_objekt<T: LokationsbuendelObjekt>(
    item: &T,
    objekttyp: Objekttyp,
    index: usize,
    struktur: Option<&'static Lokationsbuendelstruktur>,
    counts: &mut Vec<(&'static str, u32)>,
    befunde: &mut Vec<Befund>,
) {
    let Some(raw) = item.lokationsbuendel_objektcode_raw() else {
        befunde.push(Befund::ObjektcodeFehlt { objekttyp, index });
        return;
    };
    if let Err(fehler) = LokationsbuendelObjektcode::new(raw) {
        befunde.push(Befund::ObjektcodeUngueltig {
            objekttyp,
            index,
            code: raw.to_owned(),
            fehler,
        });
        return;
    }
    let Some(rolle) = Objektrolle::from_wire(raw) else {
        befunde.push(Befund::ObjektcodeUnbekannt {
            objekttyp,
            index,
            code: raw.to_owned(),
        });
        return;
    };
    if rolle.objekttyp != objekttyp {
        befunde.push(Befund::ObjekttypWiderspruch {
            gefunden: objekttyp,
            index,
            erwartet: rolle.objekttyp,
            code: raw.to_owned(),
        });
        return;
    }
    if let Some(s) = struktur {
        if s.objekt(raw).is_none() {
            befunde.push(Befund::ObjektcodeNichtInStruktur {
                objekttyp,
                index,
                code: raw.to_owned(),
            });
            return;
        }
    }
    match counts.iter_mut().find(|(c, _)| *c == rolle.code) {
        Some((_, n)) => *n += 1,
        None => counts.push((rolle.code, 1)),
    }
}

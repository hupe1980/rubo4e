//! Namespaced `ZusatzAttribut`s: the supported way to carry what BO4E does not
//! model.
//!
//! BO4E gives every Geschäftsobjekt and component a `zusatzAttribute` list —
//! *"Beim Austausch von Datenobjekten zwischen verschiedenen Systemen ist es daher
//! hilfreich, sich die eindeutigen IDs der anzubindenden Systeme zu merken"* — and
//! then says nothing about how two systems writing into the same list stay out of
//! each other's way. This module supplies the missing half: a `namespace:key`
//! convention, a registry of claimed prefixes, and typed accessors over it.
//!
//! ```
//! # #[cfg(all(feature = "versioned", feature = "json"))] {
//! use rubo4e::current::SteuerbareRessource;
//! use rubo4e::zusatz_attribut::{Namespace, ZusatzAttributeExt};
//!
//! let mut sr = SteuerbareRessource::default();
//!
//! // BO4E has no slot for an EEBUS SKI, so it travels in a namespace.
//! sr.set_zusatz_attribut_in(
//!     &Namespace::HEMS,
//!     "eebus-ski",
//!     "d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0",
//! );
//!
//! assert_eq!(
//!     sr.zusatz_attribut_str_in(&Namespace::HEMS, "eebus-ski"),
//!     Some("d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0"),
//! );
//!
//! // The wire form is the flat BO4E name, so any BO4E reader still sees it.
//! let names: Vec<&str> = sr.zusatz_attribute().filter_map(|a| a.name.as_deref()).collect();
//! assert_eq!(names, ["hems:eebus-ski"]);
//! # }
//! ```
//!
//! # Why a namespace and not a field
//!
//! Anything you put in `zusatzAttribute` is, by construction, outside the
//! standard. Two facts follow, and both are the reason this module exists rather
//! than a set of new struct fields:
//!
//! 1. **A receiver may drop it.** BO4E states no obligation to round-trip
//!    `zusatzAttribute`, so treat a namespaced value as a hint you re-derive, not
//!    as the system of record.
//! 2. **A collision is silent.** `"id"` written by two systems is one entry, and
//!    the second write wins. `"mako:id"` and `"hems:id"` are two.
//!
//! Where BO4E *does* model something, use the field: a control channel's
//! characteristic is [`SteuerkanalLeistungsbeschreibung`], not a namespaced
//! string. Check first — the *Beyond the Schema* guide lists what looks missing
//! and is not.
//!
//! Defining the **values** is the domain crate's job, not this one's; `rubo4e`
//! would be inventing code lists the market has not published.
//! [`set_zusatz_attribut_as`](ZusatzAttributeExt::set_zusatz_attribut_as) stores
//! any `Serialize`, so an enum defined in *your* crate round-trips with no
//! stringly-typed step.
//!
//! [`SteuerkanalLeistungsbeschreibung`]: crate::current::SteuerkanalLeistungsbeschreibung

use std::borrow::Cow;
use std::marker::PhantomData;

use crate::current::ZusatzAttribut;

/// The character that separates a namespace from a key.
pub const SEPARATOR: char = ':';

// ─── Namespaces ──────────────────────────────────────────────────────────────

/// Why a namespace prefix was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NamespaceError {
    /// The prefix was empty.
    Empty,
    /// The prefix contained the separator, which would make the split ambiguous.
    ContainsSeparator,
    /// The prefix contained a character outside `[A-Za-z0-9_-]`.
    InvalidCharacter {
        /// Byte offset of the offending character.
        position: usize,
        /// The character itself.
        character: char,
    },
}

impl std::fmt::Display for NamespaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("a namespace prefix may not be empty"),
            Self::ContainsSeparator => write!(
                f,
                "a namespace prefix may not contain the separator '{SEPARATOR}'"
            ),
            Self::InvalidCharacter {
                position,
                character,
            } => write!(
                f,
                "invalid character '{character}' at position {position}; \
                 a namespace prefix is [A-Za-z0-9_-]+"
            ),
        }
    }
}

impl std::error::Error for NamespaceError {}

/// A reserved prefix for `ZusatzAttribut` names — the `mako` in `"mako:vorgangsnummer"`.
///
/// Two systems writing into the same `zusatzAttribute` list collide on any name
/// they both choose. A namespace makes that impossible by construction, and makes
/// "everything this system wrote" a query rather than a guess.
///
/// # Registered prefixes
///
/// [`REGISTERED`](Namespace::REGISTERED) lists the ones the rubo4e ecosystem has
/// claimed. It is a convention, not an enforcement: [`Namespace::new`] takes any
/// well-formed prefix, and [`is_registered`](Namespace::is_registered) says
/// whether a given one is on the list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(Cow<'static, str>);

impl Namespace {
    /// `mako:` — market-communication projections (EDIFACT/AS4 vorgang metadata,
    /// message references, sender and receiver context).
    pub const MAKO: Self = Self(Cow::Borrowed("mako"));

    /// `hems:` — home energy management: device keys such as an EEBUS SKI, the
    /// § 14a EnWG Steuerungsvariante, and anything else the household model knows
    /// that BO4E does not.
    pub const HEMS: Self = Self(Cow::Borrowed("hems"));

    /// `edmd:` — Energiedaten-Management: series provenance, replacement-value
    /// procedure, and the identifiers a metering back end keys on.
    pub const EDMD: Self = Self(Cow::Borrowed("edmd"));

    /// `mabis:` — settlement facts from the Marktregeln für die
    /// Bilanzkreisabrechnung Strom that BO4E has no field for.
    ///
    /// The keys this crate itself registers live here — see [`well_known`].
    /// Reserved for values whose *shape* is fixed by a published BNetzA or BDEW
    /// document, so that two producers writing one are writing the same thing.
    ///
    /// # Why a namespace may name a standard, and a module may not
    ///
    /// There is deliberately no `rubo4e::mabis` **module**: a module named for a
    /// standard invites everything that standard covers, and this crate holds only
    /// what reads a BO4E field. A namespace is the opposite kind of name. It is a
    /// wire-level **provenance** tag, read by a consumer who has to decide whether
    /// an attribute means anything to them — and "this came from the MaBiS rules"
    /// is precisely the useful answer. `mako`, `hems` and `edmd` name systems
    /// because those systems' data has no published provenance to name instead.
    pub const MABIS: Self = Self(Cow::Borrowed("mabis"));

    /// The prefixes claimed within the rubo4e ecosystem.
    ///
    /// Claiming a new one means adding it here and shipping it, so a collision is
    /// caught in review rather than in production. Anything not on this list is
    /// still usable — see [`Namespace::new`].
    pub const REGISTERED: &'static [Self] = &[Self::MAKO, Self::HEMS, Self::EDMD, Self::MABIS];

    /// Creates a namespace, validating the prefix.
    ///
    /// # Errors
    /// [`NamespaceError`] if the prefix is empty, contains the separator, or
    /// carries a character outside `[A-Za-z0-9_-]`.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::zusatz_attribut::Namespace;
    ///
    /// assert!(Namespace::new("acme-billing").is_ok());
    /// assert!(Namespace::new("acme:billing").is_err());
    /// # }
    /// ```
    pub fn new(prefix: &str) -> Result<Self, NamespaceError> {
        Self::check(prefix)?;
        Ok(Self(Cow::Owned(prefix.to_owned())))
    }

    /// Creates a namespace from a `'static` prefix without allocating.
    ///
    /// Validates the same way as [`new`](Self::new); the difference is only that
    /// the prefix is borrowed rather than copied.
    ///
    /// # Errors
    /// As [`new`](Self::new).
    pub fn from_static(prefix: &'static str) -> Result<Self, NamespaceError> {
        Self::check(prefix)?;
        Ok(Self(Cow::Borrowed(prefix)))
    }

    fn check(prefix: &str) -> Result<(), NamespaceError> {
        if prefix.is_empty() {
            return Err(NamespaceError::Empty);
        }
        for (position, character) in prefix.char_indices() {
            if character == SEPARATOR {
                return Err(NamespaceError::ContainsSeparator);
            }
            if !character.is_ascii_alphanumeric() && character != '_' && character != '-' {
                return Err(NamespaceError::InvalidCharacter {
                    position,
                    character,
                });
            }
        }
        Ok(())
    }

    /// The prefix, without the separator.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `true` if this prefix is on [`REGISTERED`](Namespace::REGISTERED).
    #[must_use]
    pub fn is_registered(&self) -> bool {
        Self::REGISTERED.contains(self)
    }

    /// The full `ZusatzAttribut` name for `key` — `"hems"` + `"eebus-ski"` →
    /// `"hems:eebus-ski"`.
    #[must_use]
    pub fn name(&self, key: &str) -> String {
        let mut out = String::with_capacity(self.0.len() + 1 + key.len());
        out.push_str(&self.0);
        out.push(SEPARATOR);
        out.push_str(key);
        out
    }

    /// Splits a `ZusatzAttribut` name into its namespace prefix and key.
    ///
    /// Splits at the **first** separator, so a key may itself contain one.
    /// Returns `None` for a name with no separator — a plain, un-namespaced
    /// attribute — and for one whose prefix is empty, since `""` is not a prefix
    /// [`new`](Self::new) would ever hand out.
    ///
    /// ```
    /// # #[cfg(feature = "versioned")] {
    /// use rubo4e::zusatz_attribut::Namespace;
    ///
    /// assert_eq!(Namespace::split("hems:eebus-ski"), Some(("hems", "eebus-ski")));
    /// assert_eq!(Namespace::split("mako:ref:1"), Some(("mako", "ref:1")));
    /// assert_eq!(Namespace::split("kundennummer"), None);
    /// assert_eq!(Namespace::split(":orphan"), None);
    /// # }
    /// ```
    #[must_use]
    pub fn split(name: &str) -> Option<(&str, &str)> {
        let (prefix, key) = name.split_once(SEPARATOR)?;
        (!prefix.is_empty()).then_some((prefix, key))
    }

    /// The key part of `name`, if it is in this namespace.
    #[must_use]
    pub fn key_of<'a>(&self, name: &'a str) -> Option<&'a str> {
        let (ns, key) = Self::split(name)?;
        (ns == self.0).then_some(key)
    }
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{SEPARATOR}", self.0)
    }
}

impl std::str::FromStr for Namespace {
    type Err = NamespaceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept both `"hems"` and the `"hems:"` spelling `Display` writes.
        Self::new(s.strip_suffix(SEPARATOR).unwrap_or(s))
    }
}

// ─── Typed keys ──────────────────────────────────────────────────────────────

/// A namespaced attribute name that also fixes the **type** of its value.
///
/// A [`Namespace`] stops two systems colliding on a name. It does not stop them
/// disagreeing about what the value behind that name *is* — one writing a string
/// where the other reads an object. An `AttributKey<T>` closes that too: the key
/// and the type travel together, as one `const` both sides import.
///
/// ```
/// # #[cfg(all(feature = "versioned", feature = "json"))] {
/// use rubo4e::current::Marktlokation;
/// use rubo4e::zusatz_attribut::{AttributKey, Namespace, ZusatzAttributeExt};
///
/// # #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
/// # struct Ladepunkt { evse_id: String }
/// // Declared once, in the crate that owns the namespace.
/// const LADEPUNKT: AttributKey<Ladepunkt> = AttributKey::new(Namespace::HEMS, "ladepunkt");
///
/// let mut malo = Marktlokation::default();
/// malo.set_zusatz_attribut_key(&LADEPUNKT, &Ladepunkt { evse_id: "DE*ABC*E123".into() }).unwrap();
///
/// assert_eq!(LADEPUNKT.name(), "hems:ladepunkt");
/// assert!(malo.has_zusatz_attribut_key(&LADEPUNKT));
/// let read = malo.zusatz_attribut_key(&LADEPUNKT).unwrap().unwrap();
/// assert_eq!(read.evse_id, "DE*ABC*E123");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributKey<T: ?Sized> {
    namespace: Namespace,
    key: &'static str,
    // `fn() -> T` rather than `T`, so the key is `Send + Sync` and carries no
    // drop or variance obligation from the value type it names.
    _value: PhantomData<fn() -> T>,
}

impl<T: ?Sized> AttributKey<T> {
    /// Declares a key. `const`, so a registry is a list of `const`s.
    ///
    /// `key` is not validated here — a `const fn` cannot return an error — but it
    /// must not contain the [`SEPARATOR`], or [`Namespace::split`] will read part
    /// of it as the namespace. `tests` in this module pin that for every key the
    /// crate registers; do the same for yours.
    #[must_use]
    pub const fn new(namespace: Namespace, key: &'static str) -> Self {
        Self {
            namespace,
            key,
            _value: PhantomData,
        }
    }

    /// The namespace this key lives in.
    #[must_use]
    pub const fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    /// The key, without its namespace.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// The full `ZusatzAttribut` name — `"mabis:zaehlpunkt"`.
    #[must_use]
    pub fn name(&self) -> String {
        self.namespace.name(self.key)
    }
}

impl<T: ?Sized> std::fmt::Display for AttributKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{SEPARATOR}{}", self.namespace.as_str(), self.key)
    }
}

/// The keys `rubo4e` itself registers, so two crates carrying the same MaBiS fact
/// carry it under the same name.
///
/// A key here is a commitment: its name and its type are part of the crate's
/// public API and change only with a major version. The bar for adding one is
/// therefore the bar the crate applies to every hand-written accessor — the value must
/// **qualify a BO4E field** that cannot express the distinction on its own.
/// A domain aggregate of some other standard does not qualify, however useful:
/// it belongs in the crate that owns that standard, which can register its own
/// key in its own namespace with [`AttributKey::new`].
///
/// ```
/// # #[cfg(all(feature = "versioned", feature = "json"))] {
/// use rubo4e::current::Messlokation;
/// use rubo4e::identifiers::{Zaehlpunkt, Zaehlpunktart, Zaehlpunktbezeichnung};
/// use rubo4e::zusatz_attribut::{well_known, ZusatzAttributeExt};
///
/// // A Zählpunkt (eMob) is *not* a Messlokations-ID, so it cannot go in
/// // `messlokationsId` — it goes here, under the key both sides import.
/// let mut melo = Messlokation::default();
/// melo.set_zusatz_attribut_key(
///     &well_known::ZAEHLPUNKT,
///     &Zaehlpunkt::new(
///         Zaehlpunktart::NetzgangzeitreiheEmob,
///         Zaehlpunktbezeichnung::new("DE0000000000000000000000000000042").unwrap(),
///     ),
/// )
/// .unwrap();
///
/// assert_eq!(well_known::ZAEHLPUNKT.name(), "mabis:zaehlpunkt");
/// let zp = melo.zusatz_attribut_key(&well_known::ZAEHLPUNKT).unwrap().unwrap();
/// assert!(zp.is_emobilitaet());
/// # }
/// ```
pub mod well_known {
    use super::{AttributKey, Namespace};

    /// Which kind of Zählpunkt a Zählpunktbezeichnung names.
    ///
    /// BO4E has one field for a Zählpunktbezeichnung and assumes it is always a
    /// Messlokation; BK6-20-160 § 1.6.2 says the Zählpunkt (eMob) explicitly is
    /// not. See [`crate::identifiers::Zaehlpunkt`].
    pub const ZAEHLPUNKT: AttributKey<crate::identifiers::Zaehlpunkt> =
        AttributKey::new(Namespace::MABIS, "zaehlpunkt");
}

// ─── The generated half ──────────────────────────────────────────────────────

/// Access to a type's `zusatzAttribute` field.
///
/// Implemented by the generator for every BO4E Geschäftsobjekt and component that
/// declares one — which is all of them but [`ZusatzAttribut`] itself. You do not
/// implement this; you use [`ZusatzAttributeExt`], which is blanket-implemented
/// on top of it.
pub trait HasZusatzAttribute {
    /// The field, as the schema declares it.
    fn zusatz_attribute_field(&self) -> Option<&Vec<ZusatzAttribut>>;

    /// The field, mutably — `None` included, so a first write can create the
    /// vector.
    fn zusatz_attribute_field_mut(&mut self) -> &mut Option<Vec<ZusatzAttribut>>;
}

// ─── The ergonomic half ──────────────────────────────────────────────────────

/// Reading and writing `zusatzAttribute`, with and without a namespace.
///
/// Blanket-implemented for every type that implements [`HasZusatzAttribute`], so
/// it is available on every BO4E Geschäftsobjekt and component.
///
/// # Ordering and duplicates
///
/// The list is a `Vec` and stays one: entries keep insertion order, and a `set_*`
/// replaces the **first** entry with that name rather than appending a second.
/// A payload that arrived with duplicates keeps them; [`zusatz_attribut`] reads
/// the first, which is what a reader that ignores the problem would also see.
///
/// [`zusatz_attribut`]: ZusatzAttributeExt::zusatz_attribut
pub trait ZusatzAttributeExt: HasZusatzAttribute {
    /// Every attribute, in order.
    fn zusatz_attribute(&self) -> impl Iterator<Item = &ZusatzAttribut> {
        self.zusatz_attribute_field()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
    }

    /// The first attribute with this exact name.
    fn zusatz_attribut(&self, name: &str) -> Option<&ZusatzAttribut> {
        self.zusatz_attribute()
            .find(|a| a.name.as_deref() == Some(name))
    }

    /// The first attribute with `key` inside `namespace`.
    fn zusatz_attribut_in(&self, namespace: &Namespace, key: &str) -> Option<&ZusatzAttribut> {
        self.zusatz_attribut(&namespace.name(key))
    }

    /// Every attribute inside `namespace`, as `(key, attribute)`.
    ///
    /// Attributes with no name, or a name in another namespace, are skipped.
    fn zusatz_attribute_in<'a>(
        &'a self,
        namespace: &'a Namespace,
    ) -> impl Iterator<Item = (&'a str, &'a ZusatzAttribut)> {
        self.zusatz_attribute()
            .filter_map(move |a| Some((namespace.key_of(a.name.as_deref()?)?, a)))
    }

    /// `true` if any attribute carries a name in `namespace`.
    fn has_zusatz_attribute_in(&self, namespace: &Namespace) -> bool {
        self.zusatz_attribute_in(namespace).next().is_some()
    }

    /// Every namespace prefix that appears in the list, deduplicated, in first-use
    /// order.
    ///
    /// Useful at an ingest boundary: a prefix you do not recognise is a system you
    /// are not round-tripping for.
    fn zusatz_attribut_namespaces(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for a in self.zusatz_attribute() {
            if let Some((ns, _)) = a.name.as_deref().and_then(Namespace::split) {
                if !out.contains(&ns) {
                    out.push(ns);
                }
            }
        }
        out
    }

    /// The value of the first attribute with this name, as a string.
    ///
    /// With the `json` feature the value is a `serde_json::Value`, and this reads
    /// only a JSON string — a number or an object yields `None`. Use
    /// [`zusatz_attribut_as`](Self::zusatz_attribut_as) for those.
    fn zusatz_attribut_str(&self, name: &str) -> Option<&str> {
        let attribut = self.zusatz_attribut(name)?;
        #[cfg(feature = "json")]
        {
            attribut.wert.as_ref()?.as_str()
        }
        #[cfg(not(feature = "json"))]
        {
            attribut.wert.as_deref()
        }
    }

    /// The value of `key` inside `namespace`, as a string.
    fn zusatz_attribut_str_in(&self, namespace: &Namespace, key: &str) -> Option<&str> {
        self.zusatz_attribut_str(&namespace.name(key))
    }

    /// Sets an attribute to a string value, replacing the first entry with that
    /// name.
    ///
    /// Returns the entry that was replaced, if there was one.
    fn set_zusatz_attribut(
        &mut self,
        name: impl Into<String>,
        wert: impl Into<String>,
    ) -> Option<ZusatzAttribut> {
        let name = name.into();
        #[cfg(feature = "json")]
        let wert = serde_json::Value::String(wert.into());
        #[cfg(not(feature = "json"))]
        let wert = wert.into();
        self.put_zusatz_attribut(ZusatzAttribut {
            name: Some(name),
            wert: Some(wert),
            ..Default::default()
        })
    }

    /// Sets `key` inside `namespace` to a string value.
    fn set_zusatz_attribut_in(
        &mut self,
        namespace: &Namespace,
        key: &str,
        wert: impl Into<String>,
    ) -> Option<ZusatzAttribut> {
        self.set_zusatz_attribut(namespace.name(key), wert)
    }

    /// Inserts `attribut`, replacing the first existing entry with the same name.
    ///
    /// The low-level primitive the `set_*` methods are built on. An attribute with
    /// no `name` is appended, never matched — BO4E leaves the field optional, and
    /// two nameless entries are not "the same" one.
    fn put_zusatz_attribut(&mut self, attribut: ZusatzAttribut) -> Option<ZusatzAttribut> {
        let slot = self
            .zusatz_attribute_field_mut()
            .get_or_insert_with(Vec::new);
        if let Some(name) = attribut.name.as_deref() {
            if let Some(existing) = slot.iter_mut().find(|a| a.name.as_deref() == Some(name)) {
                return Some(std::mem::replace(existing, attribut));
            }
        }
        slot.push(attribut);
        None
    }

    /// Removes the first attribute with this name and returns it.
    ///
    /// Leaves the field as `Some(vec![])` rather than `None` when the last entry
    /// goes: BO4E treats an absent list and an empty one alike, and re-`None`ing
    /// would make `remove` visible in the serialised output of a document that
    /// never had the attribute.
    fn remove_zusatz_attribut(&mut self, name: &str) -> Option<ZusatzAttribut> {
        let slot = self.zusatz_attribute_field_mut().as_mut()?;
        let at = slot.iter().position(|a| a.name.as_deref() == Some(name))?;
        Some(slot.remove(at))
    }

    /// Removes `key` inside `namespace`.
    fn remove_zusatz_attribut_in(
        &mut self,
        namespace: &Namespace,
        key: &str,
    ) -> Option<ZusatzAttribut> {
        self.remove_zusatz_attribut(&namespace.name(key))
    }

    /// Removes every attribute in `namespace` and returns them, in order.
    ///
    /// The call to make before handing a document to a partner who has no business
    /// seeing one system's internals.
    fn remove_zusatz_attribute_in(&mut self, namespace: &Namespace) -> Vec<ZusatzAttribut> {
        let Some(slot) = self.zusatz_attribute_field_mut().as_mut() else {
            return Vec::new();
        };
        let mut taken = Vec::new();
        let mut i = 0;
        while i < slot.len() {
            let matches = slot[i]
                .name
                .as_deref()
                .is_some_and(|n| namespace.key_of(n).is_some());
            if matches {
                taken.push(slot.remove(i));
            } else {
                i += 1;
            }
        }
        taken
    }

    /// Reads an attribute's value as `T`.
    ///
    /// `None` when the attribute is absent or carries no value;
    /// `Some(Err(_))` when it is present but does not deserialize into `T`.
    ///
    /// ```
    /// # #[cfg(all(feature = "versioned", feature = "json"))] {
    /// use rubo4e::current::TechnischeRessource;
    /// use rubo4e::zusatz_attribut::{Namespace, ZusatzAttributeExt};
    ///
    /// // A code list BO4E does not publish stays a type in *your* crate.
    /// #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    /// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    /// enum Steuerungsvariante { Direktansteuerung, Ems }
    ///
    /// let mut tr = TechnischeRessource::default();
    /// tr.set_zusatz_attribut_as(
    ///     Namespace::HEMS.name("steuerungsvariante"),
    ///     &Steuerungsvariante::Ems,
    /// ).unwrap();
    ///
    /// let read: Steuerungsvariante = tr
    ///     .zusatz_attribut_as(&Namespace::HEMS.name("steuerungsvariante"))
    ///     .unwrap()
    ///     .unwrap();
    /// assert_eq!(read, Steuerungsvariante::Ems);
    /// # }
    /// ```
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn zusatz_attribut_as<T: serde::de::DeserializeOwned>(
        &self,
        name: &str,
    ) -> Option<Result<T, serde_json::Error>> {
        let wert = self.zusatz_attribut(name)?.wert.as_ref()?;
        Some(T::deserialize(wert))
    }

    /// Reads `key` inside `namespace` as `T`.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn zusatz_attribut_as_in<T: serde::de::DeserializeOwned>(
        &self,
        namespace: &Namespace,
        key: &str,
    ) -> Option<Result<T, serde_json::Error>> {
        self.zusatz_attribut_as(&namespace.name(key))
    }

    /// Sets an attribute to the JSON encoding of `value`.
    ///
    /// # Errors
    /// [`serde_json::Error`] if `value` does not serialize.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn set_zusatz_attribut_as<T: serde::Serialize + ?Sized>(
        &mut self,
        name: impl Into<String>,
        value: &T,
    ) -> Result<Option<ZusatzAttribut>, serde_json::Error> {
        let wert = serde_json::to_value(value)?;
        Ok(self.put_zusatz_attribut(ZusatzAttribut {
            name: Some(name.into()),
            wert: Some(wert),
            ..Default::default()
        }))
    }

    /// Sets `key` inside `namespace` to the JSON encoding of `value`.
    ///
    /// # Errors
    /// As [`set_zusatz_attribut_as`](Self::set_zusatz_attribut_as).
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn set_zusatz_attribut_as_in<T: serde::Serialize + ?Sized>(
        &mut self,
        namespace: &Namespace,
        key: &str,
        value: &T,
    ) -> Result<Option<ZusatzAttribut>, serde_json::Error> {
        self.set_zusatz_attribut_as(namespace.name(key), value)
    }

    /// `true` if this object carries `key`.
    fn has_zusatz_attribut_key<T: ?Sized>(&self, key: &AttributKey<T>) -> bool {
        self.zusatz_attribut(&key.name()).is_some()
    }

    /// Reads the value behind a typed key.
    ///
    /// `None` when the attribute is absent or carries no value; `Some(Err(_))`
    /// when it is present but is not the type the key declares.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn zusatz_attribut_key<T: serde::de::DeserializeOwned>(
        &self,
        key: &AttributKey<T>,
    ) -> Option<Result<T, serde_json::Error>> {
        self.zusatz_attribut_as(&key.name())
    }

    /// Writes the value behind a typed key, replacing any existing entry.
    ///
    /// # Errors
    /// [`serde_json::Error`] if `value` does not serialize.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn set_zusatz_attribut_key<T: serde::Serialize>(
        &mut self,
        key: &AttributKey<T>,
        value: &T,
    ) -> Result<Option<ZusatzAttribut>, serde_json::Error> {
        self.set_zusatz_attribut_as(key.name(), value)
    }

    /// Removes the attribute behind a typed key and returns it.
    fn remove_zusatz_attribut_key<T: ?Sized>(
        &mut self,
        key: &AttributKey<T>,
    ) -> Option<ZusatzAttribut> {
        self.remove_zusatz_attribut(&key.name())
    }
}

impl<T: HasZusatzAttribute + ?Sized> ZusatzAttributeExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::current::{Marktlokation, SteuerbareRessource};

    #[test]
    fn namespace_rejects_what_would_be_ambiguous() {
        assert_eq!(Namespace::new(""), Err(NamespaceError::Empty));
        assert_eq!(
            Namespace::new("a:b"),
            Err(NamespaceError::ContainsSeparator)
        );
        assert!(matches!(
            Namespace::new("a b"),
            Err(NamespaceError::InvalidCharacter { position: 1, .. })
        ));
        assert!(Namespace::new("acme-billing_2").is_ok());
    }

    /// Every registered key must be well formed, or `Namespace::split` reads part
    /// of the key as a namespace and the registry silently means nothing.
    #[test]
    fn registered_keys_are_well_formed() {
        fn check(name: &str, ns: &Namespace, key: &str) {
            assert!(
                ns.is_registered(),
                "{name}: {ns} is not a registered namespace"
            );
            assert!(!key.is_empty(), "{name}: empty key");
            assert!(
                !key.contains(SEPARATOR),
                "{name}: key contains the separator"
            );
            assert_eq!(Namespace::split(name), Some((ns.as_str(), key)));
        }
        check(
            &well_known::ZAEHLPUNKT.name(),
            well_known::ZAEHLPUNKT.namespace(),
            well_known::ZAEHLPUNKT.key(),
        );
    }

    #[test]
    fn registered_namespaces_are_well_formed_and_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for ns in Namespace::REGISTERED {
            assert!(Namespace::new(ns.as_str()).is_ok(), "{ns}");
            assert!(ns.is_registered());
            assert!(seen.insert(ns.as_str()), "duplicate namespace {ns}");
        }
        assert!(!Namespace::new("acme").unwrap().is_registered());
    }

    #[test]
    fn display_and_from_str_round_trip() {
        let ns = Namespace::HEMS;
        assert_eq!(ns.to_string(), "hems:");
        assert_eq!("hems:".parse::<Namespace>().unwrap(), ns);
        assert_eq!("hems".parse::<Namespace>().unwrap(), ns);
    }

    #[test]
    fn split_takes_the_first_separator_only() {
        assert_eq!(Namespace::split("mako:ref:1"), Some(("mako", "ref:1")));
        // An empty prefix is not a namespace — `new` would never hand one out.
        assert_eq!(Namespace::split(":orphan"), None);
        assert_eq!(Namespace::HEMS.key_of("hems:a:b"), Some("a:b"));
        assert_eq!(Namespace::HEMS.key_of("mako:a"), None);
        assert_eq!(Namespace::HEMS.key_of("plain"), None);
    }

    #[test]
    fn set_replaces_rather_than_appends() {
        let mut sr = SteuerbareRessource::default();
        assert!(sr
            .set_zusatz_attribut_in(&Namespace::HEMS, "ski", "aaa")
            .is_none());
        let old = sr
            .set_zusatz_attribut_in(&Namespace::HEMS, "ski", "bbb")
            .expect("the first value comes back");
        assert_eq!(old.name.as_deref(), Some("hems:ski"));
        assert_eq!(sr.zusatz_attribute().count(), 1);
        assert_eq!(
            sr.zusatz_attribut_str_in(&Namespace::HEMS, "ski"),
            Some("bbb")
        );
    }

    #[test]
    fn namespaces_do_not_collide() {
        let mut malo = Marktlokation::default();
        malo.set_zusatz_attribut_in(&Namespace::MAKO, "id", "M-1");
        malo.set_zusatz_attribut_in(&Namespace::HEMS, "id", "H-1");
        assert_eq!(malo.zusatz_attribute().count(), 2);
        assert_eq!(
            malo.zusatz_attribut_str_in(&Namespace::MAKO, "id"),
            Some("M-1")
        );
        assert_eq!(
            malo.zusatz_attribut_str_in(&Namespace::HEMS, "id"),
            Some("H-1")
        );
        assert_eq!(malo.zusatz_attribut_namespaces(), ["mako", "hems"]);

        // A leading separator is not a namespace, so it does not appear.
        malo.set_zusatz_attribut(":orphan", "x");
        assert_eq!(malo.zusatz_attribut_namespaces(), ["mako", "hems"]);
    }

    #[test]
    fn removing_a_namespace_leaves_the_others() {
        let mut malo = Marktlokation::default();
        malo.set_zusatz_attribut_in(&Namespace::MAKO, "a", "1");
        malo.set_zusatz_attribut("kundennummer", "K-9");
        malo.set_zusatz_attribut_in(&Namespace::MAKO, "b", "2");

        let taken = malo.remove_zusatz_attribute_in(&Namespace::MAKO);
        assert_eq!(taken.len(), 2);
        assert_eq!(malo.zusatz_attribute().count(), 1);
        assert_eq!(malo.zusatz_attribut_str("kundennummer"), Some("K-9"));
        assert!(!malo.has_zusatz_attribute_in(&Namespace::MAKO));
    }

    #[test]
    fn remove_on_an_absent_list_is_a_no_op() {
        let mut malo = Marktlokation::default();
        assert!(malo.remove_zusatz_attribut("nope").is_none());
        assert!(malo.remove_zusatz_attribute_in(&Namespace::HEMS).is_empty());
        assert!(malo.zusatz_attribute_field().is_none());
    }

    /// A nameless entry is never treated as "the same" as another nameless one.
    #[test]
    fn nameless_attributes_are_appended() {
        let mut malo = Marktlokation::default();
        malo.put_zusatz_attribut(ZusatzAttribut::default());
        malo.put_zusatz_attribut(ZusatzAttribut::default());
        assert_eq!(malo.zusatz_attribute().count(), 2);
    }

    #[cfg(feature = "json")]
    #[test]
    fn typed_values_round_trip() {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Steuerung {
            variante: String,
            stufen: u8,
        }

        let mut sr = SteuerbareRessource::default();
        sr.set_zusatz_attribut_as_in(
            &Namespace::HEMS,
            "steuerung",
            &Steuerung {
                variante: "EMS".into(),
                stufen: 4,
            },
        )
        .unwrap();

        let read: Steuerung = sr
            .zusatz_attribut_as_in(&Namespace::HEMS, "steuerung")
            .unwrap()
            .unwrap();
        assert_eq!(read.stufen, 4);

        // A structured value is not a string, and does not pretend to be one.
        assert_eq!(
            sr.zusatz_attribut_str_in(&Namespace::HEMS, "steuerung"),
            None
        );

        // A type mismatch is an error, not a silent `None`.
        assert!(sr
            .zusatz_attribut_as_in::<u8>(&Namespace::HEMS, "steuerung")
            .unwrap()
            .is_err());
    }
}

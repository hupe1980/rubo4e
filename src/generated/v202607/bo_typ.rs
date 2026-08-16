#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Auflistung sämtlicher existierender Geschäftsobjekte.
#[non_exhaustive]
pub enum BoTyp {
    #[cfg_attr(feature = "serde", serde(rename = "ANGEBOT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANGEBOT"))]
    Angebot,
    #[cfg_attr(feature = "serde", serde(rename = "AUSSCHREIBUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUSSCHREIBUNG"))]
    Ausschreibung,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNG"))]
    Bilanzierung,
    #[cfg_attr(feature = "serde", serde(rename = "BUENDELVERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUENDELVERTRAG"))]
    Buendelvertrag,
    #[cfg_attr(feature = "serde", serde(rename = "ENERGIEMENGE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ENERGIEMENGE"))]
    Energiemenge,
    #[cfg_attr(feature = "serde", serde(rename = "FREMDKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FREMDKOSTEN"))]
    Fremdkosten,
    #[cfg_attr(feature = "serde", serde(rename = "GERAET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GERAET"))]
    Geraet,
    #[cfg_attr(feature = "serde", serde(rename = "GESCHAEFTSOBJEKT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GESCHAEFTSOBJEKT"))]
    Geschaeftsobjekt,
    #[cfg_attr(feature = "serde", serde(rename = "GESCHAEFTSPARTNER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GESCHAEFTSPARTNER"))]
    Geschaeftspartner,
    #[cfg_attr(feature = "serde", serde(rename = "KOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KOSTEN"))]
    Kosten,
    #[cfg_attr(feature = "serde", serde(rename = "LASTGANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LASTGANG"))]
    Lastgang,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTLOKATION"))]
    Marktlokation,
    #[cfg_attr(feature = "serde", serde(rename = "MESSLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSLOKATION"))]
    Messlokation,
    #[cfg_attr(feature = "serde", serde(rename = "NETZLOKATION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZLOKATION"))]
    Netzlokation,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTTEILNEHMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTTEILNEHMER"))]
    Marktteilnehmer,
    #[cfg_attr(feature = "serde", serde(rename = "NETZNUTZUNGSRECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZNUTZUNGSRECHNUNG"))]
    Netznutzungsrechnung,
    #[cfg_attr(feature = "serde", serde(rename = "TECHNISCHERESSOURCE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TECHNISCHERESSOURCE"))]
    Technischeressource,
    #[cfg_attr(feature = "serde", serde(rename = "STEUERBARERESSOURCE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STEUERBARERESSOURCE"))]
    Steuerbareressource,
    #[cfg_attr(feature = "serde", serde(rename = "PERSON"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PERSON"))]
    Person,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATT"))]
    Preisblatt,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTDIENSTLEISTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTDIENSTLEISTUNG"))]
    Preisblattdienstleistung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTHARDWARE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTHARDWARE"))]
    Preisblatthardware,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTKONZESSIONSABGABE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTKONZESSIONSABGABE"))]
    Preisblattkonzessionsabgabe,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTMESSUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTMESSUNG"))]
    Preisblattmessung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTNETZNUTZUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTNETZNUTZUNG"))]
    Preisblattnetznutzung,
    #[cfg_attr(feature = "serde", serde(rename = "PREISBLATTUMLAGEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PREISBLATTUMLAGEN"))]
    Preisblattumlagen,
    #[cfg_attr(feature = "serde", serde(rename = "RECHNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RECHNUNG"))]
    Rechnung,
    #[cfg_attr(feature = "serde", serde(rename = "REGION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGION"))]
    Region,
    #[cfg_attr(feature = "serde", serde(rename = "STANDORTEIGENSCHAFTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STANDORTEIGENSCHAFTEN"))]
    Standorteigenschaften,
    #[cfg_attr(feature = "serde", serde(rename = "TARIF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIF"))]
    Tarif,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFINFO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFINFO"))]
    Tarifinfo,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFKOSTEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFKOSTEN"))]
    Tarifkosten,
    #[cfg_attr(feature = "serde", serde(rename = "TARIFPREISBLATT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TARIFPREISBLATT"))]
    Tarifpreisblatt,
    #[cfg_attr(feature = "serde", serde(rename = "VERTRAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERTRAG"))]
    Vertrag,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLER"))]
    Zaehler,
    #[cfg_attr(feature = "serde", serde(rename = "ZAEHLZEITDEFINITION"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAEHLZEITDEFINITION"))]
    Zaehlzeitdefinition,
    #[cfg_attr(feature = "serde", serde(rename = "ZEITREIHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZEITREIHE"))]
    Zeitreihe,
    #[cfg_attr(feature = "serde", serde(rename = "LOKATIONSZUORDNUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LOKATIONSZUORDNUNG"))]
    Lokationszuordnung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl BoTyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`BoTyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Angebot,
        Self::Ausschreibung,
        Self::Bilanzierung,
        Self::Buendelvertrag,
        Self::Energiemenge,
        Self::Fremdkosten,
        Self::Geraet,
        Self::Geschaeftsobjekt,
        Self::Geschaeftspartner,
        Self::Kosten,
        Self::Lastgang,
        Self::Marktlokation,
        Self::Messlokation,
        Self::Netzlokation,
        Self::Marktteilnehmer,
        Self::Netznutzungsrechnung,
        Self::Technischeressource,
        Self::Steuerbareressource,
        Self::Person,
        Self::Preisblatt,
        Self::Preisblattdienstleistung,
        Self::Preisblatthardware,
        Self::Preisblattkonzessionsabgabe,
        Self::Preisblattmessung,
        Self::Preisblattnetznutzung,
        Self::Preisblattumlagen,
        Self::Rechnung,
        Self::Region,
        Self::Standorteigenschaften,
        Self::Tarif,
        Self::Tarifinfo,
        Self::Tarifkosten,
        Self::Tarifpreisblatt,
        Self::Vertrag,
        Self::Zaehler,
        Self::Zaehlzeitdefinition,
        Self::Zeitreihe,
        Self::Lokationszuordnung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`BoTyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `BoTyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`BoTyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::BoTyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(BoTyp::iter_known().count(), BoTyp::COUNT);
    /// assert!(BoTyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`BoTyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Angebot => "ANGEBOT",
            Self::Ausschreibung => "AUSSCHREIBUNG",
            Self::Bilanzierung => "BILANZIERUNG",
            Self::Buendelvertrag => "BUENDELVERTRAG",
            Self::Energiemenge => "ENERGIEMENGE",
            Self::Fremdkosten => "FREMDKOSTEN",
            Self::Geraet => "GERAET",
            Self::Geschaeftsobjekt => "GESCHAEFTSOBJEKT",
            Self::Geschaeftspartner => "GESCHAEFTSPARTNER",
            Self::Kosten => "KOSTEN",
            Self::Lastgang => "LASTGANG",
            Self::Marktlokation => "MARKTLOKATION",
            Self::Messlokation => "MESSLOKATION",
            Self::Netzlokation => "NETZLOKATION",
            Self::Marktteilnehmer => "MARKTTEILNEHMER",
            Self::Netznutzungsrechnung => "NETZNUTZUNGSRECHNUNG",
            Self::Technischeressource => "TECHNISCHERESSOURCE",
            Self::Steuerbareressource => "STEUERBARERESSOURCE",
            Self::Person => "PERSON",
            Self::Preisblatt => "PREISBLATT",
            Self::Preisblattdienstleistung => "PREISBLATTDIENSTLEISTUNG",
            Self::Preisblatthardware => "PREISBLATTHARDWARE",
            Self::Preisblattkonzessionsabgabe => "PREISBLATTKONZESSIONSABGABE",
            Self::Preisblattmessung => "PREISBLATTMESSUNG",
            Self::Preisblattnetznutzung => "PREISBLATTNETZNUTZUNG",
            Self::Preisblattumlagen => "PREISBLATTUMLAGEN",
            Self::Rechnung => "RECHNUNG",
            Self::Region => "REGION",
            Self::Standorteigenschaften => "STANDORTEIGENSCHAFTEN",
            Self::Tarif => "TARIF",
            Self::Tarifinfo => "TARIFINFO",
            Self::Tarifkosten => "TARIFKOSTEN",
            Self::Tarifpreisblatt => "TARIFPREISBLATT",
            Self::Vertrag => "VERTRAG",
            Self::Zaehler => "ZAEHLER",
            Self::Zaehlzeitdefinition => "ZAEHLZEITDEFINITION",
            Self::Zeitreihe => "ZEITREIHE",
            Self::Lokationszuordnung => "LOKATIONSZUORDNUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`BoTyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::BoTyp;
    /// /// assert_eq!(BoTyp::from_wire("ANGEBOT"), Ok(BoTyp::Angebot));
    /// assert!(BoTyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(BoTyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "ANGEBOT" => Ok(Self::Angebot),
            "AUSSCHREIBUNG" => Ok(Self::Ausschreibung),
            "BILANZIERUNG" => Ok(Self::Bilanzierung),
            "BUENDELVERTRAG" => Ok(Self::Buendelvertrag),
            "ENERGIEMENGE" => Ok(Self::Energiemenge),
            "FREMDKOSTEN" => Ok(Self::Fremdkosten),
            "GERAET" => Ok(Self::Geraet),
            "GESCHAEFTSOBJEKT" => Ok(Self::Geschaeftsobjekt),
            "GESCHAEFTSPARTNER" => Ok(Self::Geschaeftspartner),
            "KOSTEN" => Ok(Self::Kosten),
            "LASTGANG" => Ok(Self::Lastgang),
            "MARKTLOKATION" => Ok(Self::Marktlokation),
            "MESSLOKATION" => Ok(Self::Messlokation),
            "NETZLOKATION" => Ok(Self::Netzlokation),
            "MARKTTEILNEHMER" => Ok(Self::Marktteilnehmer),
            "NETZNUTZUNGSRECHNUNG" => Ok(Self::Netznutzungsrechnung),
            "TECHNISCHERESSOURCE" => Ok(Self::Technischeressource),
            "STEUERBARERESSOURCE" => Ok(Self::Steuerbareressource),
            "PERSON" => Ok(Self::Person),
            "PREISBLATT" => Ok(Self::Preisblatt),
            "PREISBLATTDIENSTLEISTUNG" => Ok(Self::Preisblattdienstleistung),
            "PREISBLATTHARDWARE" => Ok(Self::Preisblatthardware),
            "PREISBLATTKONZESSIONSABGABE" => Ok(Self::Preisblattkonzessionsabgabe),
            "PREISBLATTMESSUNG" => Ok(Self::Preisblattmessung),
            "PREISBLATTNETZNUTZUNG" => Ok(Self::Preisblattnetznutzung),
            "PREISBLATTUMLAGEN" => Ok(Self::Preisblattumlagen),
            "RECHNUNG" => Ok(Self::Rechnung),
            "REGION" => Ok(Self::Region),
            "STANDORTEIGENSCHAFTEN" => Ok(Self::Standorteigenschaften),
            "TARIF" => Ok(Self::Tarif),
            "TARIFINFO" => Ok(Self::Tarifinfo),
            "TARIFKOSTEN" => Ok(Self::Tarifkosten),
            "TARIFPREISBLATT" => Ok(Self::Tarifpreisblatt),
            "VERTRAG" => Ok(Self::Vertrag),
            "ZAEHLER" => Ok(Self::Zaehler),
            "ZAEHLZEITDEFINITION" => Ok(Self::Zaehlzeitdefinition),
            "ZEITREIHE" => Ok(Self::Zeitreihe),
            "LOKATIONSZUORDNUNG" => Ok(Self::Lokationszuordnung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`BoTyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for BoTyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for BoTyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for BoTyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for BoTyp {
    const VARIANTS: &'static [Self] = Self::VARIANTS;
    const COUNT: usize = Self::COUNT;
    fn as_wire(&self) -> &'static str {
        Self::as_wire(self)
    }
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        Self::from_wire(s)
    }
    fn is_unknown(&self) -> bool {
        Self::is_unknown(self)
    }
}
#[cfg(feature = "versioned")]
impl crate::Bo4eStrict for BoTyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for BoTyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for BoTyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`BoTyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`BoTyp::from_wire`] on a `String` column, or check
/// [`BoTyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BoTyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for BoTyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}

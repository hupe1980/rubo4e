#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Bei diesem Enum handelt es sich um die Abbildung von Zählertypen der Sparten Strom und Gas.
#[non_exhaustive]
pub enum Zaehlertyp {
    #[cfg_attr(feature = "serde", serde(rename = "DREHSTROMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DREHSTROMZAEHLER"))]
    Drehstromzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "BALGENGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BALGENGASZAEHLER"))]
    Balgengaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "DREHKOLBENZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DREHKOLBENZAEHLER"))]
    Drehkolbenzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "LEISTUNGSZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LEISTUNGSZAEHLER"))]
    Leistungszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "MAXIMUMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAXIMUMZAEHLER"))]
    Maximumzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "TURBINENRADGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TURBINENRADGASZAEHLER"))]
    Turbinenradgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "ULTRASCHALLGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ULTRASCHALLGASZAEHLER"))]
    Ultraschallgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WECHSELSTROMZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WECHSELSTROMZAEHLER"))]
    Wechselstromzaehler,
    #[cfg_attr(feature = "serde", serde(rename = "MODERNE_MESSEINRICHTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MODERNE_MESSEINRICHTUNG"))]
    ModerneMesseinrichtung,
    ///
    /// **Wire spelling:** `INTELLIGENTES_MESSSYSTEM` (three `s`).  ⚠ BO4E spells the
    /// *same* iMSys concept differently across BOs: `Geraetetyp::IntelligentesMessystem`
    /// uses `INTELLIGENTES_MESSYSTEM` (two `s`).  This divergence is upstream, not a
    /// `rubo4e` transcription error; each BO keeps its own canonical spelling.
    #[cfg_attr(feature = "serde", serde(rename = "INTELLIGENTES_MESSSYSTEM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INTELLIGENTES_MESSSYSTEM"))]
    IntelligentesMesssystem,
    #[cfg_attr(feature = "serde", serde(rename = "ELEKTRONISCHER_ZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ELEKTRONISCHER_ZAEHLER"))]
    ElektronischerZaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WIRBELGASZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WIRBELGASZAEHLER"))]
    Wirbelgaszaehler,
    #[cfg_attr(feature = "serde", serde(rename = "WASSERZAEHLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WASSERZAEHLER"))]
    Wasserzaehler,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Zaehlertyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Zaehlertyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Drehstromzaehler,
        Self::Balgengaszaehler,
        Self::Drehkolbenzaehler,
        Self::Leistungszaehler,
        Self::Maximumzaehler,
        Self::Turbinenradgaszaehler,
        Self::Ultraschallgaszaehler,
        Self::Wechselstromzaehler,
        Self::ModerneMesseinrichtung,
        Self::IntelligentesMesssystem,
        Self::ElektronischerZaehler,
        Self::Wirbelgaszaehler,
        Self::Wasserzaehler,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Zaehlertyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Zaehlertyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Zaehlertyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zaehlertyp;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Zaehlertyp::iter_known().count(), Zaehlertyp::COUNT);
    /// assert!(Zaehlertyp::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Zaehlertyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Drehstromzaehler => "DREHSTROMZAEHLER",
            Self::Balgengaszaehler => "BALGENGASZAEHLER",
            Self::Drehkolbenzaehler => "DREHKOLBENZAEHLER",
            Self::Leistungszaehler => "LEISTUNGSZAEHLER",
            Self::Maximumzaehler => "MAXIMUMZAEHLER",
            Self::Turbinenradgaszaehler => "TURBINENRADGASZAEHLER",
            Self::Ultraschallgaszaehler => "ULTRASCHALLGASZAEHLER",
            Self::Wechselstromzaehler => "WECHSELSTROMZAEHLER",
            Self::ModerneMesseinrichtung => "MODERNE_MESSEINRICHTUNG",
            Self::IntelligentesMesssystem => "INTELLIGENTES_MESSSYSTEM",
            Self::ElektronischerZaehler => "ELEKTRONISCHER_ZAEHLER",
            Self::Wirbelgaszaehler => "WIRBELGASZAEHLER",
            Self::Wasserzaehler => "WASSERZAEHLER",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Zaehlertyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Zaehlertyp;
    /// /// assert_eq!(Zaehlertyp::from_wire("DREHSTROMZAEHLER"), Ok(Zaehlertyp::Drehstromzaehler));
    /// assert!(Zaehlertyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Zaehlertyp::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "DREHSTROMZAEHLER" => Ok(Self::Drehstromzaehler),
            "BALGENGASZAEHLER" => Ok(Self::Balgengaszaehler),
            "DREHKOLBENZAEHLER" => Ok(Self::Drehkolbenzaehler),
            "LEISTUNGSZAEHLER" => Ok(Self::Leistungszaehler),
            "MAXIMUMZAEHLER" => Ok(Self::Maximumzaehler),
            "TURBINENRADGASZAEHLER" => Ok(Self::Turbinenradgaszaehler),
            "ULTRASCHALLGASZAEHLER" => Ok(Self::Ultraschallgaszaehler),
            "WECHSELSTROMZAEHLER" => Ok(Self::Wechselstromzaehler),
            "MODERNE_MESSEINRICHTUNG" => Ok(Self::ModerneMesseinrichtung),
            "INTELLIGENTES_MESSSYSTEM" => Ok(Self::IntelligentesMesssystem),
            "ELEKTRONISCHER_ZAEHLER" => Ok(Self::ElektronischerZaehler),
            "WIRBELGASZAEHLER" => Ok(Self::Wirbelgaszaehler),
            "WASSERZAEHLER" => Ok(Self::Wasserzaehler),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Zaehlertyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Zaehlertyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Zaehlertyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Zaehlertyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Zaehlertyp {
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
impl crate::Bo4eStrict for Zaehlertyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Zaehlertyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Zaehlertyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Zaehlertyp::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Zaehlertyp::from_wire`] on a `String` column, or check
/// [`Zaehlertyp::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Zaehlertyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Zaehlertyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}

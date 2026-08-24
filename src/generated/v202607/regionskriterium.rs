#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Klassifizierung der Kriterien für eine regionale Eingrenzung.
#[non_exhaustive]
pub enum Regionskriterium {
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESLANDKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESLANDKENNZIFFER"))]
    Bundeslandkennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESLAND_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESLAND_NAME"))]
    BundeslandName,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIET_NUMMER"))]
    MarktgebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MARKTGEBIET_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MARKTGEBIET_NAME"))]
    MarktgebietName,
    #[cfg_attr(feature = "serde", serde(rename = "REGELGEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELGEBIET_NUMMER"))]
    RegelgebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "REGELGEBIET_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "REGELGEBIET_NAME"))]
    RegelgebietName,
    #[cfg_attr(feature = "serde", serde(rename = "NETZ_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZ_STROM"))]
    NetzStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZ_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZ_GAS"))]
    NetzGas,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NUMMER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NUMMER_STROM"))]
    NetzbetreiberNummerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NUMMER_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NUMMER_GAS"))]
    NetzbetreiberNummerGas,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NAME_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NAME_STROM"))]
    NetzbetreiberNameStrom,
    #[cfg_attr(feature = "serde", serde(rename = "NETZBETREIBER_NAME_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NETZBETREIBER_NAME_GAS"))]
    NetzbetreiberNameGas,
    #[cfg_attr(feature = "serde", serde(rename = "BILANZIERUNGS_GEBIET_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BILANZIERUNGS_GEBIET_NUMMER"))]
    BilanzierungsGebietNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_NUMMER"))]
    MsbNummer,
    #[cfg_attr(feature = "serde", serde(rename = "MSB_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MSB_NAME"))]
    MsbName,
    #[cfg_attr(feature = "serde", serde(rename = "VERSORGER_NUMMER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSORGER_NUMMER"))]
    VersorgerNummer,
    #[cfg_attr(feature = "serde", serde(rename = "VERSORGER_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERSORGER_NAME"))]
    VersorgerName,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NUMMER_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NUMMER_STROM"))]
    GrundversorgerNummerStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NAME_STROM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NAME_STROM"))]
    GrundversorgerNameStrom,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NUMMER_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NUMMER_GAS"))]
    GrundversorgerNummerGas,
    #[cfg_attr(feature = "serde", serde(rename = "GRUNDVERSORGER_NAME_GAS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GRUNDVERSORGER_NAME_GAS"))]
    GrundversorgerNameGas,
    #[cfg_attr(feature = "serde", serde(rename = "KREIS_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KREIS_NAME"))]
    KreisName,
    #[cfg_attr(feature = "serde", serde(rename = "KREISKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KREISKENNZIFFER"))]
    Kreiskennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINDE_NAME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINDE_NAME"))]
    GemeindeName,
    #[cfg_attr(feature = "serde", serde(rename = "GEMEINDEKENNZIFFER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEMEINDEKENNZIFFER"))]
    Gemeindekennziffer,
    #[cfg_attr(feature = "serde", serde(rename = "POSTLEITZAHL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "POSTLEITZAHL"))]
    Postleitzahl,
    #[cfg_attr(feature = "serde", serde(rename = "ORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ORT"))]
    Ort,
    #[cfg_attr(feature = "serde", serde(rename = "POSTORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "POSTORT"))]
    Postort,
    #[cfg_attr(feature = "serde", serde(rename = "EINWOHNERZAHL_GEMEINDE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINWOHNERZAHL_GEMEINDE"))]
    EinwohnerzahlGemeinde,
    #[cfg_attr(feature = "serde", serde(rename = "EINWOHNERZAHL_ORT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EINWOHNERZAHL_ORT"))]
    EinwohnerzahlOrt,
    #[cfg_attr(feature = "serde", serde(rename = "PLZ_KM_UMKREIS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLZ_KM_UMKREIS"))]
    PlzKmUmkreis,
    #[cfg_attr(feature = "serde", serde(rename = "BUNDESWEIT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BUNDESWEIT"))]
    Bundesweit,
    #[cfg_attr(feature = "serde", serde(rename = "PLZ_BEREICH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLZ_BEREICH"))]
    PlzBereich,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Regionskriterium {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Regionskriterium::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Bundeslandkennziffer,
        Self::BundeslandName,
        Self::MarktgebietNummer,
        Self::MarktgebietName,
        Self::RegelgebietNummer,
        Self::RegelgebietName,
        Self::NetzStrom,
        Self::NetzGas,
        Self::NetzbetreiberNummerStrom,
        Self::NetzbetreiberNummerGas,
        Self::NetzbetreiberNameStrom,
        Self::NetzbetreiberNameGas,
        Self::BilanzierungsGebietNummer,
        Self::MsbNummer,
        Self::MsbName,
        Self::VersorgerNummer,
        Self::VersorgerName,
        Self::GrundversorgerNummerStrom,
        Self::GrundversorgerNameStrom,
        Self::GrundversorgerNummerGas,
        Self::GrundversorgerNameGas,
        Self::KreisName,
        Self::Kreiskennziffer,
        Self::GemeindeName,
        Self::Gemeindekennziffer,
        Self::Postleitzahl,
        Self::Ort,
        Self::Postort,
        Self::EinwohnerzahlGemeinde,
        Self::EinwohnerzahlOrt,
        Self::PlzKmUmkreis,
        Self::Bundesweit,
        Self::PlzBereich,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Regionskriterium::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Regionskriterium`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Regionskriterium::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Regionskriterium;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Regionskriterium::iter_known().count(), Regionskriterium::COUNT);
    /// assert!(Regionskriterium::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Regionskriterium::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Bundeslandkennziffer => "BUNDESLANDKENNZIFFER",
            Self::BundeslandName => "BUNDESLAND_NAME",
            Self::MarktgebietNummer => "MARKTGEBIET_NUMMER",
            Self::MarktgebietName => "MARKTGEBIET_NAME",
            Self::RegelgebietNummer => "REGELGEBIET_NUMMER",
            Self::RegelgebietName => "REGELGEBIET_NAME",
            Self::NetzStrom => "NETZ_STROM",
            Self::NetzGas => "NETZ_GAS",
            Self::NetzbetreiberNummerStrom => "NETZBETREIBER_NUMMER_STROM",
            Self::NetzbetreiberNummerGas => "NETZBETREIBER_NUMMER_GAS",
            Self::NetzbetreiberNameStrom => "NETZBETREIBER_NAME_STROM",
            Self::NetzbetreiberNameGas => "NETZBETREIBER_NAME_GAS",
            Self::BilanzierungsGebietNummer => "BILANZIERUNGS_GEBIET_NUMMER",
            Self::MsbNummer => "MSB_NUMMER",
            Self::MsbName => "MSB_NAME",
            Self::VersorgerNummer => "VERSORGER_NUMMER",
            Self::VersorgerName => "VERSORGER_NAME",
            Self::GrundversorgerNummerStrom => "GRUNDVERSORGER_NUMMER_STROM",
            Self::GrundversorgerNameStrom => "GRUNDVERSORGER_NAME_STROM",
            Self::GrundversorgerNummerGas => "GRUNDVERSORGER_NUMMER_GAS",
            Self::GrundversorgerNameGas => "GRUNDVERSORGER_NAME_GAS",
            Self::KreisName => "KREIS_NAME",
            Self::Kreiskennziffer => "KREISKENNZIFFER",
            Self::GemeindeName => "GEMEINDE_NAME",
            Self::Gemeindekennziffer => "GEMEINDEKENNZIFFER",
            Self::Postleitzahl => "POSTLEITZAHL",
            Self::Ort => "ORT",
            Self::Postort => "POSTORT",
            Self::EinwohnerzahlGemeinde => "EINWOHNERZAHL_GEMEINDE",
            Self::EinwohnerzahlOrt => "EINWOHNERZAHL_ORT",
            Self::PlzKmUmkreis => "PLZ_KM_UMKREIS",
            Self::Bundesweit => "BUNDESWEIT",
            Self::PlzBereich => "PLZ_BEREICH",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Regionskriterium::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Regionskriterium;
    /// assert_eq!(Regionskriterium::from_wire("BUNDESLANDKENNZIFFER"), Ok(Regionskriterium::Bundeslandkennziffer));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Regionskriterium::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Regionskriterium::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "BUNDESLANDKENNZIFFER" => Ok(Self::Bundeslandkennziffer),
            "BUNDESLAND_NAME" => Ok(Self::BundeslandName),
            "MARKTGEBIET_NUMMER" => Ok(Self::MarktgebietNummer),
            "MARKTGEBIET_NAME" => Ok(Self::MarktgebietName),
            "REGELGEBIET_NUMMER" => Ok(Self::RegelgebietNummer),
            "REGELGEBIET_NAME" => Ok(Self::RegelgebietName),
            "NETZ_STROM" => Ok(Self::NetzStrom),
            "NETZ_GAS" => Ok(Self::NetzGas),
            "NETZBETREIBER_NUMMER_STROM" => Ok(Self::NetzbetreiberNummerStrom),
            "NETZBETREIBER_NUMMER_GAS" => Ok(Self::NetzbetreiberNummerGas),
            "NETZBETREIBER_NAME_STROM" => Ok(Self::NetzbetreiberNameStrom),
            "NETZBETREIBER_NAME_GAS" => Ok(Self::NetzbetreiberNameGas),
            "BILANZIERUNGS_GEBIET_NUMMER" => Ok(Self::BilanzierungsGebietNummer),
            "MSB_NUMMER" => Ok(Self::MsbNummer),
            "MSB_NAME" => Ok(Self::MsbName),
            "VERSORGER_NUMMER" => Ok(Self::VersorgerNummer),
            "VERSORGER_NAME" => Ok(Self::VersorgerName),
            "GRUNDVERSORGER_NUMMER_STROM" => Ok(Self::GrundversorgerNummerStrom),
            "GRUNDVERSORGER_NAME_STROM" => Ok(Self::GrundversorgerNameStrom),
            "GRUNDVERSORGER_NUMMER_GAS" => Ok(Self::GrundversorgerNummerGas),
            "GRUNDVERSORGER_NAME_GAS" => Ok(Self::GrundversorgerNameGas),
            "KREIS_NAME" => Ok(Self::KreisName),
            "KREISKENNZIFFER" => Ok(Self::Kreiskennziffer),
            "GEMEINDE_NAME" => Ok(Self::GemeindeName),
            "GEMEINDEKENNZIFFER" => Ok(Self::Gemeindekennziffer),
            "POSTLEITZAHL" => Ok(Self::Postleitzahl),
            "ORT" => Ok(Self::Ort),
            "POSTORT" => Ok(Self::Postort),
            "EINWOHNERZAHL_GEMEINDE" => Ok(Self::EinwohnerzahlGemeinde),
            "EINWOHNERZAHL_ORT" => Ok(Self::EinwohnerzahlOrt),
            "PLZ_KM_UMKREIS" => Ok(Self::PlzKmUmkreis),
            "BUNDESWEIT" => Ok(Self::Bundesweit),
            "PLZ_BEREICH" => Ok(Self::PlzBereich),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Regionskriterium::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Regionskriterium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Regionskriterium {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Regionskriterium {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Regionskriterium {
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
impl crate::Bo4eStrict for Regionskriterium {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Regionskriterium {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Regionskriterium {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Regionskriterium::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Regionskriterium::from_wire`] on a `String` column, or check
/// [`Regionskriterium::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Regionskriterium {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Regionskriterium>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Regionskriterium {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Regionskriterium {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}

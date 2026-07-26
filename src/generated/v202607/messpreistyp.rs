#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Festlegung, welcher Typ von Messung mit einem Preis belegt wird
#[non_exhaustive]
pub enum Messpreistyp {
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G2_5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G2_5"))]
    MesspreisG25,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G4"))]
    MesspreisG4,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G6"))]
    MesspreisG6,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G10"))]
    MesspreisG10,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G16"))]
    MesspreisG16,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G25"))]
    MESSPREISG25,
    #[cfg_attr(feature = "serde", serde(rename = "MESSPREIS_G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MESSPREIS_G40"))]
    MesspreisG40,
    #[cfg_attr(feature = "serde", serde(rename = "ELEKTRONISCHER_AUFSATZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ELEKTRONISCHER_AUFSATZ"))]
    ElektronischerAufsatz,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G2_5"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G2_5"))]
    SmartMeterMesspreisG25,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G4"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G4"))]
    SmartMeterMesspreisG4,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G6"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G6"))]
    SmartMeterMesspreisG6,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G10"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G10"))]
    SmartMeterMesspreisG10,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G16"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G16"))]
    SmartMeterMesspreisG16,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G25"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G25"))]
    SMARTMETERMESSPREISG25,
    #[cfg_attr(feature = "serde", serde(rename = "SMART_METER_MESSPREIS_G40"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SMART_METER_MESSPREIS_G40"))]
    SmartMeterMesspreisG40,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ET_WECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ET_WECHSEL"))]
    VerrechnungspreisEtWechsel,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ET_DREH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ET_DREH"))]
    VerrechnungspreisEtDreh,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ZT_WECHSEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ZT_WECHSEL"))]
    VerrechnungspreisZtWechsel,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_ZT_DREH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_ZT_DREH"))]
    VerrechnungspreisZtDreh,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_L_ET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_L_ET"))]
    VerrechnungspreisLEt,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_L_ZT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_L_ZT"))]
    VerrechnungspreisLZt,
    #[cfg_attr(feature = "serde", serde(rename = "VERRECHNUNGSPREIS_SM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VERRECHNUNGSPREIS_SM"))]
    VerrechnungspreisSm,
    #[cfg_attr(feature = "serde", serde(rename = "AUFSCHLAG_WANDLER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFSCHLAG_WANDLER"))]
    AufschlagWandler,
    #[cfg_attr(feature = "serde", serde(rename = "AUFSCHLAG_TARIFSCHALTUNG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUFSCHLAG_TARIFSCHALTUNG"))]
    AufschlagTarifschaltung,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Messpreistyp {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Messpreistyp::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::MesspreisG25,
        Self::MesspreisG4,
        Self::MesspreisG6,
        Self::MesspreisG10,
        Self::MesspreisG16,
        Self::MESSPREISG25,
        Self::MesspreisG40,
        Self::ElektronischerAufsatz,
        Self::SmartMeterMesspreisG25,
        Self::SmartMeterMesspreisG4,
        Self::SmartMeterMesspreisG6,
        Self::SmartMeterMesspreisG10,
        Self::SmartMeterMesspreisG16,
        Self::SMARTMETERMESSPREISG25,
        Self::SmartMeterMesspreisG40,
        Self::VerrechnungspreisEtWechsel,
        Self::VerrechnungspreisEtDreh,
        Self::VerrechnungspreisZtWechsel,
        Self::VerrechnungspreisZtDreh,
        Self::VerrechnungspreisLEt,
        Self::VerrechnungspreisLZt,
        Self::VerrechnungspreisSm,
        Self::AufschlagWandler,
        Self::AufschlagTarifschaltung,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Messpreistyp::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Messpreistyp`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Messpreistyp::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Messpreistyp::iter_known() {
    ///     println!("{}", v.as_wire());
    /// }
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Messpreistyp::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::MesspreisG25 => "MESSPREIS_G2_5",
            Self::MesspreisG4 => "MESSPREIS_G4",
            Self::MesspreisG6 => "MESSPREIS_G6",
            Self::MesspreisG10 => "MESSPREIS_G10",
            Self::MesspreisG16 => "MESSPREIS_G16",
            Self::MESSPREISG25 => "MESSPREIS_G25",
            Self::MesspreisG40 => "MESSPREIS_G40",
            Self::ElektronischerAufsatz => "ELEKTRONISCHER_AUFSATZ",
            Self::SmartMeterMesspreisG25 => "SMART_METER_MESSPREIS_G2_5",
            Self::SmartMeterMesspreisG4 => "SMART_METER_MESSPREIS_G4",
            Self::SmartMeterMesspreisG6 => "SMART_METER_MESSPREIS_G6",
            Self::SmartMeterMesspreisG10 => "SMART_METER_MESSPREIS_G10",
            Self::SmartMeterMesspreisG16 => "SMART_METER_MESSPREIS_G16",
            Self::SMARTMETERMESSPREISG25 => "SMART_METER_MESSPREIS_G25",
            Self::SmartMeterMesspreisG40 => "SMART_METER_MESSPREIS_G40",
            Self::VerrechnungspreisEtWechsel => "VERRECHNUNGSPREIS_ET_WECHSEL",
            Self::VerrechnungspreisEtDreh => "VERRECHNUNGSPREIS_ET_DREH",
            Self::VerrechnungspreisZtWechsel => "VERRECHNUNGSPREIS_ZT_WECHSEL",
            Self::VerrechnungspreisZtDreh => "VERRECHNUNGSPREIS_ZT_DREH",
            Self::VerrechnungspreisLEt => "VERRECHNUNGSPREIS_L_ET",
            Self::VerrechnungspreisLZt => "VERRECHNUNGSPREIS_L_ZT",
            Self::VerrechnungspreisSm => "VERRECHNUNGSPREIS_SM",
            Self::AufschlagWandler => "AUFSCHLAG_WANDLER",
            Self::AufschlagTarifschaltung => "AUFSCHLAG_TARIFSCHALTUNG",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Messpreistyp::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```rust,ignore
    /// assert!(Messpreistyp::from_wire("NOT_A_REAL_VALUE").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "MESSPREIS_G2_5" => Ok(Self::MesspreisG25),
            "MESSPREIS_G4" => Ok(Self::MesspreisG4),
            "MESSPREIS_G6" => Ok(Self::MesspreisG6),
            "MESSPREIS_G10" => Ok(Self::MesspreisG10),
            "MESSPREIS_G16" => Ok(Self::MesspreisG16),
            "MESSPREIS_G25" => Ok(Self::MESSPREISG25),
            "MESSPREIS_G40" => Ok(Self::MesspreisG40),
            "ELEKTRONISCHER_AUFSATZ" => Ok(Self::ElektronischerAufsatz),
            "SMART_METER_MESSPREIS_G2_5" => Ok(Self::SmartMeterMesspreisG25),
            "SMART_METER_MESSPREIS_G4" => Ok(Self::SmartMeterMesspreisG4),
            "SMART_METER_MESSPREIS_G6" => Ok(Self::SmartMeterMesspreisG6),
            "SMART_METER_MESSPREIS_G10" => Ok(Self::SmartMeterMesspreisG10),
            "SMART_METER_MESSPREIS_G16" => Ok(Self::SmartMeterMesspreisG16),
            "SMART_METER_MESSPREIS_G25" => Ok(Self::SMARTMETERMESSPREISG25),
            "SMART_METER_MESSPREIS_G40" => Ok(Self::SmartMeterMesspreisG40),
            "VERRECHNUNGSPREIS_ET_WECHSEL" => Ok(Self::VerrechnungspreisEtWechsel),
            "VERRECHNUNGSPREIS_ET_DREH" => Ok(Self::VerrechnungspreisEtDreh),
            "VERRECHNUNGSPREIS_ZT_WECHSEL" => Ok(Self::VerrechnungspreisZtWechsel),
            "VERRECHNUNGSPREIS_ZT_DREH" => Ok(Self::VerrechnungspreisZtDreh),
            "VERRECHNUNGSPREIS_L_ET" => Ok(Self::VerrechnungspreisLEt),
            "VERRECHNUNGSPREIS_L_ZT" => Ok(Self::VerrechnungspreisLZt),
            "VERRECHNUNGSPREIS_SM" => Ok(Self::VerrechnungspreisSm),
            "AUFSCHLAG_WANDLER" => Ok(Self::AufschlagWandler),
            "AUFSCHLAG_TARIFSCHALTUNG" => Ok(Self::AufschlagTarifschaltung),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Messpreistyp::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Messpreistyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Messpreistyp {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Messpreistyp {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Messpreistyp {
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
impl crate::Bo4eStrict for Messpreistyp {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Messpreistyp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encode via the canonical wire string (`as_wire`, always available) — no
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Messpreistyp {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Messpreistyp {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Messpreistyp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}

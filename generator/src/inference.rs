use std::collections::HashMap;
use std::sync::LazyLock;

/// Semantic type-inference table.
///
/// Maps JSON property names (or name patterns) to [`crate::ast::FieldType`]
/// values that carry more semantic meaning than the raw JSON Schema type.
///
/// The lookup order is:
/// 1. Exact match on `(parent_struct_name, json_property_name)` — highest priority.
/// 2. Exact match on the JSON property name alone.
/// 3. Suffix match (field name ends with the key), longest-suffix first.
///
/// All keys are *camelCase* JSON property names as they appear in the BO4E schemas.
use crate::ast::{FieldType, PrimitiveType};

// ── Static lookup tables — built once, shared across all infer() calls ────────

/// Per-struct field overrides: `(struct_title, json_field_name) → FieldType`.
///
/// Used for fields where the general-purpose inference rules produce the wrong
/// type for a specific struct.  Takes priority over `EXACT_MAP` and `SUFFIX_MAP`.
///
/// **Schema-override entries** (marked with a `!`) also override the parser's
/// schema-format detection (normally authoritative for `date`/`date-time` fields).
/// Use sparingly — only when upstream schema has `"format": "date-time"` but
/// BDEW practice is date-only (e.g. INVOIC DTM segments with qualifier 102).
static STRUCT_FIELD_MAP: LazyLock<HashMap<(&'static str, &'static str), FieldType>> =
    LazyLock::new(|| {
        HashMap::from([
            // ZusatzAttribut.wert is a free-form JSON value — string, number, boolean,
            // or object.  The "Wert" suffix rule (→ Decimal) must not apply here.
            (("ZusatzAttribut", "wert"), FieldType::JsonValue),
            // ── Schema overrides: date-only BDEW fields ─────────────────────
            // These fields carry `"format": "date-time"` in the upstream BO4E schema
            // but BDEW INVOIC AHB uses date-only qualifiers (DTM qualifier 102,
            // YYYYMMDD). Using `time::Date` avoids silent timezone assumptions and
            // aligns with Zeitraum.startdatum/enddatum (which already use Date).

            // Rechnung.faelligkeitsdatum — INVOIC DTM+92 (qualifier 102)
            (
                ("Rechnung", "faelligkeitsdatum"),
                FieldType::Primitive(PrimitiveType::Date),
            ),
            // Rechnung.rechnungsdatum — INVOIC DTM+137 (qualifier 102)
            (
                ("Rechnung", "rechnungsdatum"),
                FieldType::Primitive(PrimitiveType::Date),
            ),
            // Rechnungsposition.lieferungBis — INVOIC DTM+164 (qualifier 102)
            (
                ("Rechnungsposition", "lieferungBis"),
                FieldType::Primitive(PrimitiveType::Date),
            ),
            // Rechnungsposition.lieferungVon — INVOIC DTM+163 (qualifier 102)
            (
                ("Rechnungsposition", "lieferungVon"),
                FieldType::Primitive(PrimitiveType::Date),
            ),
        ])
    });

/// Checks for a **struct-specific field override** that must take priority
/// even over schema-format-detected types.
///
/// Unlike [`infer_with_parent`], this function only checks [`STRUCT_FIELD_MAP`]
/// and requires a parent struct name.  Use this in the parser after resolving
/// the schema type to allow targeted schema corrections (e.g. `"date-time"` →
/// `Date` for BDEW date-only fields).
pub fn infer_schema_override(parent: &str, json_name: &str) -> Option<FieldType> {
    STRUCT_FIELD_MAP.get(&(parent, json_name)).cloned()
}

/// Exact-name → FieldType mapping.
///
/// Constructed once on first use via `LazyLock`; never rebuilt.
static EXACT_MAP: LazyLock<HashMap<&'static str, FieldType>> = LazyLock::new(|| {
    HashMap::from([
        // Identifier newtypes
        ("marktlokationsId", FieldType::Identifier("MaloId".into())),
        ("messlokationsId", FieldType::Identifier("MeloId".into())),
        ("netzlokationsId", FieldType::Identifier("NeloId".into())),
        (
            "steuerbareRessourceId",
            FieldType::Identifier("SrId".into()),
        ),
        (
            "technischeRessourceId",
            FieldType::Identifier("TrId".into()),
        ),
        ("bilanzkreis", FieldType::Identifier("EicCode".into())),
        ("eicCode", FieldType::Identifier("EicCode".into())),
        // H-05: control area and balance area codes are EIC codes.
        ("marktgebiet", FieldType::Identifier("EicCode".into())),
        ("regelzone", FieldType::Identifier("EicCode".into())),
        ("obisKennzahl", FieldType::Identifier("ObisCode".into())),
        (
            "rollencodenummer",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        ("codenummer", FieldType::Identifier("MarktpartnerId".into())),
        // Grid/supplier operator codes — all carry BDEW Marktpartner IDs.
        (
            "grundversorgercodenr",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        (
            "netzbetreibercodenr",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        (
            "grundzustaendigerMsbCodenr",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        (
            "grundzustaendigerMsbimCodenr",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        (
            "zugeordneteMsbCodenummer",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        // Decimal scalars
        ("prozentsatz", FieldType::Primitive(PrimitiveType::Decimal)),
        // Well-known enum types
        ("waehrungscode", FieldType::BoEnum("Waehrungscode".into())),
        ("mengeneinheit", FieldType::BoEnum("Mengeneinheit".into())),
        // zeiteinheit references the Mengeneinheit enum in BO4E schemas (Mengeneinheit.json)
        ("zeiteinheit", FieldType::BoEnum("Mengeneinheit".into())),
    ])
});

/// Suffix-pattern → FieldType mapping.
///
/// All entries use the exact lowercase suffix as it appears in BO4E JSON
/// property names (e.g. `"vertragsbeginn"` ends with `"beginn"`).
/// Entries are sorted longest-suffix first so that more specific patterns win.
///
/// **Note:** since the parser now reads `"format"` from JSON Schema directly,
/// these suffix rules only fire for string-typed fields with **no** format
/// annotation.  Date/datetime fields with proper `"format": "date"` or
/// `"format": "date-time"` annotations get their types from the schema and
/// are never overridden here.
///
/// The `"datum"` suffix rule has been intentionally removed: German "datum"
/// means "date" (date-only), not "datetime", and fields ending in "datum"
/// in BO4E schemas carry `"format": "date"` annotations which are now handled
/// directly by the parser.  Keeping the rule would produce incorrect
/// `OffsetDateTime` types for any un-annotated future "datum" fields.
static SUFFIX_MAP: LazyLock<Vec<(&'static str, FieldType)>> = LazyLock::new(|| {
    vec![
        // Identifiers
        ("marktlokationsid", FieldType::Identifier("MaloId".into())),
        ("messlokationsid", FieldType::Identifier("MeloId".into())),
        ("netzlokationsid", FieldType::Identifier("NeloId".into())),
        // Timestamps → time::OffsetDateTime (feature-gated).
        // These fire only when the schema field has "type": "string" with no
        // "format" annotation — a schema omission rather than a design choice.
        (
            "zeitpunkt",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        (
            "beginn",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        ("ende", FieldType::Primitive(PrimitiveType::OffsetDateTime)),
        // Monetary / quantity decimals
        ("betrag", FieldType::Primitive(PrimitiveType::Decimal)),
        ("preis", FieldType::Primitive(PrimitiveType::Decimal)),
        ("menge", FieldType::Primitive(PrimitiveType::Decimal)),
        ("wert", FieldType::Primitive(PrimitiveType::Decimal)),
    ]
});

// ─── Public API ───────────────────────────────────────────────────────────────

/// Returns the semantic field-type override for a given JSON property name,
/// or `None` if no semantic mapping is registered.
///
/// Optionally accepts the parent struct title (e.g. `"ZusatzAttribut"`) to
/// allow struct-specific overrides that supersede the general inference rules.
///
/// Lookup order:
/// 1. Struct+field exact match in [`STRUCT_FIELD_MAP`] (highest priority).
/// 2. Exact match in [`EXACT_MAP`].
/// 3. Suffix match in [`SUFFIX_MAP`] (longest-suffix-first wins).
pub fn infer(json_name: &str) -> Option<FieldType> {
    infer_with_parent(None, json_name)
}

/// Like [`infer`] but also checks the per-struct override table first.
pub fn infer_with_parent(parent: Option<&str>, json_name: &str) -> Option<FieldType> {
    // 1. Struct-specific override (highest priority).
    if let Some(p) = parent {
        if let Some(ft) = STRUCT_FIELD_MAP.get(&(p, json_name)) {
            return Some(ft.clone());
        }
    }
    // 2. Exact name match.
    if let Some(ft) = EXACT_MAP.get(json_name) {
        return Some(ft.clone());
    }
    // 3. Suffix match (longest first).
    for (suffix, ft) in SUFFIX_MAP.iter() {
        if json_name.ends_with(suffix) {
            return Some(ft.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ───────────────────────────────────────────────────────────

    fn ident(name: &str) -> FieldType {
        FieldType::Identifier(name.into())
    }
    fn bo_enum(name: &str) -> FieldType {
        FieldType::BoEnum(name.into())
    }
    const DT: FieldType = FieldType::Primitive(PrimitiveType::OffsetDateTime);
    const DEC: FieldType = FieldType::Primitive(PrimitiveType::Decimal);

    // ── Exact identifier matches ──────────────────────────────────────────

    #[test]
    fn exact_identifier_matches() {
        // Primary BO identifiers
        assert_eq!(infer("marktlokationsId"), Some(ident("MaloId")));
        assert_eq!(infer("messlokationsId"), Some(ident("MeloId")));
        assert_eq!(infer("netzlokationsId"), Some(ident("NeloId")));
        assert_eq!(infer("steuerbareRessourceId"), Some(ident("SrId")));
        assert_eq!(infer("technischeRessourceId"), Some(ident("TrId")));
        // EIC codes: direct + balance-group + area uses
        assert_eq!(infer("bilanzkreis"), Some(ident("EicCode")));
        assert_eq!(infer("eicCode"), Some(ident("EicCode")));
        assert_eq!(infer("marktgebiet"), Some(ident("EicCode")));
        assert_eq!(infer("regelzone"), Some(ident("EicCode")));
        // OBIS
        assert_eq!(infer("obisKennzahl"), Some(ident("ObisCode")));
        // Market partner codes
        assert_eq!(infer("rollencodenummer"), Some(ident("MarktpartnerId")));
        assert_eq!(infer("codenummer"), Some(ident("MarktpartnerId")));
        assert_eq!(infer("grundversorgercodenr"), Some(ident("MarktpartnerId")));
        assert_eq!(infer("netzbetreibercodenr"), Some(ident("MarktpartnerId")));
        assert_eq!(
            infer("grundzustaendigerMsbCodenr"),
            Some(ident("MarktpartnerId"))
        );
        assert_eq!(
            infer("grundzustaendigerMsbimCodenr"),
            Some(ident("MarktpartnerId"))
        );
        assert_eq!(
            infer("zugeordneteMsbCodenummer"),
            Some(ident("MarktpartnerId"))
        );
    }

    // ── Exact enum matches ────────────────────────────────────────────────

    #[test]
    fn exact_enum_matches() {
        assert_eq!(infer("waehrungscode"), Some(bo_enum("Waehrungscode")));
        assert_eq!(infer("mengeneinheit"), Some(bo_enum("Mengeneinheit")));
        // zeiteinheit references Mengeneinheit in BO4E schemas
        assert_eq!(infer("zeiteinheit"), Some(bo_enum("Mengeneinheit")));
    }

    // ── Exact decimal matches ─────────────────────────────────────────────

    #[test]
    fn exact_decimal_matches() {
        assert_eq!(infer("prozentsatz"), Some(DEC));
    }

    // ── Suffix: datetime (ends with "beginn", "ende", "zeitpunkt") ──────────
    // NOTE: the "datum" suffix rule was removed — see SUFFIX_MAP comment.
    // All "datum" fields in BO4E schemas carry "format": "date" annotations
    // and are now typed as time::Date via the parser's schema-format detection.

    #[test]
    fn suffix_datetime_matches() {
        // "beginn" suffix
        assert_eq!(infer("vertragsbeginn"), Some(DT));
        assert_eq!(infer("bilanzierungsbeginn"), Some(DT));
        assert_eq!(infer("lieferbeginn"), Some(DT));
        assert_eq!(infer("vertragsteilbeginn"), Some(DT));
        // "ende" suffix
        assert_eq!(infer("vertragsende"), Some(DT));
        assert_eq!(infer("bilanzierungsende"), Some(DT));
        assert_eq!(infer("vertragsteilende"), Some(DT));
        // "zeitpunkt" suffix
        assert_eq!(infer("veroeffentlichungszeitpunkt"), Some(DT));
    }

    // "datum" suffix no longer infers to OffsetDateTime (removed rule).
    // "datum" in German means "date" (date-only) — these fields carry
    // "format": "date" in the BO4E JSON Schema and are correctly typed
    // as time::Date via parser schema-format detection.
    #[test]
    fn datum_suffix_no_longer_infers_to_offsetdatetime() {
        // These used to return Some(DT) with the old "datum" inference rule.
        // Now they return None (no inference override), so the parser's
        // schema-format detection produces the correct time::Date type.
        assert_eq!(infer("angebotsdatum"), None);
        assert_eq!(infer("rechnungsdatum"), None);
        assert_eq!(infer("faelligkeitsdatum"), None);
        assert_eq!(infer("geburtsdatum"), None);
        assert_eq!(infer("startdatum"), None);
        assert_eq!(infer("enddatum"), None);
    }

    // ── Suffix: decimal (ends with "wert", "preis", "betrag", "menge") ───

    #[test]
    fn suffix_decimal_matches() {
        // "wert" suffix
        assert_eq!(infer("wert"), Some(DEC));
        assert_eq!(infer("zeitreihenwert"), Some(DEC)); // struct-level override tested separately
                                                        // "preis" suffix
        assert_eq!(infer("grundpreis"), Some(DEC));
        assert_eq!(infer("arbeitspreis"), Some(DEC));
        assert_eq!(infer("einzelpreis"), Some(DEC));
        // "betrag" suffix
        assert_eq!(infer("gesamtbetrag"), Some(DEC));
        assert_eq!(infer("nettobetrag"), Some(DEC));
        // "menge" suffix — the suffix map uses case-sensitive "menge" (all lowercase).
        // Any BO4E field whose JSON name ends with "menge" resolves to Decimal.
        assert_eq!(infer("geliefertemenge"), Some(DEC)); // all-lowercase — matches
        assert_eq!(infer("summierungsmenge"), Some(DEC)); // compound — matches
                                                          // camelCase with uppercase does NOT match (different byte sequence)
        assert_eq!(infer("gelieferteMenge"), None); // capital M — no match
    }

    // ── Struct-specific overrides ─────────────────────────────────────────

    #[test]
    fn zusatz_attribut_wert_is_json_value_not_decimal() {
        // ZusatzAttribut.wert is free-form (string/number/bool/object).
        // The "wert" suffix rule (→ Decimal) must NOT fire here.
        assert_eq!(
            infer_with_parent(Some("ZusatzAttribut"), "wert"),
            Some(FieldType::JsonValue)
        );
    }

    #[test]
    fn other_structs_wert_still_decimal() {
        // Struct override is per-(struct, field) — other structs still get Decimal.
        assert_eq!(infer_with_parent(Some("Zeitreihenwert"), "wert"), Some(DEC),);
        assert_eq!(infer_with_parent(Some("Betrag"), "wert"), Some(DEC),);
        assert_eq!(infer_with_parent(Some("Menge"), "wert"), Some(DEC),);
    }

    // ── No-match cases ────────────────────────────────────────────────────

    #[test]
    fn unknown_fields_return_none() {
        assert_eq!(infer("someUnknownField"), None);
        assert_eq!(infer("beschreibung"), None);
        assert_eq!(infer("name"), None);
        assert_eq!(infer("version"), None);
        assert_eq!(infer("id"), None);
    }

    // ── Case sensitivity: BO4E names are always lowercase JSON camelCase ──

    #[test]
    fn case_sensitive_no_camel_match() {
        // Suffix map uses lowercase keys; camelCase variants must NOT match.
        assert_eq!(infer("vertragsBeginn"), None); // camelCase — wrong
        assert_eq!(infer("vertragsbeginn"), Some(DT)); // canonical lowercase — ok
        assert_eq!(infer("Wert"), None); // uppercase first letter — wrong
        assert_eq!(infer("WERT"), None); // all-caps — wrong
    }

    // ── Suffix uniqueness invariant ───────────────────────────────────────

    #[test]
    fn no_duplicate_suffixes() {
        let suffixes: Vec<&str> = SUFFIX_MAP.iter().map(|(s, _)| *s).collect();
        let mut seen = std::collections::HashSet::new();
        for s in &suffixes {
            assert!(seen.insert(*s), "duplicate suffix in SUFFIX_MAP: {s:?}");
        }
    }

    // ── Suffix priority: longest suffix wins ─────────────────────────────

    #[test]
    fn suffix_priority_longest_wins() {
        // "marktlokationsid" is in SUFFIX_MAP and is longer than any single-word suffix.
        // It must resolve to MaloId, not trigger the generic "menge/wert/..." rules.
        assert_eq!(infer("marktlokationsid"), Some(ident("MaloId")));
        assert_eq!(infer("messlokationsid"), Some(ident("MeloId")));
        assert_eq!(infer("netzlokationsid"), Some(ident("NeloId")));
    }

    // ── Struct override priority: beats exact and suffix matches ─────────

    #[test]
    fn struct_override_wins_over_exact_and_suffix() {
        // "wert" is an exact match to Decimal, but the struct override for
        // ZusatzAttribut.wert must take priority.
        assert_eq!(
            infer_with_parent(Some("ZusatzAttribut"), "wert"),
            Some(FieldType::JsonValue),
        );
        // Without parent, falls through to exact/suffix rule.
        assert_eq!(infer("wert"), Some(DEC));
    }
}

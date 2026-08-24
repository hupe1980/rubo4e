//! Semantic field typing: which BO4E `"type": "string"` properties are really a
//! validated domain identifier.
//!
//! BO4E declares every identifier as a bare string. This table is what turns
//! `Marktlokation.marktlokationsId` into a `MaloId` that verifies its own BDEW
//! check digit instead of a `String` that does not. Three rules govern it:
//!
//! 1. **Keyed on `(struct, field)`** — never a bare name, never a suffix. BO4E
//!    reuses names: `Marktlokation.marktgebiet` is *"Code vom EIC"*, while
//!    `MarktgebietInfo.marktgebiet` is *"Der Name des Marktgebietes"*.
//! 2. **The schema wins.** The parser consults this table only for properties
//!    typed as a plain, unannotated `"string"`. A `$ref`, a `"format"`, or
//!    `"type": "number"` is authoritative.
//! 3. **Type only what the schema names** — "EIC-Nummer", "OBIS-Kennzahl",
//!    "Codenummer des Netzbetreibers" — not a field the schema calls merely *a
//!    code*. A newtype that rejects a value takes the enclosing object down with
//!    it; a missing one costs the caller one `EicCode::try_from(&s)`.
//! 4. **Weigh the blast radius even when rule 3 is satisfied.** `Zahlungsinformation`
//!    hangs off `Rechnung` and nothing else, so typing its `iban` as the
//!    checksum-verified [`Iban`] would make a masked IBAN — `DE89 **** **** 3000`,
//!    routine on an invoice — destroy the entire `Rechnung`: line items, amounts,
//!    periods and all. The same goes for `bic`. Both types exist and are worth
//!    using; `Zahlungsinformation::iban_checked()` runs the check on demand
//!    without putting the invoice at risk.
//!

//!
//! See `site/content/docs/generator.md` for the resulting table and the fields
//! left untyped under rule 3.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::ast::FieldType;

/// `(struct title, JSON property name) → FieldType`.
///
/// Exhaustive: a property not listed here keeps the type its schema declares.
static FIELD_TYPES: LazyLock<HashMap<(&'static str, &'static str), FieldType>> =
    LazyLock::new(|| {
        fn id(name: &'static str) -> FieldType {
            FieldType::Identifier(name.into())
        }
        HashMap::from([
            // ── Primary location identifiers ─────────────────────────────────
            (("Marktlokation", "marktlokationsId"), id("MaloId")),
            (("Bilanzierung", "marktlokationsId"), id("MaloId")),
            (("Ausschreibungsdetail", "marktlokationsId"), id("MaloId")),
            (("Messlokation", "messlokationsId"), id("MeloId")),
            (("Netzlokation", "netzlokationsId"), id("NeloId")),
            // ── Redispatch 2.0 resource identifiers ──────────────────────────
            (("SteuerbareRessource", "steuerbareRessourceId"), id("SrId")),
            (("TechnischeRessource", "technischeRessourceId"), id("TrId")),
            // ── EIC codes ────────────────────────────────────────────────────
            //
            // Only where the schema says the field carries a *code*.  The same
            // names elsewhere hold human-readable names and stay `String`.
            (("Marktlokation", "marktgebiet"), id("EicCode")),
            (("Marktlokation", "regelzone"), id("EicCode")),
            // Left as the general `EicCode`: a German electricity Bilanzkreis is
            // a party code (`11X…`), so `BilanzkreisId` would be tighter — but
            // this field also carries gas Bilanzkreise (GaBi Gas BK7-14-020),
            // whose object type is not established here.  Narrowing it would turn
            // an unverified assumption into a hard deserialization failure on
            // real payloads; callers opt in via `BilanzkreisId::try_from(eic)`.
            (("Bilanzierung", "bilanzkreis"), id("EicCode")),
            // Both name their format outright: "De EIC-Nummer der Regelzone" and
            // "EIC-Code des Regel- oder Marktgebietes … Z.B. '10YDE-EON------1'".
            (
                ("StandorteigenschaftenStrom", "regelzoneEic"),
                id("EicCode"),
            ),
            (("Fremdkostenposition", "gebietcodeEic"), id("EicCode")),
            // A Bilanzierungsgebiet is an area code.  The schema documents this
            // as "Die EIC-Nummer des Bilanzierungsgebietes" but declares it a
            // plain string; all 645 codes in the TSOs' published
            // VNB-Bilanzierungsgebiete list carry object type 'Y'.
            (
                ("StandorteigenschaftenStrom", "bilanzierungsgebietEic"),
                id("BilanzierungsgebietId"),
            ),
            // ── BDEW Marktpartner codes ──────────────────────────────────────
            (
                ("Marktteilnehmer", "rollencodenummer"),
                id("MarktpartnerId"),
            ),
            (
                ("Marktlokation", "grundversorgercodenr"),
                id("MarktpartnerId"),
            ),
            (
                ("Marktlokation", "netzbetreibercodenr"),
                id("MarktpartnerId"),
            ),
            (
                ("Messlokation", "grundzustaendigerMsbCodenr"),
                id("MarktpartnerId"),
            ),
            (
                ("Messlokation", "grundzustaendigerMsbimCodenr"),
                id("MarktpartnerId"),
            ),
            (
                ("Netzlokation", "grundzustaendigerMsbCodenr"),
                id("MarktpartnerId"),
            ),
            (
                ("SteuerbareRessource", "zugeordneteMsbCodenummer"),
                id("MarktpartnerId"),
            ),
            // ── OBIS ─────────────────────────────────────────────────────────
            //
            // `Netzlokation` spells the property `obiskennzahl`, the other three
            // `obisKennzahl`.  The casing is an upstream inconsistency, not a
            // difference in meaning — all four carry an OBIS code.
            (("Energiemenge", "obisKennzahl"), id("ObisCode")),
            (("Lastgang", "obisKennzahl"), id("ObisCode")),
            (("Zaehlwerk", "obisKennzahl"), id("ObisCode")),
            (("Netzlokation", "obiskennzahl"), id("ObisCode")),
        ])
    });

/// Returns the domain type for `(parent, json_name)`, or `None` to keep the type
/// the schema declares.
///
/// `parent` is the enclosing schema's title. Without one there is no lookup: a
/// field name alone is not enough to decide.
pub fn infer_with_parent(parent: Option<&str>, json_name: &str) -> Option<FieldType> {
    FIELD_TYPES.get(&(parent?, json_name)).cloned()
}

/// Every `(struct, field)` pair the table types, for the drift guard in
/// `generator/tests/round_trip.rs`.
pub fn typed_fields() -> impl Iterator<Item = (&'static str, &'static str)> {
    FIELD_TYPES.keys().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident(name: &str) -> FieldType {
        FieldType::Identifier(name.into())
    }

    #[test]
    fn typed_fields_resolve() {
        assert_eq!(
            infer_with_parent(Some("Marktlokation"), "marktlokationsId"),
            Some(ident("MaloId"))
        );
        assert_eq!(
            infer_with_parent(Some("Marktteilnehmer"), "rollencodenummer"),
            Some(ident("MarktpartnerId"))
        );
        assert_eq!(
            infer_with_parent(Some("Zaehlwerk"), "obisKennzahl"),
            Some(ident("ObisCode"))
        );
    }

    /// The same name in a different struct must not inherit the type.
    #[test]
    fn homonyms_in_other_structs_are_untyped() {
        assert_eq!(
            infer_with_parent(Some("Marktlokation"), "marktgebiet"),
            Some(ident("EicCode"))
        );
        // "Der Name des Marktgebietes" — a name, not a code.
        assert_eq!(
            infer_with_parent(Some("MarktgebietInfo"), "marktgebiet"),
            None
        );

        assert_eq!(
            infer_with_parent(Some("Marktlokation"), "regelzone"),
            Some(ident("EicCode"))
        );
        // "Der Name der Regelzone".
        assert_eq!(
            infer_with_parent(Some("StandorteigenschaftenStrom"), "regelzone"),
            None
        );
    }

    /// `Kontaktweg.kontaktwert` is *"Die Nummer oder E-Mail-Adresse"*. A suffix
    /// rule on `wert` typed it as a `Decimal`, so any object carrying a contact
    /// method failed to deserialize whole.
    #[test]
    fn kontaktwert_is_not_a_decimal() {
        assert_eq!(infer_with_parent(Some("Kontaktweg"), "kontaktwert"), None);
    }

    /// No suffix, prefix, or bare-name matching: an unlisted pair keeps its
    /// schema type however much its name resembles one that is listed.
    #[test]
    fn unlisted_pairs_are_untyped() {
        for (parent, field) in [
            ("Betrag", "wert"),
            ("Menge", "wert"),
            ("ZusatzAttribut", "wert"),
            ("Preisstaffel", "preis"),
            ("Vertrag", "vertragsbeginn"),
            ("Rechnung", "rechnungsdatum"),
            ("Marktlokation", "someFutureField"),
            ("Marktlokation", "xmarktlokationsId"),
        ] {
            assert_eq!(
                infer_with_parent(Some(parent), field),
                None,
                "{parent}.{field} must keep its schema type"
            );
        }
    }

    #[test]
    fn without_a_parent_nothing_resolves() {
        assert_eq!(infer_with_parent(None, "marktlokationsId"), None);
    }
}

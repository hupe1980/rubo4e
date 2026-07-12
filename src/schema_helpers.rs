//! Schema helper functions for schemars integration.
//!
//! These are referenced from generated code via `#[schemars(schema_with = "...")]`
//! to provide richer JSON Schema annotations for types that schemars 1.x does
//! not natively support (notably `time::OffsetDateTime` and `time::Date`).

/// Returns a JSON Schema representing an ISO 8601 / RFC 3339 date-time string.
///
/// Equivalent to `{ "type": "string", "format": "date-time" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::datetime_schema")]`
/// on required `time::OffsetDateTime` fields in generated structs.
pub fn datetime_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "format": "date-time"
    })
}

/// Returns a JSON Schema representing a nullable ISO 8601 / RFC 3339 date-time string.
///
/// Equivalent to `{ "type": ["string", "null"], "format": "date-time" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::opt_datetime_schema")]`
/// on `Option<time::OffsetDateTime>` fields in generated structs.
pub fn opt_datetime_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": ["string", "null"],
        "format": "date-time"
    })
}

/// Returns a JSON Schema representing an ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Equivalent to `{ "type": "string", "format": "date" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::date_schema")]`
/// on required `time::Date` fields in generated structs.
pub fn date_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "format": "date"
    })
}

/// Returns a JSON Schema representing a nullable ISO 8601 date-only string (`YYYY-MM-DD`).
///
/// Equivalent to `{ "type": ["string", "null"], "format": "date" }`.
///
/// Used as `#[schemars(schema_with = "crate::schema_helpers::opt_date_schema")]`
/// on `Option<time::Date>` fields in generated structs.
pub fn opt_date_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": ["string", "null"],
        "format": "date"
    })
}

// ── Identifier schemas ────────────────────────────────────────────────────────

/// JSON Schema for [`MaloId`](crate::identifiers::MaloId): 11-digit BDEW MaLo-ID.
pub fn malo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^[0-9]{11}$",
        "description": "11-stellige BDEW Marktlokations-ID mit alternierend gewichtetem BDEW-Prüfziffer",
        "examples": ["51238696780"]
    })
}

/// JSON Schema for [`MeloId`](crate::identifiers::MeloId): 33-character MeLo-ID.
pub fn melo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^[A-Z]{2}[A-Za-z0-9]{31}$",
        "description": "33-stellige Messlokations-ID: 2-stelliger ISO-3166-1-Ländercode + 31 alphanumerische Zeichen",
        "examples": ["DE0000000000000000000000000000001"]
    })
}

/// JSON Schema for [`MarktpartnerId`](crate::identifiers::MarktpartnerId): 13-digit MP-ID.
pub fn marktpartner_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^[0-9]{13}$",
        "description": "13-stellige Marktpartner-ID (BDEW-Codenummer Strom mit Prefix 99, DVGW-Codenummer Gas mit Prefix 98, oder GS1 GLN)",
        "examples": ["9900357000004"]
    })
}

/// JSON Schema for [`NeloId`](crate::identifiers::NeloId): 11-character BDEW NeLo-ID.
///
/// Format: Codetyp `'E'` (position 1) + 9 uppercase alphanumeric characters
/// (positions 2–10) + 1 numeric ASCII-Verfahren check digit (position 11).
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §4.
pub fn nelo_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^E[A-Z0-9]{9}[0-9]$",
        "description": "11-stellige BDEW Netzlokations-ID (NeLo-ID): Codetyp 'E' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
        "examples": ["E0000000019"]
    })
}

/// JSON Schema for [`SrId`](crate::identifiers::SrId): 11-character Steuerbare-Ressource-ID.
///
/// Format: Codetyp `'C'` (position 1) + 9 uppercase alphanumeric characters
/// (positions 2–10) + 1 numeric ASCII-Verfahren check digit (position 11).
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §6.3/§6.6.
pub fn sr_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^C[A-Z0-9]{9}[0-9]$",
        "description": "11-stellige Steuerbare-Ressource-ID (SR-ID): Codetyp 'C' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
        "examples": ["C0000000003"]
    })
}

/// JSON Schema for [`TrId`](crate::identifiers::TrId): 11-character Technische-Ressource-ID.
///
/// Format: Codetyp `'D'` (position 1) + 9 uppercase alphanumeric characters
/// (positions 2–10) + 1 numeric ASCII-Verfahren check digit (position 11).
/// Defined by BDEW "Identifikatoren in der Marktkommunikation" v1.2 (February 2025), §6.2/§6.6.
pub fn tr_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^D[A-Z0-9]{9}[0-9]$",
        "description": "11-stellige Technische-Ressource-ID (TR-ID): Codetyp 'D' + 9 alphanumerische Zeichen [A-Z0-9] + ASCII-Verfahren-Prüfziffer",
        "examples": ["D0000000002"]
    })
}

/// JSON Schema for [`BilanzkreisId`](crate::identifiers::BilanzkreisId): 16-character EIC
/// code restricted to type character `'Z'` (Bilanzierungszone).
pub fn bilanzkreis_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^[A-Z0-9]{2}Z[A-Z0-9-]{12}[A-Z0-9]$",
        "description": "16-stellige EIC-Code mit Typ 'Z' (Bilanzkreis-ID / Bilanzierungszone): Positions-1+2 LIO-Kennung + 'Z' + 12 Zeichen Körper + ENTSO-E-Prüfzeichen",
        "examples": ["11ZVEW---------O"]
    })
}

/// JSON Schema for [`AkivId`](crate::identifiers::AkivId): Aktivierungsidentifikator
/// für Redispatch 2.0 (BDEW WiM AHB BK6-24-174, §14a EnWG).
pub fn akiv_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 36,
        "pattern": "^[!-~]{1,36}$",
        "description": "Aktivierungsidentifikator für Redispatch 2.0 und §14a EnWG Modul 3 (BDEW WiM AHB BK6-24-174): 1–36 druckbare ASCII-Zeichen (UUID-kompatibel)",
        "examples": ["550e8400-e29b-41d4-a716-446655440000"]
    })
}

/// JSON Schema for [`TranchennummerId`](crate::identifiers::TranchennummerId): 1–6 digit
/// numeric tranche identifier for MABIS Bilanzkreisabrechnung (PID 13003).
pub fn tranchennummer_id_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "pattern": "^(0|[1-9][0-9]{0,5})$",
        "description": "Tranchennummer für MABIS Bilanzkreisabrechnung (PID 13003): 1–6-stellige Dezimalzahl ohne führende Nullen (Wertebereich 0–999 999)",
        "examples": ["1", "42", "999999"]
    })
}

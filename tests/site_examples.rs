//! Compiles and runs the examples shown on the landing page and in the README.
//!
//! Those snippets live in Markdown, so nothing else type-checks them. Keeping an
//! executable copy here means an API change breaks the build instead of quietly
//! leaving the published documentation wrong.
//!
//! Sources:
//! - `site/snippets/quickstart.md` — the landing page round-trip
//! - `README.md` § Quick Start — the builder walkthrough
#![cfg(all(
    feature = "versioned",
    feature = "json",
    feature = "builder",
    feature = "validate"
))]

use rubo4e::current::{Marktlokation, Sparte, Vertrag};
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};
use rubo4e::prelude::*;
use rubo4e::Bo4eStrict;

/// `site/snippets/quickstart.md`
#[test]
fn landing_page_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","sparte":"STROM"}"#;

    let malo =
        Marktlokation::from_json_german_hardened(body, JsonParseLimits::untrusted_defaults())?;

    malo.ensure_known_enums()?;

    let id = malo.marktlokations_id.as_ref().expect("id present");
    assert_eq!(id.as_ref(), "51238696781");
    assert_eq!(malo.sparte, Some(Sparte::Strom));
    Ok(())
}

/// The same snippet must actually reject an out-of-schema enum value — otherwise
/// the `ensure_known_enums()` line above is decorative.
#[test]
fn landing_page_round_trip_rejects_unknown_enum() {
    let body = r#"{"_typ":"MARKTLOKATION","marktlokationsId":"51238696781","sparte":"PLASMA"}"#;
    let malo =
        Marktlokation::from_json_german_hardened(body, JsonParseLimits::untrusted_defaults())
            .expect("lenient decode succeeds");
    let err = malo
        .ensure_known_enums()
        .expect_err("PLASMA is not a Sparte in this schema version");
    assert_eq!(err.paths, ["sparte"]);
}

/// `README.md` § Quick Start
#[test]
fn readme_quick_start() -> Result<(), Box<dyn std::error::Error>> {
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2026-001".to_string())
        .build();

    // Reached through the prelude — the README states no direct garde dependency
    // is needed, so this import path is part of the contract.
    vertrag.validate()?;

    let json = vertrag.to_json_german()?;
    assert!(
        json.contains(r#""sparte":"STROM""#),
        "unexpected JSON: {json}"
    );
    Ok(())
}

// ─── Identifier snippets ─────────────────────────────────────────────────────
//
// `README.md` § Identifiers and `site/content/docs/identifiers.md` show these
// verbatim. They document the ENTSO-E object-type distinction and the OBIS
// canonical form, both of which were wrong before — so an executable copy is
// what keeps the published text honest.

/// `README.md` § EIC codes and object types, and the site's
/// "Object type (position 3)" and "BilanzkreisId and BilanzierungsgebietId"
/// sections.
#[test]
fn docs_eic_object_types() -> Result<(), Box<dyn std::error::Error>> {
    use rubo4e::identifiers::{BilanzierungsgebietId, BilanzkreisId, EicCode, EicType};

    let area = EicCode::new("10YDE-EON------1")?; // TenneT control area
    assert_eq!(area.type_char(), 'Y');
    assert_eq!(area.eic_type(), EicType::Area);

    let party = EicCode::new("11XSUEDWESTSTRO8")?; // a Bilanzkreis
    assert_eq!(party.eic_type(), EicType::Party);

    assert_eq!(EicCode::compute_check_char("10XDE-EON-NETZ-"), Some('C'));
    assert_eq!(
        EicCode::new_from_prefix("10YDE-EON------")?.as_ref(),
        "10YDE-EON------1"
    );

    // The restricted types pin position 3, so the roles cannot be confused.
    let bk = BilanzkreisId::from_prefix("11XSUEDWESTSTRO")?;
    assert_eq!(bk.as_ref(), "11XSUEDWESTSTRO8");
    assert_eq!(bk.to_eic_code().eic_type(), EicType::Party);

    let eic: EicCode = bk.clone().into();
    assert_eq!(BilanzkreisId::try_from(eic)?, bk);

    let bg = BilanzierungsgebietId::new("11YN-0000-0001-Q")?;
    assert_eq!(bg.to_eic_code().eic_type(), EicType::Area);

    assert!(BilanzkreisId::new("11YN-0000-0001-Q").is_err());
    assert!(BilanzierungsgebietId::new("11XSUEDWESTSTRO8").is_err());
    assert!(BilanzkreisId::new("10YDE-EON------1").is_err());
    Ok(())
}

/// `README.md` § OBIS codes and the site's "Canonical form" section.
#[test]
fn docs_obis_canonical_form() -> Result<(), Box<dyn std::error::Error>> {
    use rubo4e::identifiers::ObisCode;

    ObisCode::new("1-0:1.8.1")?; // electricity, active energy forward, tariff 1
    ObisCode::new("7-0:3.1.0")?; // gas, volume
    ObisCode::new("0-0:0.0.0")?; // C=0 — general metering group
    assert!(ObisCode::new("not-an-obis").is_err());

    // Canonicalisation
    assert_eq!(ObisCode::new("1.8.1&255")?, ObisCode::new("1.8.1*255")?);
    assert_eq!(
        ObisCode::new("01-00:01.08.00")?,
        ObisCode::new("1-0:1.8.0")?
    );
    assert_eq!(ObisCode::new("01-00:01.08.00")?.as_str(), "1-0:1.8.0");

    // Value groups are octets
    assert!(ObisCode::new("1-0:1.8.256").is_err());

    // Stored components
    let parts = ObisCode::new("1-0:1.8.0*255")?.components();
    assert_eq!(
        (parts.a, parts.b, parts.c, parts.d, parts.e, parts.f),
        (Some(1), Some(0), 1, 8, Some(0), Some(255))
    );

    assert_eq!(ObisCode::new("1-0:1.8.0*255")?.to_pia_string(), "1-0:1.8.0");
    Ok(())
}

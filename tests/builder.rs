//! E07-S31 — Builder compile-fail tests.
//!
//! Verifies that `typed_builder` correctly enforces required fields at compile
//! time.  All generated BO/COM optional fields carry `#[builder(default)]`, so
//! they can be omitted.  A struct that exposes a *required* field (no `default`)
//! must fail to build when that field is omitted.

#[cfg(feature = "builder")]
#[test]
fn required_field_must_be_supplied() {
    // Happy path: optional fields with #[builder(default)] can be omitted.
    // This exercises the generated builder on a real BO type.
    #[cfg(feature = "versioned")]
    {
        use rubo4e::v202501::Vertrag;
        let _ = Vertrag::builder().build();
    }
}

#[cfg(all(feature = "builder", feature = "versioned"))]
#[test]
fn bo_builder_auto_sets_typ_discriminant() {
    use rubo4e::v202501::{BoTyp, Vertrag};

    let vertrag = Vertrag::builder().build();
    assert_eq!(vertrag.typ, Some(BoTyp::Vertrag));
}

/// `setter(into)` replaces `strip_option`: the setter now accepts *both* `T`
/// (via `From<T> for Option<T>`) and `Option<T>` (identity conversion).
///
/// This is the fix for the user feedback about EDIFACT-parsing ergonomics: code
/// that extracts `Option<String>` fields from segments can pass them directly
/// without an intermediate `if let Some(v) = opt { builder.field(v) }`.
#[cfg(all(feature = "builder", feature = "versioned"))]
#[test]
fn builder_setter_into_accepts_option_and_value() {
    use rubo4e::v202501::Betrag;

    // Use `organisationsname` (always `Option<String>`) as the test field so the
    // assertion is independent of the `decimal` feature flag.
    use rubo4e::v202501::Geschaeftspartner;

    // 1. Passing `T` directly — same ergonomics as the old strip_option.
    let g1 = Geschaeftspartner::builder()
        .organisationsname("ACME GmbH".to_owned())
        .build();
    assert_eq!(g1.organisationsname, Some("ACME GmbH".to_owned()));

    // 2. Passing `Option<T>` directly — the NEW capability enabled by setter(into).
    //    This is the key use-case from EDIFACT segment parsing where each field
    //    may or may not be present.
    let extracted: Option<String> = Some("Muster AG".to_owned());
    let g2 = Geschaeftspartner::builder()
        .organisationsname(extracted.clone())
        .build();
    assert_eq!(g2.organisationsname, extracted);

    // 3. Passing `None` explicitly also works.
    let g3 = Geschaeftspartner::builder()
        .organisationsname(None::<String>)
        .build();
    assert_eq!(g3.organisationsname, None);

    // 4. Also verify on Betrag with the correct wert type (feature-conditional).
    #[cfg(not(feature = "decimal"))]
    {
        let b = Betrag::builder().wert(Some("12.50".to_owned())).build();
        assert_eq!(b.wert, Some("12.50".to_owned()));
    }
    #[cfg(feature = "decimal")]
    {
        use rust_decimal::Decimal;
        use std::str::FromStr;
        let amount = Decimal::from_str("12.50").unwrap();
        let b = Betrag::builder().wert(Some(amount)).build();
        assert_eq!(b.wert, Some(amount));
        // None also works
        let b_none = Betrag::builder().wert(None::<Decimal>).build();
        assert_eq!(b_none.wert, None);
    }
    let _ = (g1, g2, g3);
}

/// External crates can use `..Default::default()` (functional update syntax)
/// because `_additional` is now `pub` (with `#[doc(hidden)]`).
///
/// This is the fix for the private-field `_additional` bug that blocked users
/// from writing single-expression struct literals with partial field overrides.
#[cfg(feature = "versioned")]
#[test]
fn functional_update_syntax_works() {
    use rubo4e::v202501::Geschaeftspartner;

    // Previously: `error[E0451]: field `_additional` is private`
    // Now: compiles cleanly — the `_additional` field is `pub #[doc(hidden)]`.
    // Use Geschaeftspartner (String-only fields) so test is feature-independent.
    let g = Geschaeftspartner {
        organisationsname: Some("ACME GmbH".to_owned()),
        ..Default::default()
    };
    assert_eq!(g.organisationsname, Some("ACME GmbH".to_owned()));
    assert_eq!(g.nachname, None); // other fields from Default::default()
}

#[cfg(feature = "builder")]
#[test]
fn compile_fail_required_field() {
    // Compile-fail test: a TypedBuilder struct with a required field (no
    // `builder(default)`) must not compile when the field is omitted.
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/builder/ui/required_field_omitted.rs");
    t.compile_fail("tests/builder/ui/bo_typ_setter_unavailable.rs");
}

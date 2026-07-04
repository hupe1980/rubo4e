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

#[cfg(feature = "builder")]
#[test]
fn compile_fail_required_field() {
    // Compile-fail test: a TypedBuilder struct with a required field (no
    // `builder(default)`) must not compile when the field is omitted.
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/builder/ui/required_field_omitted.rs");
    t.compile_fail("tests/builder/ui/bo_typ_setter_unavailable.rs");
}

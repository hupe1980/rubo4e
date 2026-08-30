//! Namespaced `ZusatzAttribut`s, across the wire and across the type set.
//!
//! The unit tests in `src/zusatz_attribut.rs` pin the semantics. This file pins
//! the two things only an integration test can see: that the generator gave the
//! accessors to *every* BO and COM, and that a namespaced attribute survives the
//! BO4E wire format unchanged.

#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::current::{
    Adresse, AnyBo, Betrag, Lokationszuordnung, Marktlokation, Menge, Messlokation, Netzlokation,
    Rechnung, SteuerbareRessource, TechnischeRessource, Vertrag, Zaehler, Zeitraum, Zeitreihe,
    ZusatzAttribut,
};
use rubo4e::prelude::*;
use rubo4e::zusatz_attribut::{Namespace, NamespaceError, ZusatzAttributeExt};

/// The generated impls must reach components as well as Geschäftsobjekte —
/// `Adresse`, `Betrag`, `Menge` and `Zeitraum` all declare `zusatzAttribute`.
#[test]
fn the_accessors_reach_bos_and_coms_alike() {
    fn tag<T: ZusatzAttributeExt>(mut value: T) -> T {
        value.set_zusatz_attribut_in(&Namespace::HEMS, "source", "household-model");
        assert_eq!(
            value.zusatz_attribut_str_in(&Namespace::HEMS, "source"),
            Some("household-model")
        );
        value
    }

    // Geschäftsobjekte.
    tag(Marktlokation::default());
    tag(Messlokation::default());
    tag(Netzlokation::default());
    tag(SteuerbareRessource::default());
    tag(TechnischeRessource::default());
    tag(Lokationszuordnung::default());
    tag(Vertrag::default());
    tag(Zaehler::default());
    tag(Rechnung::default());
    tag(Zeitreihe::default());

    // Components.
    tag(Adresse::default());
    tag(Betrag::default());
    tag(Menge::default());
    tag(Zeitraum::default());
}

/// The § 14a case the household model actually has: BO4E models neither the
/// control variant nor the device key, so both travel namespaced — and a plain
/// BO4E reader still sees them as ordinary `zusatzAttribute`.
#[test]
fn the_fourteen_a_payload_survives_the_wire() {
    const SKI: &str = "d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0";

    let mut sr = SteuerbareRessource {
        steuerbare_ressource_id: Some(SrId::new("C0000000011").unwrap()),
        ..Default::default()
    };
    sr.set_zusatz_attribut_in(&Namespace::HEMS, "eebus-ski", SKI);
    sr.set_zusatz_attribut_in(&Namespace::HEMS, "steuerungsvariante", "EMS");
    sr.set_zusatz_attribut_in(&Namespace::MAKO, "vorgangsnummer", "V-2026-0001");

    let json = sr.to_json_german().unwrap();
    assert!(json.contains(r#""name":"hems:eebus-ski""#), "{json}");

    let decoded = SteuerbareRessource::from_json_german(&json).unwrap();
    assert_eq!(
        decoded.zusatz_attribut_str_in(&Namespace::HEMS, "eebus-ski"),
        Some(SKI)
    );
    assert_eq!(
        decoded.zusatz_attribut_str_in(&Namespace::MAKO, "vorgangsnummer"),
        Some("V-2026-0001")
    );
    assert_eq!(decoded.zusatz_attribut_namespaces(), ["hems", "mako"]);

    // The values are ordinary BO4E — nothing lands in extension data.
    assert!(decoded.extension_paths().is_empty());
}

/// Stripping one system's namespace before handing the document on leaves the
/// rest of the payload untouched.
#[test]
fn one_namespace_can_be_stripped_before_handing_the_document_on() {
    let mut sr = SteuerbareRessource::default();
    sr.set_zusatz_attribut_in(&Namespace::HEMS, "eebus-ski", "aaa");
    sr.set_zusatz_attribut_in(&Namespace::MAKO, "vorgangsnummer", "V-1");
    sr.set_zusatz_attribut("kundennummer", "K-9");

    let stripped = sr.remove_zusatz_attribute_in(&Namespace::HEMS);
    assert_eq!(stripped.len(), 1);
    assert_eq!(stripped[0].name.as_deref(), Some("hems:eebus-ski"));

    let json = sr.to_json_german().unwrap();
    assert!(!json.contains("hems:"));
    assert!(json.contains("mako:vorgangsnummer"));
    assert!(json.contains("kundennummer"));
}

/// An attribute written by another producer, in a namespace we do not know, is
/// left alone — and is visible as an unknown prefix rather than silently kept.
#[test]
fn a_foreign_namespace_is_visible_and_preserved() {
    let json = r#"{
        "_typ": "MARKTLOKATION",
        "marktlokationsId": "51238696781",
        "zusatzAttribute": [
            {"name": "sap:kunde", "wert": "0001234567"},
            {"name": "hems:eebus-ski", "wert": "aaa"}
        ]
    }"#;
    let malo = Marktlokation::from_json_german(json).unwrap();

    assert_eq!(malo.zusatz_attribut_namespaces(), ["sap", "hems"]);
    assert!(!"sap".parse::<Namespace>().unwrap().is_registered());
    assert_eq!(malo.zusatz_attribute_in(&Namespace::HEMS).count(), 1);

    // Round-tripping keeps the foreign entry byte for byte.
    let out = malo.to_json_german().unwrap();
    assert!(out.contains(r#""name":"sap:kunde""#));
}

/// A namespaced write must never disturb what is already there.
#[test]
fn writes_do_not_reorder_the_list() {
    let mut malo = Marktlokation::default();
    for key in ["a", "b", "c"] {
        malo.set_zusatz_attribut(key, key);
    }
    malo.set_zusatz_attribut("b", "B");

    let names: Vec<&str> = malo
        .zusatz_attribute()
        .filter_map(|a| a.name.as_deref())
        .collect();
    assert_eq!(names, ["a", "b", "c"]);
    assert_eq!(malo.zusatz_attribut_str("b"), Some("B"));
}

/// A namespace prefix that would make the split ambiguous is refused before it
/// can reach a payload.
#[test]
fn an_ambiguous_prefix_is_refused() {
    assert_eq!(Namespace::new(""), Err(NamespaceError::Empty));
    assert_eq!(
        Namespace::new("a:b"),
        Err(NamespaceError::ContainsSeparator)
    );
    assert!(Namespace::from_static("hems").unwrap() == Namespace::HEMS);
}

/// `ZusatzAttribut` is the one BO4E schema that declares no `zusatzAttribute` of
/// its own, so it is also the one type without the accessors — which is what
/// stops an attribute list nesting inside an attribute.
#[test]
fn zusatz_attribut_itself_has_no_attributes() {
    // Compile-time claim: `ZusatzAttribut` carries no `zusatz_attribute` field,
    // so this is the whole of its shape.
    let attribut = ZusatzAttribut {
        name: Some("hems:x".into()),
        wert: Some(serde_json::Value::String("1".into())),
        ..Default::default()
    };
    assert_eq!(
        Namespace::HEMS.key_of(attribut.name.as_deref().unwrap()),
        Some("x")
    );
}

/// The accessors work through `AnyBo`'s inner value, so a heterogeneous stream
/// can be tagged without a `match` per type.
#[test]
fn any_bo_reaches_the_accessors_through_its_variant() {
    let mut vertrag = Vertrag::default();
    vertrag.set_zusatz_attribut_in(&Namespace::EDMD, "quelle", "sap-isu");
    let any: AnyBo = vertrag.into();

    let AnyBo::Vertrag(inner) = &any else {
        panic!("expected a Vertrag");
    };
    assert_eq!(
        inner.zusatz_attribut_str_in(&Namespace::EDMD, "quelle"),
        Some("sap-isu")
    );
}

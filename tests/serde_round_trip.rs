//! E04-S20 & E04-S21 — serde round-trip and `_typ`/`_version` meta-field tests.
//!
//! These tests run under `--features versioned,serde` (the `serde` default is on).

#[cfg(feature = "versioned")]
mod serde_tests {
    use rubo4e::v202501::{BoTyp, Vertrag};

    /// E04-S21: A freshly constructed `Vertrag` with `_typ = BoTyp::Vertrag` must
    /// serialize to JSON that contains `"_typ":"VERTRAG"`.
    #[test]
    fn vertrag_typ_field_serializes() {
        let mut v = Vertrag::default();
        v.typ = Some(BoTyp::Vertrag);
        let json = serde_json::to_string(&v).expect("serialization failed");
        assert!(
            json.contains("\"_typ\":\"VERTRAG\""),
            "_typ field not found in JSON output: {json}"
        );
    }

    /// E04-S20: Round-trip: serialize then deserialize → identical value.
    #[test]
    fn vertrag_serde_round_trip() {
        let mut original = Vertrag::default();
        original.typ = Some(BoTyp::Vertrag);
        let json = serde_json::to_string(&original).expect("serialize failed");
        let restored: Vertrag = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(original.typ, restored.typ);
    }

    /// E04-S21: Deserialization with unknown `_typ` value must not fail.
    ///
    /// `BoTyp` has a `#[serde(other)] Unknown` catch-all variant so future schema
    /// releases don't cause panic / deserialization errors in existing binaries.
    #[test]
    fn vertrag_unknown_typ_value_maps_to_unknown_variant() {
        let json = r#"{"_typ":"UNKNOWN_FUTURE_TYPE"}"#;
        let v: Vertrag = serde_json::from_str(json)
            .expect("deserialization must succeed even with unknown _typ");
        assert_eq!(
            v.typ,
            Some(BoTyp::Unknown),
            "unknown _typ value should map to BoTyp::Unknown"
        );
    }

    /// L-03: The `Unknown` variant must serialize as `"UNKNOWN"` (BO4E SCREAMING_SNAKE_CASE),
    /// not as the Rust identifier name `"Unknown"`.
    ///
    /// This matters when code that received an unknown enum value re-serializes it —
    /// the resulting JSON should not break recipients expecting a valid enum string.
    #[test]
    fn unknown_variant_serializes_as_screaming_case() {
        use rubo4e::v202501::Sparte;

        // Deserialize an unknown enum value → Unknown variant.
        let sparte: Sparte = serde_json::from_str(r#""FUTURE_VALUE""#)
            .expect("Unknown catch-all must accept any string");
        assert_eq!(sparte, Sparte::Unknown);

        // Re-serialize → must produce "UNKNOWN", not "Unknown".
        let serialized = serde_json::to_string(&sparte).expect("serialize failed");
        assert_eq!(
            serialized, r#""UNKNOWN""#,
            "Unknown must serialize as SCREAMING_SNAKE_CASE"
        );
    }

    /// L-03: Round-trip of a payload containing an unknown enum value preserves
    /// the `Unknown` variant without panic, and re-serializes as `"UNKNOWN"`.
    #[test]
    fn unknown_enum_round_trip() {
        use rubo4e::v202501::Sparte;

        let sparte: Sparte =
            serde_json::from_str(r#""SOME_FUTURE_SPARTE""#).expect("deserialize failed");
        assert_eq!(sparte, Sparte::Unknown);

        let json = serde_json::to_string(&sparte).expect("serialize failed");
        // The deserialized unknown variant re-serializes as "UNKNOWN" — the library's
        // sentinel value for "we don't know this value".
        assert_eq!(json, r#""UNKNOWN""#);
        // A second deserialization of "UNKNOWN" also gives Unknown.
        let again: Sparte = serde_json::from_str(&json).expect("deserialize from UNKNOWN failed");
        assert_eq!(again, Sparte::Unknown);
    }
}

/// E10-S43 — Unknown-field round-trip tests.
///
/// Verifies that unknown JSON keys survive deserialization → re-serialization
/// intact via the extension-data catch-all (requires `json` feature).
#[cfg(all(feature = "versioned", feature = "json"))]
mod extension_data_tests {
    use rubo4e::{json::Bo4eExtensionData, json::Bo4eJsonExt, v202501::Vertrag};

    /// A single unknown field round-trips through deserialize → re-serialize.
    #[test]
    fn single_unknown_field_survives_round_trip() {
        let json = r#"{"_customAttribute":"test-value","_typ":"VERTRAG"}"#;
        let v: Vertrag = serde_json::from_str(json).expect("deserialize failed");

        assert_eq!(
            v.extension_data()
                .get("_customAttribute")
                .and_then(|x| x.as_str()),
            Some("test-value"),
            "_customAttribute should be captured in extension data"
        );

        let out = v.to_json_german().expect("re-serialize failed");
        assert!(
            out.contains("\"_customAttribute\":\"test-value\""),
            "unknown field must survive re-serialization, got: {out}"
        );
    }

    /// Multiple unknown fields are all preserved, in insertion order.
    #[test]
    fn multiple_unknown_fields_preserve_insertion_order() {
        let json = r#"{"_ext_a":"alpha","_ext_b":"beta","_ext_c":"gamma"}"#;
        let v: Vertrag = serde_json::from_str(json).expect("deserialize failed");

        let keys: Vec<&str> = v.extension_data().keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            ["_ext_a", "_ext_b", "_ext_c"],
            "insertion order must be preserved"
        );

        let out = v.to_json_german().expect("re-serialize failed");
        assert!(out.contains("\"_ext_a\":\"alpha\""), "missing _ext_a");
        assert!(out.contains("\"_ext_b\":\"beta\""), "missing _ext_b");
        assert!(out.contains("\"_ext_c\":\"gamma\""), "missing _ext_c");
    }

    /// A payload with zero unknown fields has empty extension data (not serialized).
    #[test]
    fn no_unknown_fields_gives_empty_additional() {
        let json = r#"{"_typ":"VERTRAG"}"#;
        let v: Vertrag = serde_json::from_str(json).expect("deserialize failed");
        assert!(
            v.extension_data().is_empty(),
            "extension data should be empty for a clean payload"
        );

        let out = v.to_json_german().expect("re-serialize failed");
        assert!(
            !out.contains("_additional"),
            "empty extension data must not appear in output"
        );
    }
}

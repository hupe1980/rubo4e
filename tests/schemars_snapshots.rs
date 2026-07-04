//! Golden-schema snapshot tests for the `schemars` feature.
//!
//! Run with:
//! ```text
//! cargo test --test schemars_snapshots --features schemars,versioned
//! ```
//!
//! To accept new snapshots (e.g. after schema changes):
//! ```text
//! INSTA_UPDATE=unseen cargo test --test schemars_snapshots --features schemars,versioned
//! ```

#[cfg(all(feature = "schemars", feature = "versioned"))]
mod schema_tests {
    use schemars::{schema_for, JsonSchema};

    fn schema_json<T: JsonSchema>() -> String {
        let schema = schema_for!(T);
        serde_json::to_string_pretty(&schema).expect("schema serialization failed")
    }

    /// Ensure every key schema type generates a non-empty JSON object.
    /// Full content is snapshot-tested via insta below.
    #[test]
    fn schemas_are_non_empty() {
        use rubo4e::v202501::{Marktlokation, Rechnung, Sparte, Vertrag};
        for schema in [
            schema_json::<Vertrag>(),
            schema_json::<Marktlokation>(),
            schema_json::<Rechnung>(),
            schema_json::<Sparte>(),
        ] {
            assert!(
                schema.contains("\"type\"")
                    || schema.contains("\"$ref\"")
                    || schema.contains("\"oneOf\""),
                "schema looks empty or malformed: {schema:.200}"
            );
        }
    }

    #[test]
    fn sparte_schema_is_enum_of_strings() {
        use rubo4e::v202501::Sparte;
        let schema = schema_json::<Sparte>();
        // Enum of strings should list known variants
        assert!(
            schema.contains("STROM") || schema.contains("enum"),
            "Sparte schema should contain enum variants: {schema:.400}"
        );
    }

    #[test]
    fn identifier_schema_is_string_type() {
        use rubo4e::identifiers::MaloId;
        let schema = schema_json::<MaloId>();
        // MaloId is wrapped with schemars(with = "String") → should be a string schema
        assert!(
            schema.contains("\"string\""),
            "MaloId schema should be type string: {schema}"
        );
    }

    #[test]
    fn snapshot_vertrag() {
        use rubo4e::v202501::Vertrag;
        insta::assert_snapshot!("vertrag_schema", schema_json::<Vertrag>());
    }

    #[test]
    fn snapshot_marktlokation() {
        use rubo4e::v202501::Marktlokation;
        insta::assert_snapshot!("marktlokation_schema", schema_json::<Marktlokation>());
    }

    #[test]
    fn snapshot_sparte() {
        use rubo4e::v202501::Sparte;
        insta::assert_snapshot!("sparte_schema", schema_json::<Sparte>());
    }
}

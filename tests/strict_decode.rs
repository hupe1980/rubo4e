//! Integration tests for the recursive strict-decode walker (`Bo4eStrict`):
//! `collect_unknown_enums`, `unknown_enum_paths`, and `ensure_known_enums`.
//!
//! This is the real answer to feedback BUG-3: the lenient `serde` path maps any
//! unrecognized enum wire value to `Unknown`, so a single `ensure_known_enums()`
//! call at the ingest boundary replaces per-field `== T::Unknown` checks — and
//! finds them anywhere in a nested payload, with JSON-paths.
//!
//! Run with:
//! ```text
//! cargo test --test strict_decode --features versioned,json,decimal
//! ```

#![cfg(feature = "versioned")]

use rubo4e::current::{Mengeneinheit, Zaehler, Zaehlertyp, Zaehlwerk};
use rubo4e::Bo4eStrict;

#[test]
fn clean_value_has_no_unknown_enums() {
    let z = Zaehler {
        zaehlertyp: Some(Zaehlertyp::Wasserzaehler),
        zaehlwerke: Some(vec![Zaehlwerk {
            einheit: Some(Mengeneinheit::Kwh),
            ..Default::default()
        }]),
        ..Default::default()
    };
    assert_eq!(z.unknown_enum_paths(), Vec::<String>::new());
    assert!(z.ensure_known_enums().is_ok());
}

#[test]
fn finds_top_level_and_nested_unknown_enums_with_paths() {
    let z = Zaehler {
        zaehlertyp: Some(Zaehlertyp::Unknown),
        zaehlwerke: Some(vec![
            Zaehlwerk {
                einheit: Some(Mengeneinheit::Kwh), // known
                ..Default::default()
            },
            Zaehlwerk {
                einheit: Some(Mengeneinheit::Unknown), // out-of-schema, at index 1
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    let paths = z.unknown_enum_paths();
    assert!(
        paths.contains(&"zaehlertyp".to_string()),
        "expected top-level path, got {paths:?}"
    );
    assert!(
        paths.contains(&"zaehlwerke[1].einheit".to_string()),
        "expected nested array path with correct index, got {paths:?}"
    );
    // Index 0 was in-schema and must NOT be reported.
    assert!(!paths.iter().any(|p| p.contains("[0]")), "got {paths:?}");
    assert_eq!(
        paths.len(),
        2,
        "exactly two unknowns expected, got {paths:?}"
    );

    let err = z.ensure_known_enums().unwrap_err();
    assert_eq!(err.paths.len(), 2);
    // Error message is human-readable and lists the paths.
    let msg = err.to_string();
    assert!(msg.contains("zaehlertyp"), "{msg}");
    assert!(msg.contains("zaehlwerke[1].einheit"), "{msg}");
}

#[test]
fn none_valued_enum_fields_are_not_reported() {
    // A `None` enum field is absent, not `Unknown` — it must not be flagged.
    let z = Zaehler {
        zaehlertyp: None,
        ..Default::default()
    };
    assert!(z.ensure_known_enums().is_ok());
}

// ─── The real ingest-boundary flow: lenient decode → strict check ──────────

#[cfg(feature = "json")]
#[test]
fn lenient_json_decode_then_strict_reject() {
    // A payload with a typo'd enum value: serde maps it to Unknown (no error)...
    let json = serde_json::json!({
        "_typ": "ZAEHLER",
        "zaehlertyp": "WASSERZAEHLER",
        "zaehlwerke": [
            { "_typ": "ZAEHLWERK", "einheit": "KWH" },
            { "_typ": "ZAEHLWERK", "einheit": "NOT_A_REAL_UNIT" }
        ]
    });
    let z: Zaehler = serde_json::from_value(json).expect("lenient decode succeeds");

    // ...and the strict check rejects it, pinpointing the offending path.
    let err = z.ensure_known_enums().unwrap_err();
    assert_eq!(err.paths, vec!["zaehlwerke[1].einheit".to_string()]);
}

// ─── AnyBo delegates into the inner BO, and flags an unknown discriminant ──

#[cfg(feature = "json")]
#[test]
fn any_bo_strict_delegates_and_flags_unknown_typ() {
    use rubo4e::current::AnyBo;

    // Known BO with a nested unknown enum → path reported without a wrapper prefix.
    let z = Zaehler {
        zaehlertyp: Some(Zaehlertyp::Unknown),
        ..Default::default()
    };
    let any: AnyBo = z.into();
    assert_eq!(any.unknown_enum_paths(), vec!["zaehlertyp".to_string()]);

    // Unknown `_typ` discriminant is itself out-of-schema.
    let unknown: AnyBo = serde_json::from_value(serde_json::json!({
        "_typ": "SOME_FUTURE_BO",
        "foo": 1
    }))
    .unwrap();
    assert_eq!(unknown.unknown_enum_paths(), vec!["_typ".to_string()]);
}

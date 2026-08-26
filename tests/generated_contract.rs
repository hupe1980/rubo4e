//! Drift guards between the committed schema snapshots and the committed
//! generated code.
//!
//! `src/generated/` is checked in, so nothing at build time forces it to agree
//! with `generator/schemas/`. These tests read both sides and compare.
//!
//! They read the **generated source text** rather than the types, because
//! asserting a property over all ~96 structs is not expressible generically in
//! Rust without a type-level registry that would need the same guard itself.

#![cfg(feature = "versioned")]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The module directory the `current` series emits into.
const SERIES: &str = "v202607";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The committed schema snapshot for [`SERIES`], found rather than hard-coded.
///
/// BO4E ships patch releases inside a series (`v202607.0.0` → `v202607.1.0`),
/// and a tag spelled out in a `const` here turns every one of them into a
/// scavenger hunt through the test suite. Exactly one snapshot per series is
/// committed, so the directory name *is* the pin.
fn schema_dir() -> PathBuf {
    let root = workspace_root().join("generator/schemas");
    let mut matches: Vec<PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("reading {}: {e}", root.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with(SERIES))
        })
        .collect();
    matches.sort();
    match matches.len() {
        1 => matches.pop().expect("checked len"),
        0 => panic!(
            "no committed schema snapshot for series {SERIES} under {}",
            root.display()
        ),
        n => panic!(
            "{n} schema snapshots for series {SERIES} under {} — commit exactly one: {matches:?}",
            root.display()
        ),
    }
}

/// The full release tag of the committed snapshot, e.g. `"v202607.1.0"`.
fn schema_version() -> String {
    schema_dir()
        .file_name()
        .and_then(|n| n.to_str())
        .expect("schema dir name is UTF-8")
        .to_owned()
}

fn generated_dir() -> PathBuf {
    workspace_root().join("src/generated").join(SERIES)
}

/// Every `*.json` directly inside `dir`, sorted, as `(title, parsed)`.
fn read_schemas(dir: &Path) -> Vec<(String, serde_json::Value)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            let raw = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()));
            let doc: serde_json::Value = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", p.display()));
            let title = doc["title"]
                .as_str()
                .unwrap_or_else(|| panic!("{} has no title", p.display()))
                .to_owned();
            (title, doc)
        })
        .collect()
}

/// Whitespace-collapsed generated source, so assertions do not depend on where
/// rustfmt chose to wrap a line.
fn generated_source_flat(title: &str) -> String {
    generated_source(title).split_whitespace().collect()
}

/// The generated Rust source for the type named `title`.
fn generated_source(title: &str) -> String {
    let file = generated_dir().join(format!("{}.rs", heck::AsSnakeCase(title)));
    std::fs::read_to_string(&file).unwrap_or_else(|e| {
        panic!(
            "no generated file for schema {title:?} at {} ({e}) — run `just generate`",
            file.display()
        )
    })
}

/// Reads the wire literal a metadata property pins, the same way the generator
/// does: `const` first, then `default`.
fn metadata_literal(doc: &serde_json::Value, key: &str) -> Option<String> {
    let prop = doc.get("properties")?.get(key)?;
    prop.get("const")
        .or_else(|| prop.get("default"))
        .and_then(|v| v.as_str())
        .map(str::to_owned)
}

// ─── Coverage ────────────────────────────────────────────────────────────────

/// Every schema in the release must have emitted exactly one Rust module, and
/// nothing else may sit in the generated directory.
#[test]
fn every_schema_has_generated_code_and_vice_versa() {
    let mut expected: Vec<String> = Vec::new();
    for category in ["bo", "com", "enum"] {
        for (title, _) in read_schemas(&schema_dir().join(category)) {
            expected.push(format!("{}.rs", heck::AsSnakeCase(&title)));
        }
    }
    for (title, _) in read_schemas(&schema_dir()) {
        expected.push(format!("{}.rs", heck::AsSnakeCase(&title)));
    }
    expected.sort();

    // Catches a truncated or empty snapshot; the exact set is checked by the
    // missing/stale comparison below, so the floor only has to be far enough
    // under a real release (189 types in v202607.1.0) to survive one that
    // retires a handful of types.
    assert!(
        expected.len() >= 150,
        "expected the full BO4E schema set, found only {} files under {} — \
         run `just download-schemas {}`",
        expected.len(),
        schema_dir().display(),
        schema_version(),
    );

    let mut actual: Vec<String> = std::fs::read_dir(generated_dir())
        .expect("src/generated/v202607 must exist — run `just generate`")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".rs") && n != "mod.rs")
        .collect();
    actual.sort();

    let missing: Vec<&String> = expected.iter().filter(|f| !actual.contains(f)).collect();
    let extra: Vec<&String> = actual.iter().filter(|f| !expected.contains(f)).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "generated code is out of sync with the schemas; run `just generate`\n\
         missing: {missing:?}\nstale: {extra:?}"
    );
}

// ─── BO4E metadata drift guards ──────────────────────────────────────────────

/// `_version` must be the value the schema declares — **not** the release tag.
///
/// BO4E tags releases `v202607.0.0` but the `_version` inside a payload is
/// `202607.0.0`. The guard reads the schema rather than a hard-coded string, so
/// it stays true across releases.
#[test]
fn every_struct_stamps_the_schema_declared_version() {
    let mut checked = 0usize;
    for category in ["bo", "com", ""] {
        for (title, doc) in read_schemas(&schema_dir().join(category)) {
            let Some(declared) = metadata_literal(&doc, "_version") else {
                continue;
            };
            assert!(
                !declared.starts_with('v'),
                "{title}: schema declares _version {declared:?} with a `v` prefix — \
                 re-check the assumption this guard rests on"
            );
            // Structs with a required field get no `Default` (a required field's
            // type need not implement it), so they stamp the metadata through the
            // builder instead.  Either route has to produce the schema's value.
            let src = generated_source_flat(&title);
            let via_default = format!("version:Some({declared:?}.to_owned()),");
            let via_builder =
                format!("builder(default=Some({declared:?}.to_owned()),setter(into))");
            assert!(
                src.contains(&via_default) || src.contains(&via_builder),
                "{title}: neither the generated Default nor the builder stamps \
                 `_version` as {declared:?}; run `just generate`"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 90,
        "expected ~93 versioned schemas, saw {checked}"
    );
}

/// Every struct must stamp the exact `_typ` discriminant its schema pins —
/// COMs included.
///
/// Every BO4E COM schema declares `_typ` as a `const`, so pydantic stamps it on
/// every component the reference implementation emits. rubo4e left COM `_typ`
/// unset, which made a Rust-built `Adresse` distinguishable from one produced by
/// any other implementation.
#[test]
fn every_struct_stamps_the_schema_declared_typ() {
    let mut checked = 0usize;
    for (category, typ_enum) in [("bo", "BoTyp"), ("com", "ComTyp"), ("", "ComTyp")] {
        for (title, doc) in read_schemas(&schema_dir().join(category)) {
            let Some(declared) = metadata_literal(&doc, "_typ") else {
                continue;
            };
            let src = generated_source_flat(&title);
            // The variant name is resolved from the struct the discriminant
            // names, so it is the type's own Rust name.
            let via_default = format!("typ:Some({typ_enum}::{title}),");
            let via_builder = format!("builder(default=Some({typ_enum}::{title}),setter(skip))");
            assert!(
                src.contains(&via_default) || src.contains(&via_builder),
                "{title}: neither the generated Default nor the builder stamps \
                 `_typ` for wire value {declared:?}; run `just generate`"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 90,
        "expected ~95 schemas with a _typ constant, saw {checked}"
    );
}

/// Every BO's `Bo4eObject` constants must agree with the schema, and with each
/// other.
///
/// `TYP_WIRE` is emitted as a literal rather than derived from
/// `BO_TYP.as_wire()`, because `Bo4eEnum::as_wire` is a trait method and cannot
/// run in a const initializer. That leaves two spellings of one fact, which is
/// exactly the shape that drifts — so this pins them together, and both against
/// the schema.
#[test]
fn bo_constants_agree_with_the_schema_and_each_other() {
    let mut checked = 0usize;
    for (title, doc) in read_schemas(&schema_dir().join("bo")) {
        let Some(declared) = metadata_literal(&doc, "_typ") else {
            continue;
        };
        let src = generated_source_flat(&title);
        assert!(
            src.contains(&format!("constTYP_WIRE:&'staticstr=\"{declared}\";")),
            "{title}: TYP_WIRE does not carry the schema's `_typ` const {declared:?}; \
             run `just generate`"
        );
        assert!(
            src.contains(&format!("constBO_TYP:BoTyp=BoTyp::{title};")),
            "{title}: BO_TYP is not the variant named after the struct; run `just generate`"
        );
        checked += 1;
    }
    assert!(checked >= 30, "expected ~35 BO schemas, saw {checked}");
}

/// …and the same fact read through the API, for the types the test can name.
///
/// The source-level guard above covers all ~35; this one proves the two
/// spellings really are the same value at runtime, which a string comparison
/// over generated text cannot.
#[test]
fn typ_wire_is_the_discriminants_own_wire_string() {
    use rubo4e::current::{
        Bilanzierung, Lastgang, Marktlokation, Messlokation, Netzlokation, Rechnung, Tarif, Vertrag,
    };
    use rubo4e::{Bo4eEnum as _, Bo4eObject};

    fn check<T: Bo4eObject>(expected: &str) {
        assert_eq!(T::TYP_WIRE, T::BO_TYP.as_wire(), "TYP_WIRE vs BO_TYP");
        assert_eq!(T::TYP_WIRE, expected);
        assert_eq!(
            T::SCHEMA_SERIES,
            T::SCHEMA_VERSION.split('.').next().expect("a version"),
            "SCHEMA_SERIES must be SCHEMA_VERSION's YYYYMM prefix"
        );
        assert!(
            !T::SCHEMA_VERSION.starts_with('v'),
            "SCHEMA_VERSION is the wire spelling, which has no `v`"
        );
    }

    check::<Marktlokation>("MARKTLOKATION");
    check::<Messlokation>("MESSLOKATION");
    check::<Netzlokation>("NETZLOKATION");
    check::<Vertrag>("VERTRAG");
    check::<Rechnung>("RECHNUNG");
    check::<Bilanzierung>("BILANZIERUNG");
    // The two with required fields, which a `T: Default` bound would exclude.
    check::<Lastgang>("LASTGANG");
    check::<Tarif>("TARIF");
}

/// A struct the schema marks `required` gets a `new(...)` in place of the
/// `Default` the derive cannot produce.
///
/// Without it, `Lastgang` and `Tarif` are the only generated types that cannot
/// be constructed without the `builder` feature or writing out every optional
/// field by hand.
#[test]
fn structs_with_required_fields_get_a_constructor() {
    let mut checked = 0usize;
    for category in ["bo", "com"] {
        for (title, doc) in read_schemas(&schema_dir().join(category)) {
            let required: Vec<&str> = doc
                .get("required")
                .and_then(|r| r.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();
            let src = generated_source_flat(&title);
            if required.is_empty() {
                assert!(
                    src.contains(&format!("implDefaultfor{title}")),
                    "{title} has no required field, so it must derive or emit `Default`"
                );
                continue;
            }
            assert!(
                !src.contains(&format!("implDefaultfor{title}")),
                "{title} has required fields {required:?}, so it cannot have a `Default`"
            );
            assert!(
                src.contains("pubfnnew("),
                "{title} has required fields {required:?} and no `Default`, so it needs \
                 a `new(...)`; run `just generate`"
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked, 2,
        "v202607 declares `required` on exactly Lastgang and Tarif — if that changed, \
         re-read the schemas before updating this number"
    );
}

/// The `BoTyp` / `ComTyp` variant that a wire value maps to must be named after
/// the struct it discriminates.
///
/// Deriving it mechanically from the SCREAMING value instead loses the word
/// boundaries the schema set knows, and produced `BoTyp::Technischeressource`
/// for `TechnischeRessource` and `ComTyp::Aufabschlag` for `AufAbschlag`.
#[test]
fn discriminant_variants_are_named_after_their_structs() {
    for (category, enum_file) in [("bo", "bo_typ.rs"), ("com", "com_typ.rs")] {
        let src = std::fs::read_to_string(generated_dir().join(enum_file))
            .unwrap_or_else(|e| panic!("cannot read {enum_file}: {e}"));
        for (title, doc) in read_schemas(&schema_dir().join(category)) {
            let Some(wire) = metadata_literal(&doc, "_typ") else {
                continue;
            };
            let expected = format!("    {title},\n");
            assert!(
                src.contains(&expected),
                "{enum_file}: wire value {wire:?} must map to the variant {title:?}, \
                 named after the struct it discriminates"
            );
        }
    }
}

// ─── Enum variant naming ─────────────────────────────────────────────────────

/// Two BO4E wire values must never collapse onto one Rust variant.
///
/// `MESSPREIS_G2_5` (gas meter size G 2.5) and `MESSPREIS_G25` (G 25) both
/// rendered to `MesspreisG25` under a plain camel-case conversion, so half the
/// call sites that picked that variant meant the other meter size.
#[test]
fn enum_variants_are_injective_over_wire_values() {
    for (title, doc) in read_schemas(&schema_dir().join("enum")) {
        let Some(values) = doc["enum"].as_array() else {
            continue;
        };
        // `as_wire` is the authoritative variant ↔ value table in the output.
        // Read it from the whitespace-collapsed source so a long arm rustfmt
        // wrapped in braces parses the same as a short one.
        let src = generated_source_flat(&title);
        let mut by_variant: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for chunk in src.split("Self::").skip(1) {
            let Some((variant, tail)) = chunk.split_once("=>") else {
                continue;
            };
            let tail = tail.trim_start_matches('{');
            let Some(rest) = tail.strip_prefix('"') else {
                continue; // not an `as_wire` arm
            };
            let Some((wire, _)) = rest.split_once('"') else {
                continue;
            };
            by_variant.entry(variant).or_default().push(wire);
        }
        for (variant, wires) in &by_variant {
            assert_eq!(
                wires.len(),
                1,
                "{title}::{variant} is claimed by {wires:?} — two distinct BO4E \
                 values collapsed onto one Rust variant"
            );
        }
        let mapped = by_variant.len();
        // Every schema value plus the synthetic `Unknown` catch-all.
        assert_eq!(
            mapped,
            values.len() + 1,
            "{title}: {mapped} variants mapped but the schema declares {} values",
            values.len(),
        );
    }
}

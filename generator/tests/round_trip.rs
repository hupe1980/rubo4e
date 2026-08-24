//! Integration tests for the bo4e-generator pipeline.
//!
//! These tests parse the committed schema snapshots and verify that the emitter
//! produces the expected Rust source code.  When a generator change would alter
//! the output, the snapshot files in `tests/snapshots/` must be updated to
//! match — making diffs explicit and reviewable.

use std::collections::BTreeSet;
use std::path::Path;

use bo4e_generator::{emitter, parser};

/// Workspace root, resolved relative to the manifest directory.
fn workspace_root() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR is the generator/ subdirectory.
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().unwrap().to_owned()
}

/// The committed schema snapshot for the `v202607` series, found rather than
/// hard-coded.
///
/// BO4E ships patch releases inside a series (`v202607.0.0` → `v202607.1.0`);
/// spelling the tag out in a `const` turns each one into a scavenger hunt
/// through the test suite. Exactly one snapshot per series is committed, so the
/// directory name *is* the pin.
fn pinned_tag() -> String {
    let root = workspace_root().join("generator/schemas");
    let mut matches: Vec<String> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("reading {}: {e}", root.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
        .filter(|n| n.starts_with("v202607"))
        .collect();
    matches.sort();
    assert_eq!(
        matches.len(),
        1,
        "expected exactly one committed v202607 schema snapshot, found {matches:?}"
    );
    matches.pop().expect("checked len")
}

/// The committed snapshot directory for the `v202607` series.
fn pinned_schema_root() -> std::path::PathBuf {
    workspace_root()
        .join("generator/schemas")
        .join(pinned_tag())
}

/// Loads the snapshot file at `tests/snapshots/<name>` and returns its content.
fn load_snapshot(name: &str) -> String {
    std::fs::read_to_string(snapshot_path(name))
        .unwrap_or_else(|e| panic!("cannot read snapshot {name}: {e}"))
}

fn snapshot_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots")
        .join(name)
}

/// Compares emitter output against a committed snapshot.
///
/// Set `UPDATE_SNAPSHOTS=1` to rewrite the snapshot instead of failing, then
/// review the diff — the point of these files is that emitter changes show up as
/// a reviewable diff, so the rewrite is deliberately opt-in and still fails the
/// run so it can never pass silently in CI.
fn assert_snapshot(name: &str, generated: &str) {
    let expected = load_snapshot(name);
    if generated == expected {
        return;
    }
    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(snapshot_path(name), generated)
            .unwrap_or_else(|e| panic!("cannot write snapshot {name}: {e}"));
        panic!("snapshot {name} rewritten (UPDATE_SNAPSHOTS set) — review the diff and re-run");
    }
    eprintln!("=== EXPECTED ===\n{expected}\n=== GENERATED ===\n{generated}");
    panic!(
        "Generator output for {name} changed!\n\
         If this is intentional, re-run with UPDATE_SNAPSHOTS=1 and review the diff."
    );
}

/// Parses `<schema_dir>/<category>/<file>.json` and emits Rust source.
fn generate_one(schema_version: &str, category: &str, file_stem: &str) -> String {
    let root = workspace_root();
    let schema_dir = root
        .join("generator/schemas")
        .join(schema_version)
        .join(category);
    let path = schema_dir.join(format!("{file_stem}.json"));

    // parse_dir on a temp dir with just the one file, or use a direct parse path.
    // We use parse_dir which reads all JSON files in the directory, then find our type.
    let nodes = parser::parse_dir(&schema_dir).unwrap_or_else(|e| panic!("parse_dir failed: {e}"));

    let target_name = file_stem.to_owned();
    let node = nodes
        .iter()
        .find(|n| {
            // match by file stem case-insensitively
            heck::AsSnakeCase(n.name()).to_string()
                == heck::AsSnakeCase(target_name.as_str()).to_string()
        })
        .unwrap_or_else(|| {
            panic!(
                "type '{}' not found in {} — available: {:?}",
                file_stem,
                path.display(),
                nodes.iter().map(|n| n.name()).collect::<Vec<_>>()
            )
        });

    let names = emitter::DiscriminantNames::from_nodes(&nodes);
    let (_filename, source) = emitter::emit_node(node, schema_version, &names)
        .unwrap_or_else(|e| panic!("emit_node failed: {e}"));
    source
}

// ─── Snapshot tests ────────────────────────────────────────────────────────

/// `Menge` is a small, stable COM type — a good canary for emitter changes.
#[test]
fn v202607_menge_snapshot() {
    let generated = generate_one(&pinned_tag(), "com", "Menge");
    assert_snapshot("v202607_menge.rs", &generated);
}

/// Smoke-test: every schema in v202607 must parse without error.
#[test]
fn v202607_all_schemas_parse() {
    let schema_root = pinned_schema_root();
    let mut total = 0usize;

    for category in ["bo", "com", "enum"] {
        let dir = schema_root.join(category);
        if dir.exists() {
            let nodes = parser::parse_dir(&dir)
                .unwrap_or_else(|e| panic!("parse_dir({category}) failed: {e}"));
            total += nodes.len();
        }
    }

    // ZusatzAttribut.json at root
    let root_nodes = parser::parse_file_as_com(&schema_root.join("ZusatzAttribut.json"))
        .expect("failed to parse ZusatzAttribut.json");
    if root_nodes.is_some() {
        total += 1;
    }

    // A floor that catches a truncated or empty snapshot, not an exact count:
    // BO4E retires types (v202607.1.0 dropped two enums, taking the set from 191
    // to 189). `tests/generated_contract.rs` pins the exact set against the
    // committed codegen.
    assert!(
        total >= 150,
        "expected the full BO4E schema set, got only {total} nodes"
    );
}

/// Smoke-test: every parsed schema node must emit valid Rust source (no panics,
/// no emitter errors) for v202607.
#[test]
fn v202607_all_schemas_emit() {
    let schema_root = pinned_schema_root();
    let mut nodes = Vec::new();

    for category in ["bo", "com", "enum"] {
        let dir = schema_root.join(category);
        if dir.exists() {
            nodes.extend(
                parser::parse_dir(&dir)
                    .unwrap_or_else(|e| panic!("parse_dir({category}) failed: {e}")),
            );
        }
    }

    if let Some(n) = parser::parse_file_as_com(&schema_root.join("ZusatzAttribut.json"))
        .expect("failed to parse ZusatzAttribut.json")
    {
        nodes.push(n);
    }

    let names = emitter::DiscriminantNames::from_nodes(&nodes);
    let errors: Vec<String> = nodes
        .iter()
        .filter_map(|n| {
            emitter::emit_node(n, &pinned_tag(), &names)
                .err()
                .map(|e| format!("{}: {e}", n.name()))
        })
        .collect();

    assert!(errors.is_empty(), "emitter errors:\n{}", errors.join("\n"));
}

// ─── AST shape tests ───────────────────────────────────────────────────────

/// Verifies that `Vertrag.json` from the pinned snapshot parses into the expected AST shape.
///
/// This test guards against parser regressions: if a field is silently dropped or its
/// type inference changes, the assertion will catch it.
#[test]
fn v202607_vertrag_ast_shape() {
    use bo4e_generator::ast::{FieldType, PrimitiveType};

    let bo_dir = pinned_schema_root().join("bo");
    let nodes = parser::parse_dir(&bo_dir).expect("parse bo dir");

    let vertrag = nodes
        .iter()
        .find(|n| n.name() == "Vertrag")
        .expect("Vertrag not found in BO schema dir");

    // Must parse as a BO struct node.
    let bo = vertrag.as_struct().expect("expected a struct node");
    assert!(bo.kind.is_bo(), "Vertrag must parse as a BO, not a COM");
    assert_eq!(bo.typ_const.as_deref(), Some("VERTRAG"));
    // The wire spelling of the pinned tag: the tag without its `v`.
    assert_eq!(
        bo.version_default.as_deref(),
        Some(pinned_tag().trim_start_matches('v')),
    );

    // Check that the expected fields are present.
    let field_names: Vec<&str> = bo.fields.iter().map(|f| f.name.as_str()).collect();
    for expected in &[
        "_id",
        "_typ",
        "_version",
        "beschreibung",
        "sparte",
        "vertragsbeginn",
        "vertragsende",
        "vertragsnummer",
        "vertragsstatus",
    ] {
        assert!(
            field_names.contains(expected),
            "field '{}' missing from Vertrag AST; present: {:?}",
            expected,
            field_names
        );
    }

    // `sparte` should map to a BO enum, not a raw String.
    let sparte = bo.fields.iter().find(|f| f.name == "sparte").unwrap();
    assert!(
        matches!(&sparte.field_type, FieldType::BoEnum(_)),
        "expected sparte to be BoEnum, got {:?}",
        sparte.field_type
    );
    assert!(sparte.is_optional, "sparte should be optional");

    // `beschreibung` should be a String primitive.
    let beschreibung = bo.fields.iter().find(|f| f.name == "beschreibung").unwrap();
    assert!(
        matches!(
            &beschreibung.field_type,
            FieldType::Primitive(PrimitiveType::String)
        ),
        "expected beschreibung to be String, got {:?}",
        beschreibung.field_type
    );

    // `vertragsbeginn` should be an OffsetDateTime (timestamp inference).
    let beginn = bo
        .fields
        .iter()
        .find(|f| f.name == "vertragsbeginn")
        .unwrap();
    assert!(
        matches!(
            &beginn.field_type,
            FieldType::Primitive(PrimitiveType::OffsetDateTime)
        ),
        "expected vertragsbeginn to be OffsetDateTime, got {:?}",
        beginn.field_type
    );
}

// ─── Inference audit ───────────────────────────────────────────────────────

/// Every entry in the inference table must name a field that (a) exists and
/// (b) the schema declares as a plain, unannotated `"string"`.
///
/// A dead entry is untested speculation that will fire the day some unrelated
/// schema grows a field with the same name. An entry pointing at a field the
/// schema types some other way is a Rust type narrower than the wire format,
/// which cannot read what other BO4E implementations emit.
#[test]
fn v202607_inference_entries_are_live_and_string_typed() {
    use bo4e_generator::inference;

    let root = pinned_schema_root();
    let mut schemas: Vec<serde_json::Value> = Vec::new();
    for category in ["bo", "com"] {
        let dir = root.join(category);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.filter_map(Result::ok) {
            if e.path().extension().is_some_and(|x| x == "json") {
                let raw = std::fs::read_to_string(e.path()).expect("readable schema");
                schemas.push(serde_json::from_str(&raw).expect("valid schema JSON"));
            }
        }
    }
    assert!(schemas.len() >= 95, "expected the full BO/COM set");

    let mut problems: Vec<String> = Vec::new();
    for (struct_name, field) in inference::typed_fields() {
        let Some(schema) = schemas
            .iter()
            .find(|s| s["title"].as_str() == Some(struct_name))
        else {
            problems.push(format!("{struct_name}: no such schema"));
            continue;
        };
        let Some(prop) = schema["properties"].get(field) else {
            problems.push(format!("{struct_name}.{field}: no such property"));
            continue;
        };
        // Unwrap the `anyOf [T, null]` BO4E wraps every optional property in.
        let inner = prop["anyOf"]
            .as_array()
            .and_then(|a| a.iter().find(|v| v["type"].as_str() != Some("null")))
            .unwrap_or(prop);
        let is_plain_string =
            inner["type"].as_str() == Some("string") && inner.get("format").is_none();
        if !is_plain_string {
            problems.push(format!(
                "{struct_name}.{field}: schema declares {}, which is authoritative — \
                 the inference table must not override it",
                serde_json::to_string(inner).unwrap_or_default()
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "inference table is out of step with the schemas:\n  {}",
        problems.join("\n  ")
    );
}

/// Fields the inference table leaves alone, because their schema description
/// says they hold something other than what a same-named field elsewhere holds.
#[test]
fn v202607_homonyms_and_near_misses_stay_untyped() {
    use bo4e_generator::inference;

    for (struct_name, field) in [
        // "Der Name des Marktgebietes" / "Der Name der Regelzone" — not codes.
        ("MarktgebietInfo", "marktgebiet"),
        ("StandorteigenschaftenStrom", "regelzone"),
        // "Die Nummer oder E-Mail-Adresse" — not a number.
        ("Kontaktweg", "kontaktwert"),
    ] {
        assert_eq!(
            inference::infer_with_parent(Some(struct_name), field),
            None,
            "{struct_name}.{field} must keep the type its schema declares"
        );
    }
}

/// Pins the resolved type of high-value fields against the real schemas.
///
/// The guards above cover the inference table itself; this covers the cases
/// where the **schema** is what decides — a `$ref` or a `"format"` beating a
/// name that looks like something else.
#[test]
fn v202607_resolved_field_types() {
    use bo4e_generator::ast::{FieldType, PrimitiveType, SchemaNode};

    let root = pinned_schema_root();
    let mut nodes: Vec<SchemaNode> = Vec::new();
    for category in ["bo", "com"] {
        let dir = root.join(category);
        if dir.exists() {
            nodes.extend(
                parser::parse_dir(&dir)
                    .unwrap_or_else(|e| panic!("parse_dir({category}) failed: {e}")),
            );
        }
    }
    if let Some(n) = parser::parse_file_as_com(&root.join("ZusatzAttribut.json"))
        .expect("parse ZusatzAttribut.json")
    {
        nodes.push(n);
    }

    let ident = |n: &str| FieldType::Identifier(n.into());
    let com = |n: &str| FieldType::Com(n.into());
    const DEC: FieldType = FieldType::Primitive(PrimitiveType::Decimal);
    const DT: FieldType = FieldType::Primitive(PrimitiveType::OffsetDateTime);
    const DATE: FieldType = FieldType::Primitive(PrimitiveType::Date);
    const STR: FieldType = FieldType::Primitive(PrimitiveType::String);

    let cases: &[(&str, &str, FieldType)] = &[
        // ── Identifier newtypes, from the inference table ───────────────────
        ("Marktlokation", "marktlokationsId", ident("MaloId")),
        ("Messlokation", "messlokationsId", ident("MeloId")),
        ("Netzlokation", "netzlokationsId", ident("NeloId")),
        (
            "SteuerbareRessource",
            "steuerbareRessourceId",
            ident("SrId"),
        ),
        (
            "TechnischeRessource",
            "technischeRessourceId",
            ident("TrId"),
        ),
        ("Bilanzierung", "bilanzkreis", ident("EicCode")),
        ("Marktlokation", "regelzone", ident("EicCode")),
        ("Zaehlwerk", "obisKennzahl", ident("ObisCode")),
        (
            "Marktteilnehmer",
            "rollencodenummer",
            ident("MarktpartnerId"),
        ),
        // ── Name / code pairs ───────────────────────────────────────────────
        //
        // The schema separates the two; only the code half carries a format.
        ("MarktgebietInfo", "marktgebiet", STR), // "Der Name des Marktgebietes"
        ("StandorteigenschaftenStrom", "regelzone", STR), // "Der Name der Regelzone"
        (
            "StandorteigenschaftenStrom",
            "regelzoneEic",
            ident("EicCode"),
        ),
        (
            "StandorteigenschaftenStrom",
            "bilanzierungsgebietEic",
            ident("BilanzierungsgebietId"),
        ),
        ("Fremdkostenposition", "gebietcodeEic", ident("EicCode")),
        ("Kontaktweg", "kontaktwert", STR),
        // Every OBIS-bearing field, including the one upstream spells with a
        // lower-case `k`.
        ("Netzlokation", "obiskennzahl", ident("ObisCode")),
        // Untyped on purpose: the schema names no format for either.
        ("MarktgebietInfo", "marktgebietcode", STR),
        ("Fremdkostenposition", "marktpartnercode", STR),
        // ── `"format"` wins over any name ───────────────────────────────────
        ("Vertrag", "vertragsbeginn", DT),
        ("Vertrag", "vertragsende", DT),
        ("Rechnung", "rechnungsdatum", DT),
        ("Rechnung", "faelligkeitsdatum", DT),
        ("Zeitraum", "startdatum", DATE),
        ("Zeitraum", "enddatum", DATE),
        // ── `"type": "number"` needs no inference ───────────────────────────
        ("Betrag", "wert", DEC),
        ("Preis", "wert", DEC),
        ("Menge", "wert", DEC),
        ("Steuerbetrag", "steuerwert", DEC),
        // ── A `$ref` wins over a name that suggests a scalar ────────────────
        ("Rechnungsposition", "einzelpreis", com("Preis")),
        ("Kostenposition", "einzelpreis", com("Preis")),
        ("Kostenposition", "menge", com("Menge")),
        ("Fremdkostenposition", "menge", com("Menge")),
        ("Bilanzierung", "kundenwert", com("Menge")),
        ("Angebotsposition", "positionspreis", com("Preis")),
        // ── An untyped property stays free-form ─────────────────────────────
        ("ZusatzAttribut", "wert", FieldType::JsonValue),
    ];

    let mut failures: Vec<String> = Vec::new();
    for (struct_name, field_name, expected) in cases {
        let Some(node) = nodes.iter().find(|n| n.name() == *struct_name) else {
            failures.push(format!("{struct_name}: no such schema"));
            continue;
        };
        let Some(field) = node
            .as_struct()
            .and_then(|st| st.fields.iter().find(|f| f.name == *field_name))
        else {
            failures.push(format!("{struct_name}.{field_name}: no such property"));
            continue;
        };
        if field.field_type != *expected {
            failures.push(format!(
                "{struct_name}.{field_name}: expected {expected:?}, got {:?}",
                field.field_type
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "resolved field types changed:\n  {}",
        failures.join("\n  ")
    );
}

// ─── Unknown-variant guard ─────────────────────────────────────────────────

/// Asserts that no BO4E v202607 enum schema defines a variant literally
/// named `"UNKNOWN"`.
///
/// The generated catch-all `Unknown` variant serializes as `"UNKNOWN"` — this
/// would clash with a legitimate schema variant of the same name.  The guard
/// runs at test time so any future schema addition of `"UNKNOWN"` is caught
/// immediately rather than causing silent data-loss or misrouted dispatch.
#[test]
fn v202607_no_enum_schema_has_unknown_variant() {
    use bo4e_generator::ast::SchemaNode;

    let enum_dir = pinned_schema_root().join("enum");

    let nodes = parser::parse_dir(&enum_dir).expect("parse enum dir");

    let mut conflicts: Vec<String> = Vec::new();
    for node in &nodes {
        if let SchemaNode::Enum(en) = node {
            for (variant, _) in &en.variants {
                if variant == "UNKNOWN" {
                    conflicts.push(format!("{}::UNKNOWN", en.name));
                }
            }
        }
    }

    assert!(
        conflicts.is_empty(),
        "BO4E schema defines 'UNKNOWN' variant(s) that clash with the generated catch-all: {conflicts:?}"
    );
}

// ─── Schema `format` coverage ────────────────────────────────────────────────

/// Every `"format"` the schema uses must be one the parser has a considered
/// position on.
///
/// The parser's `match` ends in a catch-all that maps an unrecognised format to
/// `String`. That is the right default — a Rust type narrower than the schema
/// cannot read what the rest of the ecosystem emits — but it is a *silent* one:
/// a release that starts annotating a field `"format": "uri"` or `"duration"`
/// would map it to `String` with nobody deciding that it should.
///
/// So the set is pinned. A new format fails here, and the fix is to either map
/// it or add it to this list with a reason.
#[test]
fn v202607_every_schema_format_has_a_decision() {
    /// `(format, what the parser does with it)`.
    const KNOWN: &[(&str, &str)] = &[
        ("date-time", "time::OffsetDateTime"),
        ("date", "time::Date"),
        (
            "time",
            "String — BO4E's times carry a UTC offset and `time` has no \
             offset-bearing time-of-day type; parsed on demand by \
             Zeitraum::startuhrzeit_parsed",
        ),
        (
            "decimal",
            "rust_decimal::Decimal — reached via \"type\": \"number\", not via the \
             format annotation",
        ),
    ];

    let mut found: BTreeSet<String> = BTreeSet::new();
    for dir in ["bo", "com", "enum"] {
        let path = pinned_schema_root().join(dir);
        if !path.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&path).expect("readable") {
            let file = entry.expect("entry").path();
            if file.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let doc: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&file).expect("readable"))
                    .expect("valid JSON");
            collect_formats(&doc, &mut found);
        }
    }

    assert!(!found.is_empty(), "the schema uses no formats at all?");
    let known: BTreeSet<&str> = KNOWN.iter().map(|&(f, _)| f).collect();
    let unknown: Vec<&String> = found
        .iter()
        .filter(|f| !known.contains(f.as_str()))
        .collect();
    assert!(
        unknown.is_empty(),
        "the schema uses format(s) {unknown:?} that `resolve_field_type` has no \
         considered position on — they currently fall through to `String`. Map \
         them in `generator/src/parser.rs`, or add them to KNOWN with the reason."
    );
}

/// Every `"format"` string anywhere in `value`, however deeply nested.
fn collect_formats(value: &serde_json::Value, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(f) = map.get("format").and_then(serde_json::Value::as_str) {
                out.insert(f.to_owned());
            }
            for v in map.values() {
                collect_formats(v, out);
            }
        }
        serde_json::Value::Array(items) => items.iter().for_each(|v| collect_formats(v, out)),
        _ => {}
    }
}

//! Integration tests for the bo4e-generator pipeline.
//!
//! These tests parse the committed schema snapshots and verify that the emitter
//! produces the expected Rust source code.  When a generator change would alter
//! the output, the snapshot files in `tests/snapshots/` must be updated to
//! match — making diffs explicit and reviewable.

use std::path::Path;

use bo4e_generator::{emitter, parser};

/// Workspace root, resolved relative to the manifest directory.
fn workspace_root() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR is the generator/ subdirectory.
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().unwrap().to_owned()
}

/// Loads the snapshot file at `tests/snapshots/<name>` and returns its content.
fn load_snapshot(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read snapshot {}: {e}", path.display()))
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

    let (_filename, source) = emitter::emit_node(node, schema_version)
        .unwrap_or_else(|e| panic!("emit_node failed: {e}"));
    source
}

// ─── Snapshot tests ────────────────────────────────────────────────────────

/// `Menge` is a small, stable COM type — a good canary for emitter changes.
#[test]
fn v202501_menge_snapshot() {
    let generated = generate_one("v202501.0.0", "com", "Menge");
    let expected = load_snapshot("v202501_menge.rs");
    if generated != expected {
        eprintln!("=== EXPECTED ===\n{expected}\n=== GENERATED ===\n{generated}");
        panic!(
            "Generator output for Menge changed!\n\
             If this is intentional, update generator/tests/snapshots/v202501_menge.rs."
        );
    }
}

/// Smoke-test: every schema in v202501 must parse without error.
#[test]
fn v202501_all_schemas_parse() {
    let root = workspace_root();
    let schema_root = root.join("generator/schemas/v202501.0.0");
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

    assert!(
        total >= 192,
        "expected at least 192 schema nodes, got {total}"
    );
}

/// Smoke-test: every parsed schema node must emit valid Rust source (no panics,
/// no emitter errors) for v202501.
#[test]
fn v202501_all_schemas_emit() {
    let root = workspace_root();
    let schema_root = root.join("generator/schemas/v202501.0.0");
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

    let errors: Vec<String> = nodes
        .iter()
        .filter_map(|n| {
            emitter::emit_node(n, "v202501.0.0")
                .err()
                .map(|e| format!("{}: {e}", n.name()))
        })
        .collect();

    assert!(errors.is_empty(), "emitter errors:\n{}", errors.join("\n"));
}

// ─── AST shape tests ───────────────────────────────────────────────────────

/// Verifies that `Vertrag.json` from v202501.0.0 parses into the expected AST shape.
///
/// This test guards against parser regressions: if a field is silently dropped or its
/// type inference changes, the assertion will catch it.
#[test]
fn v202501_vertrag_ast_shape() {
    use bo4e_generator::ast::{FieldType, PrimitiveType, SchemaNode};

    let root = workspace_root();
    let bo_dir = root.join("generator/schemas/v202501.0.0/bo");
    let nodes = parser::parse_dir(&bo_dir).expect("parse bo dir");

    let vertrag = nodes
        .iter()
        .find(|n| n.name() == "Vertrag")
        .expect("Vertrag not found in BO schema dir");

    // Must parse as a BO type.
    let bo = match vertrag {
        SchemaNode::Bo(b) => b,
        other => panic!("expected BoNode, got {:?}", std::mem::discriminant(other)),
    };

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

/// Verifies that the inference table correctly maps specific high-value fields
/// across the real v202501.0.0 BO/COM schemas, guarding against regressions
/// where a schema change or parser update silently switches a field's type.
///
/// Covers: identifier newtypes, OffsetDateTime, Decimal, and JsonValue fields.
/// When a field is not present in a schema, the test simply skips it — this
/// keeps the test forward-compatible with schema versions that drop optional fields.
#[test]
fn v202501_inference_audit() {
    use bo4e_generator::ast::{FieldType, PrimitiveType, SchemaNode};

    let root = workspace_root();
    let schema_root = root.join("generator/schemas/v202501.0.0");

    // Collect all BO and COM nodes.
    let mut nodes: Vec<SchemaNode> = Vec::new();
    for category in ["bo", "com"] {
        let dir = schema_root.join(category);
        if dir.exists() {
            nodes.extend(
                parser::parse_dir(&dir)
                    .unwrap_or_else(|e| panic!("parse_dir({category}) failed: {e}")),
            );
        }
    }
    if let Some(n) = parser::parse_file_as_com(&schema_root.join("ZusatzAttribut.json"))
        .expect("parse ZusatzAttribut.json")
    {
        nodes.push(n);
    }

    // Helper: find a field by JSON name in a node's field list.
    let find_field = |node: &SchemaNode, field_name: &str| -> Option<FieldType> {
        let fields: &[bo4e_generator::ast::Field] = match node {
            SchemaNode::Bo(b) => &b.fields,
            SchemaNode::Com(c) => &c.fields,
            SchemaNode::Enum(_) => return None,
        };
        fields
            .iter()
            .find(|f| f.name == field_name)
            .map(|f| f.field_type.clone())
    };

    /// Audit entry: (struct_name, field_json_name, expected_FieldType)
    type Entry<'a> = (&'a str, &'a str, FieldType);

    let cases: &[Entry] = &[
        // ── Identifier newtypes ─────────────────────────────────────────────
        (
            "Marktlokation",
            "marktlokationsId",
            FieldType::Identifier("MaloId".into()),
        ),
        (
            "Messlokation",
            "messlokationsId",
            FieldType::Identifier("MeloId".into()),
        ),
        (
            "Netzlokation",
            "netzlokationsId",
            FieldType::Identifier("NeloId".into()),
        ),
        (
            "Marktlokation",
            "bilanzkreis",
            FieldType::Identifier("EicCode".into()),
        ),
        (
            "Marktlokation",
            "obisKennzahl",
            FieldType::Identifier("ObisCode".into()),
        ),
        (
            "Marktteilnehmer",
            "rollencodenummer",
            FieldType::Identifier("MarktpartnerId".into()),
        ),
        // ── OffsetDateTime fields ───────────────────────────────────────────
        (
            "Vertrag",
            "vertragsbeginn",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        (
            "Vertrag",
            "vertragsende",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        (
            "Rechnung",
            "rechnungsdatum",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        (
            "Rechnung",
            "faelligkeitsdatum",
            FieldType::Primitive(PrimitiveType::OffsetDateTime),
        ),
        // ── Decimal fields ─────────────────────────────────────────────────
        (
            "Betrag",
            "wert",
            FieldType::Primitive(PrimitiveType::Decimal),
        ),
        (
            "Preis",
            "wert",
            FieldType::Primitive(PrimitiveType::Decimal),
        ),
        (
            "Menge",
            "wert",
            FieldType::Primitive(PrimitiveType::Decimal),
        ),
        // ── JsonValue (free-form struct override) ──────────────────────────
        ("ZusatzAttribut", "wert", FieldType::JsonValue),
        // ── Schema $ref wins over suffix inference ──────────────────────────
        // These fields end with a suffix in the inference table (e.g. "preis" →
        // Decimal, "menge" → Decimal) but their schemas carry an explicit `$ref`
        // to a structured COM type.  The parser must trust the `$ref` and produce
        // the correct `Com(…)` variant rather than a bare `Primitive(Decimal)`.
        (
            "Rechnungsposition",
            "einzelpreis",
            FieldType::Com("Preis".into()),
        ),
        (
            "Angebotsposition",
            "positionsmenge",
            FieldType::Com("Menge".into()),
        ),
        (
            "Angebotsposition",
            "positionspreis",
            FieldType::Com("Preis".into()),
        ),
        (
            "Kostenposition",
            "einzelpreis",
            FieldType::Com("Preis".into()),
        ),
        ("Kostenposition", "menge", FieldType::Com("Menge".into())),
        (
            "Fremdkostenposition",
            "einzelpreis",
            FieldType::Com("Preis".into()),
        ),
        (
            "Fremdkostenposition",
            "menge",
            FieldType::Com("Menge".into()),
        ),
        (
            "Tarifberechnungsparameter",
            "mindestpreis",
            FieldType::Com("Preis".into()),
        ),
        ("Bilanzierung", "kundenwert", FieldType::Com("Menge".into())),
    ];

    let mut failures: Vec<String> = Vec::new();

    for (struct_name, field_name, expected) in cases {
        let node = match nodes.iter().find(|n| n.name() == *struct_name) {
            Some(n) => n,
            None => {
                // Schema may not exist in this version — skip gracefully.
                continue;
            }
        };
        match find_field(node, field_name) {
            None => {
                // Field not in schema for this version — skip.
                continue;
            }
            Some(actual) if &actual == expected => {} // ✓
            Some(actual) => {
                failures.push(format!(
                    "{struct_name}.{field_name}: expected {expected:?}, got {actual:?}"
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Inference audit failures:\n{}",
        failures.join("\n")
    );
}

// ─── Unknown-variant guard ─────────────────────────────────────────────────

/// Asserts that no BO4E v202501.0.0 enum schema defines a variant literally
/// named `"UNKNOWN"`.
///
/// The generated catch-all `Unknown` variant serializes as `"UNKNOWN"` — this
/// would clash with a legitimate schema variant of the same name.  The guard
/// runs at test time so any future schema addition of `"UNKNOWN"` is caught
/// immediately rather than causing silent data-loss or misrouted dispatch.
#[test]
fn v202501_no_enum_schema_has_unknown_variant() {
    use bo4e_generator::ast::SchemaNode;

    let root = workspace_root();
    let enum_dir = root.join("generator/schemas/v202501.0.0/enum");

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

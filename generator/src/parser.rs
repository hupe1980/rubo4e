use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use heck::ToSnakeCase;
use serde_json::Value;

use crate::ast::{EnumNode, Field, FieldType, PrimitiveType, SchemaNode, StructKind, StructNode};
use crate::inference;

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Parses all `*.json` files in `schema_dir` and returns a list of schema nodes.
///
/// Files are read in deterministic (sorted) order so that generated output is
/// reproducible across platforms and runs.
pub fn parse_dir(schema_dir: &Path) -> Result<Vec<SchemaNode>> {
    let mut paths: Vec<_> = std::fs::read_dir(schema_dir)
        .with_context(|| format!("cannot read schema directory: {}", schema_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    paths.sort();

    let mut nodes = Vec::with_capacity(paths.len());
    for path in &paths {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read {}", path.display()))?;
        let value: Value = serde_json::from_str(&text)
            .with_context(|| format!("invalid JSON in {}", path.display()))?;
        let node = parse_schema(&value, path)
            .with_context(|| format!("parsing schema {}", path.display()))?;
        nodes.push(node);
    }
    Ok(nodes)
}

/// Parses a single `*.json` file as a COM-type schema node.
///
/// Returns `None` if the file cannot be parsed as an object schema (e.g. pure
/// enum or metadata file). Used for root-level schemas like `ZusatzAttribut.json`.
pub fn parse_file_as_com(path: &Path) -> Result<Option<SchemaNode>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    // Only parse object schemas (type == "object") — skip plain enum files.
    if value.get("type").and_then(|v| v.as_str()) != Some("object") {
        return Ok(None);
    }
    let node =
        parse_schema(&value, path).with_context(|| format!("parsing schema {}", path.display()))?;
    Ok(Some(node))
}

// ─── Schema-level dispatch ────────────────────────────────────────────────────

fn parse_schema(value: &Value, path: &Path) -> Result<SchemaNode> {
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .or_else(|| path.file_stem().and_then(|s| s.to_str()))
        .context("schema has no 'title' and file stem is non-UTF-8")?
        .to_owned();

    let description = value
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    // Detect enum schemas by the presence of an `enum` array.
    if value.get("enum").is_some() {
        return parse_enum(value, title, description);
    }

    // Differentiate BO (business object) from COM (component) by convention:
    // BO4E files under bo/ are BOs; files under com/ are COMs. Fallback: "properties" key.
    let category = path
        .components()
        .find_map(|c| {
            let s = c.as_os_str().to_str()?;
            if s == "bo" || s == "com" || s == "enum" {
                Some(s.to_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "com".to_owned());

    if category == "enum" {
        return parse_enum(value, title, description);
    }

    let properties = value.get("properties").and_then(|v| v.as_object());
    let required: Vec<String> = value
        .get("required")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                .collect()
        })
        .unwrap_or_default();

    let fields = if let Some(props) = properties {
        parse_fields_with_parent(props, &required, Some(&title))?
    } else {
        Vec::new()
    };

    Ok(SchemaNode::Struct(StructNode {
        name: title,
        kind: if category == "bo" {
            StructKind::Bo
        } else {
            StructKind::Com
        },
        fields,
        description,
        typ_const: properties.and_then(|p| metadata_literal(p, "_typ")),
        version_default: properties.and_then(|p| metadata_literal(p, "_version")),
    }))
}

/// Reads the wire value a BO4E metadata property (`_typ`, `_version`) pins.
///
/// BO4E declares these two ways: `_typ` is usually `{"const": "MARKTLOKATION",
/// "default": "MARKTLOKATION"}`, but a few schemas declare it as a nullable
/// `$ref` to `BoTyp` carrying only a `default`. `_version` only ever has a
/// `default`, and it is **not** the release tag — the tag is `v202607.0.0`, the
/// wire value `202607.0.0`.
fn metadata_literal(props: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
    let prop = props.get(key)?;
    prop.get("const")
        .or_else(|| prop.get("default"))
        .and_then(|v| v.as_str())
        .map(str::to_owned)
}

// ─── Enum parsing ─────────────────────────────────────────────────────────────

fn parse_enum(value: &Value, name: String, description: Option<String>) -> Result<SchemaNode> {
    let enum_values = value
        .get("enum")
        .and_then(|v| v.as_array())
        .context("expected 'enum' array")?;

    // BO4E enums often carry per-value descriptions in `x-enumDescriptions` or
    // in a parallel `enumDescriptions` field (varies by schema version).
    let descriptions: HashMap<String, String> = value
        .get("x-enumDescriptions")
        .or_else(|| value.get("enumDescriptions"))
        .and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned()))
                .collect()
        })
        .unwrap_or_default();

    let variants = enum_values
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| (s.to_owned(), descriptions.get(s).cloned()))
        .collect();

    Ok(SchemaNode::Enum(EnumNode {
        name,
        variants,
        description,
    }))
}

// ─── Field parsing ────────────────────────────────────────────────────────────

fn parse_fields_with_parent(
    props: &serde_json::Map<String, Value>,
    required: &[String],
    parent_name: Option<&str>,
) -> Result<Vec<Field>> {
    let required_set: HashSet<&str> = required.iter().map(String::as_str).collect();
    let mut fields = Vec::with_capacity(props.len());
    for (json_name, schema) in props {
        let field = parse_field(
            json_name,
            schema,
            required_set.contains(json_name.as_str()),
            parent_name,
        )?;
        fields.push(field);
    }
    // Sort alphabetically by Rust name for deterministic output.
    fields.sort_by(|a, b| a.rust_name.cmp(&b.rust_name));
    Ok(fields)
}

fn parse_field(
    json_name: &str,
    schema: &Value,
    is_required: bool,
    parent_name: Option<&str>,
) -> Result<Field> {
    let rust_name = json_name.to_snake_case();
    let description = schema
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    // Resolve the schema's structural type first.
    let schema_type = resolve_field_type(schema);

    // Semantic inference enriches raw primitive types (string → identifier newtype,
    // number → Decimal, string → OffsetDateTime).  It never overrides a type the
    // schema states outright:
    //
    // * A `$ref` to a BO or COM is authoritative.  `einzelpreis` ends with the
    //   "preis" suffix but its schema says `$ref: Preis.json`, so the field is
    //   `Option<Preis>`, not `Option<Decimal>`.
    // * A `"format": "date"` / `"date-time"` annotation is authoritative too.
    //   A Rust type narrower than the schema cannot read what the rest of the
    //   ecosystem emits for that field.
    let field_type = match &schema_type {
        FieldType::Bo(_)
        | FieldType::Com(_)
        | FieldType::Primitive(PrimitiveType::OffsetDateTime)
        | FieldType::Primitive(PrimitiveType::Date) => schema_type,
        _ => inference::infer_with_parent(parent_name, json_name).unwrap_or(schema_type),
    };

    // Fields not in `required` are optional.
    let is_optional = !is_required;

    Ok(Field {
        name: json_name.to_owned(),
        rust_name,
        is_optional,
        field_type,
        description,
    })
}

// ─── Type resolution ──────────────────────────────────────────────────────────

fn resolve_field_type(schema: &Value) -> FieldType {
    // Handle `$ref` → extract type name from last path segment.
    if let Some(r) = schema.get("$ref").and_then(|v| v.as_str()) {
        return ref_to_field_type(r);
    }

    // `anyOf` / `oneOf` with a single non-null variant → unwrap the variant.
    for key in ["anyOf", "oneOf"] {
        if let Some(arr) = schema.get(key).and_then(|v| v.as_array()) {
            let non_null: Vec<_> = arr
                .iter()
                .filter(|v| v.get("type").and_then(|t| t.as_str()) != Some("null"))
                .collect();
            if non_null.len() == 1 {
                return resolve_field_type(non_null[0]);
            }
            if non_null.len() > 1 {
                return FieldType::JsonValue; // true union — keep as serde_json::Value
            }
        }
    }

    match schema.get("type").and_then(|v| v.as_str()) {
        Some("string") => match schema.get("format").and_then(|v| v.as_str()) {
            Some("date-time") => FieldType::Primitive(PrimitiveType::OffsetDateTime),
            Some("date") => FieldType::Primitive(PrimitiveType::Date),
            // `time` is a *deliberate* passthrough, not an oversight. BO4E's
            // examples carry a UTC offset (`"18:00:00+01:00"`) and the `time`
            // crate has no offset-bearing time-of-day type, so there is nothing
            // narrower than `String` to map it to without dropping the offset.
            // `Zeitraum::startuhrzeit_parsed` parses it on demand instead.
            //
            // Any *other* format falls through here too, which is the right
            // default — but a silent one. `generator/tests/round_trip.rs` fails
            // if the schema introduces a format this match has no considered
            // position on, so the decision gets made rather than defaulted.
            Some("time") | None => FieldType::Primitive(PrimitiveType::String),
            _ => FieldType::Primitive(PrimitiveType::String),
        },
        Some("boolean") => FieldType::Primitive(PrimitiveType::Bool),
        Some("integer") => FieldType::Primitive(PrimitiveType::I64),
        Some("number") => FieldType::Primitive(PrimitiveType::Decimal),
        Some("array") => {
            let items = schema
                .get("items")
                .map(resolve_field_type)
                .unwrap_or(FieldType::JsonValue);
            FieldType::Array(Box::new(items))
        }
        Some("object") | None => FieldType::JsonValue,
        _ => FieldType::JsonValue,
    }
}

fn ref_to_field_type(ref_str: &str) -> FieldType {
    // `$ref` values look like:
    //   "#/definitions/Foo"
    //   "../bo/Angebot.json"
    //   "../com/Preis.json"
    //   "../enum/Waehrungscode.json"
    let name = ref_str
        .rsplit('/')
        .next()
        .unwrap_or(ref_str)
        .trim_end_matches(".json")
        .to_owned();

    // Determine category from the path prefix when available.
    if ref_str.contains("/bo/") {
        FieldType::Bo(name)
    } else if ref_str.contains("/enum/") {
        FieldType::BoEnum(name)
    } else {
        FieldType::Com(name)
    }
}

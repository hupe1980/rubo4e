use anyhow::Result;
use heck::ToUpperCamelCase;
use std::collections::HashSet;

use crate::ast::{EnumNode, Field, FieldType, PrimitiveType, SchemaNode};

// ─── AnyBo emission ──────────────────────────────────────────────────────────

/// Emits the `AnyBo` sum type for a given schema version.
///
/// `bo_names` must be sorted and contain UpperCamelCase struct names for
/// every BO type in the version (i.e. every `SchemaNode::Bo` name).
fn emit_any_bo(bo_names: &[String]) -> String {
    if bo_names.is_empty() {
        return String::new();
    }

    let mut s = String::new();

    // ── Enum definition ────────────────────────────────────────────────────
    s.push_str("/// Sum type over **all** BO4E Geschäftsobjekte for dynamic type dispatch.\n");
    s.push_str("///\n");
    s.push_str("/// Use this when you receive a JSON message where the concrete BO type is\n");
    s.push_str("/// determined at runtime by the `\"_typ\"` discriminant field.\n");
    s.push_str("///\n");
    s.push_str("/// Deserialization requires the `json` feature (uses `serde_json::Value` for\n");
    s.push_str(
        "/// the two-pass `_typ` peeking strategy).  Serialization requires only `serde`.\n",
    );
    s.push_str("///\n");
    s.push_str("/// # Example\n");
    s.push_str("/// ```rust,ignore\n");
    s.push_str("/// use rubo4e::v202501::AnyBo;\n");
    s.push_str("/// let json = r#\"{\"_typ\":\"MARKTLOKATION\",\"marktlokationsId\":\"51238696780\"}\"#;\n");
    s.push_str("/// let bo: AnyBo = serde_json::from_str(json)?;\n");
    s.push_str("/// if let AnyBo::Marktlokation(malo) = bo {\n");
    s.push_str("///     println!(\"ID: {:?}\", malo.marktlokations_id);\n");
    s.push_str("/// }\n");
    s.push_str("/// ```\n");
    s.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    s.push_str("#[cfg_attr(not(feature = \"json\"), derive(Hash))]\n");
    s.push_str("#[non_exhaustive]\n");
    s.push_str("pub enum AnyBo {\n");
    for name in bo_names {
        s.push_str(&format!("    /// A [`{name}`] Geschäftsobjekt.\n"));
        s.push_str(&format!("    {name}(Box<{name}>),\n"));
    }
    s.push_str(
        "    /// Unrecognized `_typ` value — raw JSON preserved for forward-compatibility.\n",
    );
    s.push_str("    ///\n");
    s.push_str("    /// Produced when the `_typ` string is not matched by any known variant.\n");
    s.push_str("    /// Allows graceful handling of new BO types without a library upgrade.\n");
    s.push_str("    #[cfg(feature = \"json\")]\n");
    s.push_str("    Unknown {\n");
    s.push_str("        /// The raw value of the `_typ` field.\n");
    s.push_str("        typ: String,\n");
    s.push_str("        /// The full JSON object for inspection or re-serialization.\n");
    s.push_str("        data: serde_json::Value,\n");
    s.push_str("    },\n");
    s.push_str("}\n\n");

    // ── Inherent methods ───────────────────────────────────────────────────
    s.push_str("impl AnyBo {\n");
    s.push_str("    /// Returns the [`BoTyp`] discriminant for this BO object.\n");
    s.push_str("    ///\n");
    s.push_str("    /// Delegates to the inner type's [`Bo4eObject::bo_type`] for all known\n");
    s.push_str("    /// variants; returns [`BoTyp::Unknown`] for the `Unknown` catch-all.\n");
    s.push_str("    pub fn bo_type(&self) -> BoTyp {\n");
    s.push_str("        match self {\n");
    for name in bo_names {
        s.push_str(&format!("            AnyBo::{name}(v) => v.bo_type(),\n"));
    }
    s.push_str("            #[cfg(feature = \"json\")]\n");
    s.push_str("            AnyBo::Unknown { .. } => BoTyp::Unknown,\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n\n");

    // ── From<T> for AnyBo ─────────────────────────────────────────────────
    for name in bo_names {
        s.push_str(&format!("impl From<{name}> for AnyBo {{\n"));
        s.push_str(&format!(
            "    fn from(v: {name}) -> Self {{ AnyBo::{name}(Box::new(v)) }}\n"
        ));
        s.push_str("}\n");
        s.push_str(&format!("impl From<Box<{name}>> for AnyBo {{\n"));
        s.push_str(&format!(
            "    fn from(v: Box<{name}>) -> Self {{ AnyBo::{name}(v) }}\n"
        ));
        s.push_str("}\n");
    }
    s.push('\n');

    // ── Serialize ─────────────────────────────────────────────────────────
    s.push_str("#[cfg(feature = \"serde\")]\n");
    s.push_str("impl serde::Serialize for AnyBo {\n");
    s.push_str(
        "    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\n",
    );
    s.push_str("        match self {\n");
    for name in bo_names {
        s.push_str(&format!(
            "            AnyBo::{name}(inner) => inner.serialize(s),\n"
        ));
    }
    s.push_str("            #[cfg(feature = \"json\")]\n");
    s.push_str("            AnyBo::Unknown { data, .. } => data.serialize(s),\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n\n");

    // ── Deserialize — requires json feature for _typ peek via serde_json::Value
    s.push_str("#[cfg(all(feature = \"serde\", feature = \"json\"))]\n");
    s.push_str("impl<'de> serde::Deserialize<'de> for AnyBo {\n");
    s.push_str(
        "    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\n",
    );
    s.push_str("        // Two-pass strategy: capture the full JSON object into a Value so we\n");
    s.push_str("        // can peek at `_typ` before dispatching to the concrete deserializer.\n");
    s.push_str(
        "        // The overhead is one extra allocation; acceptable for type-dispatch paths.\n",
    );
    s.push_str("        let raw = serde_json::Value::deserialize(d)?;\n");
    s.push_str(
        "        let typ_str = raw.get(\"_typ\").and_then(|v| v.as_str()).unwrap_or(\"\");\n",
    );
    s.push_str("        match typ_str {\n");
    for name in bo_names {
        let typ_key = name.to_ascii_uppercase();
        s.push_str(&format!(
            "            \"{typ_key}\" => serde_json::from_value::<{name}>(raw)\n"
        ));
        s.push_str(&format!(
            "                .map(|v| AnyBo::{name}(Box::new(v)))\n"
        ));
        s.push_str("                .map_err(serde::de::Error::custom),\n");
    }
    s.push_str("            _ => Ok(AnyBo::Unknown { typ: typ_str.to_owned(), data: raw }),\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n\n");

    // ── Bo4eJsonExt — gives AnyBo all the from_json_german / to_json_german API
    s.push_str("#[cfg(feature = \"json\")]\n");
    s.push_str("impl crate::json::sealed::Sealed for AnyBo {}\n");
    s.push_str("#[cfg(feature = \"json\")]\n");
    s.push_str("impl crate::json::Bo4eJsonExt for AnyBo {}\n");

    s
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Emits the Rust source file for a single schema node.
/// Returns `(filename, source_code)`.
///
/// `schema_version` is the full schema version tag, e.g. `"v202501.0.0"`, used to
/// populate [`Bo4eObject::schema_version`] on generated BO types.
pub fn emit_node(node: &SchemaNode, schema_version: &str) -> Result<(String, String)> {
    // Normalize the type name to UpperCamelCase so it matches the type references
    // produced by the inference module (which also calls to_upper_camel_case() on
    // $ref-derived names like "BDEWArtikelnummer" → "BdewArtikelnummer").
    let rust_name = node.name().to_upper_camel_case();
    let source = match node {
        SchemaNode::Bo(bo) => emit_struct(
            &rust_name,
            &bo.fields,
            bo.description.as_deref(),
            true,
            schema_version,
        ),
        SchemaNode::Com(com) => emit_struct(
            &rust_name,
            &com.fields,
            com.description.as_deref(),
            false,
            schema_version,
        ),
        SchemaNode::Enum(en) => {
            let mut en2 = en.clone();
            en2.name = rust_name.clone();
            emit_enum(&en2)
        }
    }?;
    let filename = format!("{}.rs", heck::AsSnakeCase(&rust_name));
    Ok((filename, source))
}

/// Emits a `mod.rs` that re-exports every node name in `nodes` and re-exports
/// the crate-level [`Bo4eObject`] trait so struct files can impl it via `use super::*;`.
///
/// `schema_version` is passed through for completeness but is not currently
/// used in the `mod.rs` body (the trait lives in `src/lib.rs`).
pub fn emit_mod_rs(nodes: &[SchemaNode], _schema_version: &str) -> Result<String> {
    let mut s = String::from("// @generated — do not edit by hand\n\n");
    // Sort alphabetically so mod.rs is stable regardless of how the schema
    // parser enumerates JSON Schema files (directory traversal order varies
    // across platforms and filesystems).
    let mut sorted_nodes: Vec<&SchemaNode> = nodes.iter().collect();
    sorted_nodes.sort_by_key(|n| n.name().to_upper_camel_case());
    for node in &sorted_nodes {
        let rust_name = node.name().to_upper_camel_case();
        let mod_name = heck::AsSnakeCase(&rust_name).to_string();
        s.push_str(&format!("pub mod {mod_name};\n"));
    }
    s.push('\n');
    for node in &sorted_nodes {
        let rust_name = node.name().to_upper_camel_case();
        let mod_name = heck::AsSnakeCase(&rust_name).to_string();
        s.push_str(&format!("pub use {mod_name}::{rust_name};\n"));
    }
    // ── Bo4eObject re-export ──────────────────────────────────────────────────
    s.push_str("// Re-export the crate-level Bo4eObject so struct files can call trait methods.\npub use crate::Bo4eObject;\n");

    // ── AnyBo: heterogeneous dispatch enum ───────────────────────────────────
    let bo_names: Vec<String> = sorted_nodes
        .iter()
        .filter(|n| matches!(n, SchemaNode::Bo(_)))
        .map(|n| n.name().to_upper_camel_case())
        .collect();
    s.push_str(&emit_any_bo(&bo_names));

    format_source(s)
}

// ─── Struct emission ──────────────────────────────────────────────────────────

/// Collects the set of sibling type names (exported by the version `mod.rs`)
/// that a generated struct file must explicitly import from `super`.
///
/// Returned names are sorted and deduplicated.  The struct's own name is never
/// included — it is being *defined*, not imported.
fn collect_sibling_imports(name: &str, fields: &[Field], is_bo: bool) -> Vec<String> {
    let mut set = HashSet::new();

    // BO types always need `BoTyp` (used in the field definition, in `impl Bo4eObject`,
    // and as the associated `type BoTyp = BoTyp` alias) and `Bo4eObject` (the trait).
    if is_bo {
        set.insert("BoTyp".to_string());
        set.insert("Bo4eObject".to_string());
    }

    // COM types need `ComTyp` only when the struct has a `_typ` field.
    if !is_bo && fields.iter().any(|f| f.name == "_typ") {
        set.insert("ComTyp".to_string());
    }

    // Walk field types to discover referenced sibling BO/COM/enum names.
    for field in fields {
        collect_field_type_names(&field.field_type, &mut set);
    }

    // Never import the type being defined (its name is already normalized by emit_node).
    set.remove(name);

    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

/// Recursive helper: adds referenced sibling type names to `out`.
///
/// Names are normalized to `UpperCamelCase` to match the identifiers used in the
/// generated Rust code (same normalization applied by `field_type_to_rust`).
fn collect_field_type_names(ft: &FieldType, out: &mut HashSet<String>) {
    match ft {
        FieldType::Bo(n) | FieldType::Com(n) | FieldType::BoEnum(n) => {
            out.insert(n.to_upper_camel_case());
        }
        FieldType::Array(inner) => collect_field_type_names(inner, out),
        FieldType::Identifier(_) | FieldType::Primitive(_) | FieldType::JsonValue => {}
    }
}

fn emit_struct(
    name: &str,
    fields: &[Field],
    description: Option<&str>,
    is_bo: bool,
    schema_version: &str,
) -> Result<String> {
    let mut s = String::from("// @generated — do not edit by hand\n\n");
    // Emit explicit imports of all sibling types actually referenced by this struct.
    // Avoids the wildcarded `use super::*` (with its suppressed unused-import warning)
    // and makes cross-references visible to tools (rustdoc, IDEs, cargo check).
    let imports = collect_sibling_imports(name, fields, is_bo);
    if !imports.is_empty() {
        s.push_str(&format!("use super::{{{}}};\n\n", imports.join(", ")));
    }

    // BO types with a `_typ` field get a custom Default impl (H-05) so that
    // `Default::default()` pre-fills `typ` with the correct `BoTyp` discriminant,
    // producing structurally valid BO4E JSON without any manual field setting.
    let has_typ_field = is_bo && fields.iter().any(|f| f.name == "_typ");
    // BoTyp enum variants are generated from screaming-case schema values
    // (e.g. `PREISBLATTDIENSTLEISTUNG`) via `.to_upper_camel_case()` → `Preisblattdienstleistung`.
    // Internal word-boundary information is lost because the schema provides no separators.
    // To match those variants from struct impls, apply the same normalization to the struct
    // name: uppercase first (destroy acronym capitalisation), then re-camelise.
    // This guarantees struct references always agree with the generated enum variants.
    let bo_typ_variant = name.to_ascii_uppercase().to_upper_camel_case();

    emit_struct_derives(&mut s, has_typ_field, name, schema_version);

    // Doc comment: strip RST directives (from BO4E Python docs) and convert to Markdown.
    if let Some(doc) = description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("/// {}\n", line.trim()));
        }
    }

    s.push_str(&format!("pub struct {name} {{\n"));

    for field in fields {
        // For BO structs, replace the raw `_typ: Option<String>` with the typed `BoTyp`.
        if is_bo && field.name == "_typ" {
            s.push_str("    /// BO type identifier — always `BoTyp::");
            s.push_str(name);
            s.push_str("` for this struct.\n");
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(rename = \"_typ\"))]\n");
            s.push_str(
                "    #[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"Option::is_none\"))]\n",
            );
            s.push_str(&format!(
                "    #[cfg_attr(feature = \"builder\", builder(default = Some(BoTyp::{bo_typ_variant}), setter(skip)))]\n"
            ));
            s.push_str("    pub typ: Option<BoTyp>,\n");
        // For COM structs, replace the raw `_typ: Option<String>` with the typed `ComTyp`.
        } else if !is_bo && field.name == "_typ" {
            s.push_str("    /// COM type identifier for this struct.\n");
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(rename = \"_typ\"))]\n");
            s.push_str(
                "    #[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"Option::is_none\"))]\n",
            );
            s.push_str(
                "    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n",
            );
            s.push_str("    pub typ: Option<ComTyp>,\n");
        } else {
            emit_field(&mut s, field);
        }
    }

    emit_extension_field(&mut s);
    s.push_str("}\n");

    if has_typ_field {
        emit_default_impl(&mut s, name, &bo_typ_variant, fields);
    }

    emit_struct_impls(&mut s, name, is_bo, &bo_typ_variant, schema_version);

    format_source(s)
}

/// Emits the `#[derive(...)]` and `#[cfg_attr(...)]` attribute block for a struct.
fn emit_struct_derives(s: &mut String, has_typ_field: bool, name: &str, schema_version: &str) {
    // BO types with `_typ` omit `Default` here; a handwritten impl is emitted below.
    if has_typ_field {
        s.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    } else {
        s.push_str("#[derive(Debug, Clone, PartialEq, Default)]\n");
    }
    // Hash: serde_json::Value (inside LimitedExtensionMap) is not Hash, so we
    // can only derive Hash when the `json` feature is off (the _additional field
    // is a ZST stub that IS Hash).  This lets non-json builds use BO types as
    // HashMap / HashSet keys, which is common for ID-keyed lookups.
    s.push_str("#[cfg_attr(not(feature = \"json\"), derive(Hash))]\n");
    s.push_str("#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n");
    s.push_str("#[cfg_attr(feature = \"builder\", derive(typed_builder::TypedBuilder))]\n");
    s.push_str("#[cfg_attr(feature = \"validate\", derive(garde::Validate))]\n");
    // allow_unvalidated: fields without an explicit #[garde(...)] attribute are
    // implicitly accepted.  Only identifier fields get #[garde(dive)].
    s.push_str("#[cfg_attr(feature = \"validate\", garde(allow_unvalidated))]\n");
    s.push_str("#[cfg_attr(feature = \"schemars\", derive(schemars::JsonSchema))]\n");
    s.push_str("#[cfg_attr(feature = \"utoipa\", derive(utoipa::ToSchema))]\n");
    // Cross-field validators for types with non-trivial business rules.
    // The path is versioned so both v202401 and v202501 get correct impls.
    let ver = schema_version.split('.').next().unwrap_or(schema_version);
    if let Some(validator) = cross_field_validator(name, ver) {
        s.push_str(&format!(
            "#[cfg_attr(all(feature = \"validate\", feature = \"versioned\"), garde(custom({validator})))]\n"
        ));
    }
}

/// Emits the `_additional` extension-data field declaration.
fn emit_extension_field(s: &mut String) {
    // Extension data: captures unknown JSON fields for transparent round-trip
    // preservation.  Gated on `json` because it requires `serde_json::Value`.
    //
    // H-03: single `_additional` field using a cfg-adaptive type rather than
    // two cfg-gated duplicate declarations.  When `json` is on this is the real
    // `LimitedExtensionMap`; when off it is the ZST stub — same field name, same
    // serde skip/flatten semantics, no cfg branch in struct body.
    s.push_str("    /// Unknown JSON fields captured during deserialization for round-trip preservation.\n");
    s.push_str("    /// `None` when no unknown fields were present (zero heap allocation).\n");
    s.push_str("    #[cfg_attr(feature = \"json\", serde(flatten))]\n");
    s.push_str("    #[cfg_attr(feature = \"json\", serde(skip_serializing_if = \"crate::json::ext_map_is_empty\"))]\n");
    s.push_str("    #[cfg_attr(not(feature = \"json\"), serde(skip))]\n");
    s.push_str("    #[cfg_attr(feature = \"builder\", builder(default, setter(skip)))]\n");
    s.push_str("    pub(crate) _additional: crate::LimitedExtensionMap,\n");
}

/// Emits a custom `Default` impl for a BO struct that pre-fills `typ` with the correct variant.
fn emit_default_impl(s: &mut String, name: &str, bo_typ_variant: &str, fields: &[Field]) {
    // H-05: BO types with a `_typ` field get a custom Default impl that pre-fills `typ`
    // so that `Default::default()` produces structurally valid BO4E JSON with `"_typ"`
    // present, matching the BO4E specification requirement for the type discriminant.
    s.push_str(&format!(
        "\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self {{\n"
    ));
    s.push_str(&format!(
        "            typ: Some(BoTyp::{bo_typ_variant}),\n"
    ));
    for field in fields {
        if field.name == "_typ" {
            continue; // already emitted as `typ` above
        }
        s.push_str(&format!(
            "            {}: Default::default(),\n",
            field.rust_name
        ));
    }
    s.push_str("            _additional: Default::default(),\n");
    s.push_str("        }\n    }\n}\n");
}

/// Emits all trait impls for a generated struct: `Bo4eObject`, `Bo4eJsonExt`, `Sealed`,
/// `Bo4eExtensionData`, and `Display`.
fn emit_struct_impls(
    s: &mut String,
    name: &str,
    is_bo: bool,
    bo_typ_variant: &str,
    schema_version: &str,
) {
    // Bo4eObject impl — only BO types carry the BoTyp discriminant.
    // `type BoTyp = BoTyp;` binds the associated type from crate::Bo4eObject to the
    // local version-specific BoTyp enum so the impl compiles and dyn usage works as
    // `dyn Bo4eObject<BoTyp = v202501::BoTyp>`.
    if is_bo {
        // H-07: return the runtime `typ` field value so callers doing dynamic dispatch
        // see the actual discriminant from the payload (e.g. "BUENDELVERTRAG"), not the
        // hardcoded struct name.  `unwrap_or` falls back to the compile-time constant only
        // when the field was explicitly set to `None` after construction.
        s.push_str(&format!("\nimpl Bo4eObject for {name} {{\n    type BoTyp = BoTyp;\n    fn bo_type(&self) -> BoTyp {{\n        self.typ.unwrap_or(BoTyp::{bo_typ_variant})\n    }}\n    fn schema_version(&self) -> &'static str {{\n        \"{schema_version}\"\n    }}\n}}\n"));
    }

    // H-02: Sealed marker + Bo4eJsonExt impl — restricts trait to BO4E types only.
    s.push_str(&format!(
        "\n#[cfg(feature = \"json\")]\nimpl crate::json::sealed::Sealed for {name} {{}}\n"
    ));
    s.push_str(&format!(
        "#[cfg(feature = \"json\")]\nimpl crate::json::Bo4eJsonExt for {name} {{}}\n"
    ));

    // H-01/Bo4eExtensionData: lazy-init getter returns static empty map for None case.
    s.push_str(&format!(
        "\n#[cfg(feature = \"json\")]\nimpl crate::json::Bo4eExtensionData for {name} {{\n"
    ));
    s.push_str(
        "    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {\n",
    );
    // I-5: use the single crate-level EMPTY_EXTENSION_MAP instead of a per-struct
    // LazyLock static.  This reduces cold-start allocations from O(n_types) to O(1).
    s.push_str("        self._additional.as_map().unwrap_or(&crate::json::extension::EMPTY_EXTENSION_MAP)\n");
    s.push_str("    }\n");
    s.push_str("    fn has_extension_data(&self) -> bool {\n");
    s.push_str("        !self._additional.is_empty()\n");
    s.push_str("    }\n");
    s.push_str("}\n");

    // std::fmt::Display: forward to compact BO4E German JSON.
    // Gated on `json` because it requires serde_json (not just serde).
    // Allows ergonomic use in log messages and `format!("{val}")` contexts.
    s.push_str("\n#[cfg(feature = \"json\")]\n");
    s.push_str(&format!("impl std::fmt::Display for {name} {{\n"));
    s.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    s.push_str("        match serde_json::to_string(self) {\n");
    s.push_str("            Ok(json) => f.write_str(&json),\n");
    s.push_str(&format!(
        "            Err(e) => write!(f, \"<{name}: serialization error: {{e}}>\"),\n"
    ));
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n");
}

fn emit_field(s: &mut String, field: &Field) {
    if let Some(doc) = &field.description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("    /// {}\n", line.trim()));
        }
    }

    // Always emit serde(rename) with the canonical BO4E camelCase JSON name so that
    // serialized payloads are byte-compatible with Python/Go/.NET implementations,
    // even when the Rust snake_case name happens to round-trip correctly without it.
    s.push_str(&format!(
        "    #[cfg_attr(feature = \"serde\", serde(rename = \"{}\"))]\n",
        field.name
    ));

    // For Option<T> fields, omit the key entirely when the value is None.
    if field.is_optional {
        s.push_str("    #[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"Option::is_none\"))]\n");
        // Builder: optional fields default to None and accept T directly (no Some() wrapping).
        s.push_str(
            "    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n",
        );
    }

    // garde: dive into identifier newtypes so their custom validators run.
    if matches!(&field.field_type, FieldType::Identifier(_))
        || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::Identifier(_)))
    {
        s.push_str("    #[cfg_attr(feature = \"validate\", garde(dive))]\n");
    }

    // schemars: types without a built-in JsonSchema impl need an explicit override.
    // OffsetDateTime: schemars 1.x has no `time` feature — represent as ISO-8601 string
    // with the `"format": "date-time"` annotation for tooling (OpenAPI generators, etc.).
    // The same schema_with annotation is correct for BOTH the `time::OffsetDateTime`
    // variant (rfc3339 serialized) and the `String` fallback (raw ISO-8601 passthrough).
    let has_offset_datetime = matches!(
        &field.field_type,
        FieldType::Primitive(PrimitiveType::OffsetDateTime)
    ) || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::Primitive(PrimitiveType::OffsetDateTime)));
    if has_offset_datetime {
        let schema_fn = if field.is_optional {
            "crate::schema_helpers::opt_datetime_schema"
        } else {
            "crate::schema_helpers::datetime_schema"
        };
        // schemars 1.x resolves optionality via the synthetic schema_with type, which does
        // not implement `_schemars_private_is_option()`.  Adding `serde(default)` alongside
        // `skip_serializing_if` lets schemars take the `has_skip_serialize_if && has_default`
        // fast path so the field is correctly omitted from the `required` array.
        if field.is_optional {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"schemars\", schemars(schema_with = \"{schema_fn}\"))]\n"
        ));

        // C-01 + serde bug-fix: without `#[serde(with = "time::serde::rfc3339")]`, the
        // `time` crate's default `Serialize` impl produces a tuple `[year,ordinal,…]`
        // instead of an ISO-8601 string — completely wrong for BO4E wire format.
        // The rfc3339 adapter is gated on BOTH `serde` and `time` features; when `time`
        // is absent the field type falls back to `String` (passthrough, no adapter needed).
        let rfc3339_with = if field.is_optional {
            "time::serde::rfc3339::option"
        } else {
            "time::serde::rfc3339"
        };
        s.push_str(&format!(
            "    #[cfg_attr(all(feature = \"serde\", feature = \"time\"), serde(with = \"{rfc3339_with}\"))]\n"
        ));
    }

    let type_str = field_type_to_rust(&field.field_type, field.is_optional);
    // `serde_json::Value` is only available when the `json` feature is active.
    // Emit a cfg-gated pair: primary type with feature, String fallback without.
    //
    // ⚠ Rust attribute scoping: attributes before a field declaration only apply
    // to that ONE field.  When two declarations share a name under mutually
    // exclusive `#[cfg]` guards, the fallback field MUST re-declare its own
    // serde/builder/schemars attributes — they are NOT inherited from the primary.
    // Omitting them causes:
    //   • None values serializing as explicit `null` instead of being skipped
    //   • camelCase rename lost → snake_case keys in JSON output (wire-format break)
    //   • schemars generating wrong schema type for the field
    if matches!(&field.field_type, FieldType::JsonValue)
        || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::JsonValue))
    {
        emit_feature_gated_field(
            s,
            field,
            "json",
            &type_str,
            "    /// Requires the `json` feature for the full `serde_json::Value` representation.\n",
            false,
        );
    } else if has_offset_datetime {
        emit_feature_gated_field(
            s,
            field,
            "time",
            &type_str,
            "    /// Requires the `time` feature for the `time::OffsetDateTime` representation.\n    /// Without `time`, stores the ISO-8601 string value unchanged.\n",
            true,
        );
    } else if matches!(
        &field.field_type,
        FieldType::Primitive(PrimitiveType::Decimal)
    ) || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::Primitive(PrimitiveType::Decimal)))
    {
        emit_feature_gated_field(
            s,
            field,
            "decimal",
            &type_str,
            "    /// Requires the `decimal` feature for the `rust_decimal::Decimal` representation.\n    /// Without `decimal`, stores the decimal string value unchanged.\n",
            false,
        );
    } else {
        s.push_str(&format!("    pub {}: {type_str},\n", field.rust_name));
    }
}

/// Emits a cfg-gated field pair: primary type under `#[cfg(feature = "…")]` and a
/// `String`/`Option<String>` fallback under `#[cfg(not(feature = "…"))]`.
///
/// `feature` — the Cargo feature name (e.g. `"json"`, `"time"`, `"decimal"`).
/// `primary_type` — the fully resolved type string for the feature-gated variant.
/// `fallback_doc` — the doc comment lines (already `    ///`-prefixed) to emit before
///    the fallback declaration so downstream readers know why the type differs.
/// `datetime_schemars` — passed through to `emit_fallback_attrs` for datetime fields.
fn emit_feature_gated_field(
    s: &mut String,
    field: &Field,
    feature: &str,
    primary_type: &str,
    fallback_doc: &str,
    datetime_schemars: bool,
) {
    let fallback_type = if field.is_optional {
        "Option<String>".to_owned()
    } else {
        "String".to_owned()
    };
    s.push_str(&format!("    #[cfg(feature = \"{feature}\")]\n"));
    s.push_str(&format!("    pub {}: {primary_type},\n", field.rust_name));
    s.push_str(fallback_doc);
    emit_fallback_attrs(s, field, datetime_schemars);
    s.push_str(&format!("    #[cfg(not(feature = \"{feature}\"))]\n"));
    s.push_str(&format!("    pub {}: {fallback_type},\n", field.rust_name));
}

/// Emits the serde/schemars/builder attribute stack for a cfg-fallback field.
///
/// In Rust, attributes before a field declaration apply ONLY to that field.
/// When two field declarations share the same name under mutually exclusive
/// `#[cfg]` guards (e.g. the primary `time::OffsetDateTime` field and its
/// `String` fallback), the fallback must re-declare every attribute it needs.
///
/// `datetime_schemars`: set to `true` for the datetime fallback so the JSON
/// Schema retains the `"format": "date-time"` annotation.
fn emit_fallback_attrs(s: &mut String, field: &Field, datetime_schemars: bool) {
    // serde rename: always needed so the JSON key matches the BO4E spec (camelCase).
    s.push_str(&format!(
        "    #[cfg_attr(feature = \"serde\", serde(rename = \"{}\"))]\n",
        field.name
    ));
    if field.is_optional {
        // Skip None values — don't emit explicit `null` keys.
        s.push_str(
            "    #[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"Option::is_none\"))]\n",
        );
        // schemars: `schema_with` bypasses the normal is-Option? detection, so we
        // need both `skip_serializing_if` AND `serde(default)` for schemars to
        // correctly omit this field from the `required` array.
        if datetime_schemars {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        // Builder: optional fields default to None, setter unwraps Some().
        s.push_str(
            "    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n",
        );
    }
    // Datetime fallback: retain the ISO-8601 `"format": "date-time"` annotation
    // in the JSON Schema even when the `time` crate is not enabled.
    if datetime_schemars {
        let schema_fn = if field.is_optional {
            "crate::schema_helpers::opt_datetime_schema"
        } else {
            "crate::schema_helpers::datetime_schema"
        };
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"schemars\", schemars(schema_with = \"{schema_fn}\"))]\n"
        ));
    }
}

fn field_type_to_rust(ft: &FieldType, optional: bool) -> String {
    let inner = match ft {
        FieldType::Identifier(name) => format!("crate::identifiers::{name}"),
        // BO references are always boxed to prevent large-struct stack bloat.
        // `Option<Box<Marktlokation>>` costs 8 bytes; `Option<Marktlokation>` can
        // cost 700+ bytes when the BO has many optional fields.
        FieldType::Bo(name) => format!("Box<{}>", name.to_upper_camel_case()),
        FieldType::Com(name) => name.to_upper_camel_case(),
        FieldType::BoEnum(name) => name.to_upper_camel_case(),
        FieldType::Primitive(p) => primitive_to_rust(p).to_owned(),
        FieldType::Array(inner) => format!("Vec<{}>", field_type_to_rust(inner, false)),
        FieldType::JsonValue => "serde_json::Value".to_owned(),
    };
    if optional {
        format!("Option<{inner}>")
    } else {
        inner
    }
}

fn primitive_to_rust(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::String => "String",
        PrimitiveType::Bool => "bool",
        PrimitiveType::I64 => "i64",
        // C-01: these are the `time`-feature-active type strings; the emitter emits
        // cfg-conditional fallbacks to `String` when the features are absent.
        PrimitiveType::Decimal => "rust_decimal::Decimal",
        PrimitiveType::OffsetDateTime => "time::OffsetDateTime",
    }
}

// ─── Enum emission ────────────────────────────────────────────────────────────

fn emit_enum(en: &EnumNode) -> Result<String> {
    let mut s = String::from("// @generated — do not edit by hand\n\n");

    s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    s.push_str("#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n");
    s.push_str("#[cfg_attr(feature = \"strum\", derive(strum::Display, strum::EnumString, strum::EnumIter, strum::IntoStaticStr, strum::AsRefStr))]\n");
    s.push_str("#[cfg_attr(feature = \"schemars\", derive(schemars::JsonSchema))]\n");
    s.push_str("#[cfg_attr(feature = \"utoipa\", derive(utoipa::ToSchema))]\n");

    if let Some(doc) = &en.description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("/// {}\n", line.trim()));
        }
    }
    // Prevents downstream exhaustive match arms; complements the `Unknown` catch-all
    // by enforcing compile-time forward-compatibility for external crates (L-07).
    s.push_str("#[non_exhaustive]\n");
    s.push_str(&format!("pub enum {} {{\n", en.name));

    let mut seen_variants: HashSet<String> = HashSet::new();

    for (variant, doc) in &en.variants {
        if let Some(d) = doc {
            for line in clean_description(d).lines() {
                s.push_str(&format!("    /// {}\n", line.trim()));
            }
        }
        let raw_rust = variant.to_upper_camel_case();
        // Rust identifiers cannot start with a digit — prefix with 'V' (Variant).
        let camel = if raw_rust.starts_with(|c: char| c.is_ascii_digit()) {
            format!("V{raw_rust}")
        } else {
            raw_rust
        };
        // Deduplicate: when to_upper_camel_case() collapses two distinct JSON
        // values to the same Rust identifier (e.g. "G2_5" and "G25" → "G25"),
        // fall back to the JSON key sanitized as a valid Rust identifier.
        let rust_variant = if seen_variants.contains(&camel) {
            // Replace every non-alphanumeric character with underscore.
            let sanitized: String = variant
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect();
            // Capitalise first char so it still reads as a PascalCase variant.
            let mut unique = String::new();
            let mut first = true;
            for part in sanitized.split('_').filter(|p| !p.is_empty()) {
                if first {
                    unique.push_str(&part[..1].to_uppercase());
                    unique.push_str(&part[1..]);
                    first = false;
                } else {
                    unique.push_str(&part[..1].to_uppercase());
                    unique.push_str(&part[1..]);
                }
            }
            if unique.starts_with(|c: char| c.is_ascii_digit()) {
                format!("V{unique}")
            } else {
                unique
            }
        } else {
            camel
        };
        seen_variants.insert(rust_variant.clone());
        // Always emit serde(rename) so the serialized value is the canonical JSON string.
        // H-08: also emit strum(serialize) so strum::Display / AsRef / EnumString
        // produce the same canonical string as serde — not the Rust variant name.
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(rename = \"{variant}\"))]\n"
        ));
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"strum\", strum(serialize = \"{variant}\"))]\n"
        ));
        s.push_str(&format!("    {rust_variant},\n"));
    }

    // Catch-all variant for forward-compatibility: unknown values from future schema
    // releases deserialize to `Unknown` instead of causing a panic / deserialization error.
    s.push_str("    /// Unknown or future variant — produced when deserializing a value\n");
    s.push_str("    /// that is not yet known to this version of the library.\n");
    // serde(other) catches all unrecognised strings on deserialization.
    // serde(rename) ensures the variant serializes as "UNKNOWN" (BO4E SCREAMING_SNAKE_CASE)
    // rather than the Rust identifier "Unknown".  Both attributes can coexist: `other`
    // is deserialization-only and `rename` applies to serialization.
    s.push_str("    #[cfg_attr(feature = \"serde\", serde(other, rename = \"UNKNOWN\"))]\n");
    // Use \"UNKNOWN\" so strum::Display is consistent with the SCREAMING_SNAKE_CASE
    // convention of all other variants (L-01 fix: \"Unknown\" was the only variant
    // that did not match the BO4E SCREAMING_SNAKE_CASE serialization convention).
    s.push_str("    #[cfg_attr(feature = \"strum\", strum(serialize = \"UNKNOWN\"))]\n");
    s.push_str("    Unknown,\n");

    s.push_str("}\n");

    // iter_known() — gated on `strum` (requires `EnumIter`).  Returns only
    // schema-defined variants, excluding the `Unknown` catch-all.
    let enum_name = &en.name;
    s.push_str(&format!(
        r#"
impl {enum_name} {{
    /// Returns an iterator over all **known** variants of `{enum_name}`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`{enum_name}::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in {enum_name}::iter_known() {{
    ///     println!("{{v}}");
    /// }}
    /// ```
    #[cfg(feature = "strum")]
    pub fn iter_known() -> impl Iterator<Item = Self> {{
        use strum::IntoEnumIterator as _;
        Self::iter().filter(|v| !matches!(v, Self::Unknown))
    }}
}}
"#
    ));

    // SQLx impls for PostgreSQL — gated on both `sqlx` and `json` features so that
    // serde_json is guaranteed to be available for encode/decode.
    let enum_name = &en.name;
    s.push_str(&format!(
        r#"
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for {enum_name} {{
    fn type_info() -> sqlx::postgres::PgTypeInfo {{
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }}
}}

/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for {enum_name} {{
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {{
        let s: &str = self.as_ref();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }}
}}
/// Fallback when `strum` is not active: serialize via `serde_json`.
#[cfg(all(feature = "sqlx", feature = "json", not(feature = "strum")))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for {enum_name} {{
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {{
        let s = serde_json::to_value(self)?
            .as_str()
            .ok_or("enum variant did not serialize to a JSON string")?
            .to_owned();
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }}
}}

#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for {enum_name} {{
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {{
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }}
}}
"#
    ));

    // Proptest Arbitrary impl — uses strum::EnumIter to generate any known variant.
    // Gated on test mode and `strum` feature (for EnumIter).
    s.push_str(&format!(
        r#"
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for {enum_name} {{
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {{
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }}
}}
"#
    ));

    format_source(s)
}

/// Returns the fully-qualified path to a cross-field validator function, if any.
///
/// The returned path is used in `#[garde(custom(...))]` on the struct.
/// `ver` is the module version prefix, e.g. `"v202501"`.
fn cross_field_validator(name: &str, ver: &str) -> Option<String> {
    match name {
        "Marktlokation" => Some(format!("crate::validation::{ver}::validate_marktlokation")),
        "Messlokation" => Some(format!("crate::validation::{ver}::validate_messlokation")),
        "Vertrag" => Some(format!("crate::validation::{ver}::validate_vertrag_dates")),
        "Bilanzierung" => Some(format!(
            "crate::validation::{ver}::validate_bilanzierung_dates"
        )),
        "Rechnung" => Some(format!(
            "crate::validation::{ver}::validate_rechnung_arithmetic"
        )),
        "Zeitraum" => Some(format!("crate::validation::{ver}::validate_zeitraum")),
        "Kostenposition" => Some(format!(
            "crate::validation::{ver}::validate_kostenposition_arithmetic"
        )),
        _ => None,
    }
}

// ─── RST → Markdown doc cleanup ──────────────────────────────────────────────

/// Strips RST directives from a BO4E JSON Schema description and converts the
/// remaining content to Markdown suitable for rustdoc.
///
/// Transformations applied:
/// - `.. raw:: html` blocks (and their indented content) are removed entirely.
/// - `.. image::` directives are removed.
/// - `.. HINT::` converts the following RST hyperlink to `> **Note:** [text](url)`.
/// - Standalone RST hyperlinks `` `text <url>`_ `` are converted to `[text](url)`.
/// - Trailing blank lines are trimmed.
fn clean_description(desc: &str) -> String {
    let mut output: Vec<String> = Vec::new();
    let mut skip_directive = false;
    let mut hint_next = false;

    for line in desc.lines() {
        let trimmed = line.trim();

        // Directive block start: skip until a non-indented/non-empty line.
        if trimmed.starts_with(".. raw::") || trimmed.starts_with(".. image::") {
            skip_directive = true;
            continue;
        }

        // HINT directive: next non-empty, indented line is the RST link to convert.
        if trimmed == ".. HINT::" {
            hint_next = true;
            skip_directive = true;
            continue;
        }

        // While inside a directive block, skip indented / blank lines.
        if skip_directive {
            if trimmed.is_empty() {
                continue;
            }
            // Non-empty, non-indented: the block ends here.
            if !line.starts_with(' ') && !line.starts_with('\t') {
                skip_directive = false;
            } else {
                // Indented content of a directive.
                if hint_next && trimmed.starts_with('`') && trimmed.ends_with("`_") {
                    // Convert RST hyperlink to Markdown note.
                    if let Some(md) = rst_link_to_markdown(trimmed) {
                        output.push(format!("> **Note:** {md}"));
                    }
                    hint_next = false;
                }
                continue;
            }
        }

        hint_next = false;
        // Convert any inline RST hyperlinks on regular lines.
        let converted = convert_rst_links(trimmed);
        // Wrap bare URLs (not already inside `<...>` or `(...)`) so rustdoc creates hyperlinks.
        let converted = wrap_bare_urls(&converted);
        // Indent bare `= [...]` continuation lines that follow numbered list items
        // (schema descriptions sometimes use this formatting without indentation,
        // which triggers clippy::doc_lazy_continuation).
        let converted = if converted.starts_with("= [") || converted.starts_with("= (") {
            format!("   {converted}")
        } else {
            converted
        };
        output.push(converted);
    }

    // Trim trailing blank lines.
    while output.last().is_some_and(|l: &String| l.trim().is_empty()) {
        output.pop();
    }

    output.join("\n")
}

/// Wraps bare `http://` / `https://` URLs that are not already inside `<...>`,
/// `(...)`, or `[...](...)` Markdown link syntax so that rustdoc renders them
/// as clickable hyperlinks.  Only wraps URLs that are immediately preceded by a
/// space, `(`, or start-of-string and followed by a space, `)`, or end-of-string.
fn wrap_bare_urls(line: &str) -> String {
    // Fast path: no URL scheme present.
    if !line.contains("http://") && !line.contains("https://") {
        return line.to_owned();
    }
    let mut result = String::with_capacity(line.len() + 8);
    let mut chars = line.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        // Check if we're at the start of a URL.
        let rest = &line[i..];
        if rest.starts_with("http://") || rest.starts_with("https://") {
            // Look at what precedes: must be a boundary character (space, (, or start).
            let preceded_by_boundary = i == 0 || {
                let prev: char = line[..i].chars().next_back().unwrap();
                prev == ' ' || prev == '(' || prev == '\t'
            };
            if preceded_by_boundary {
                // Find the end of the URL: stop at whitespace or closing ) / >
                let url_end = rest
                    .find(|c: char| c.is_whitespace() || c == ')' || c == '>')
                    .unwrap_or(rest.len());
                let url = &rest[..url_end];
                // Avoid double-wrapping (already in <...>) or Markdown link targets.
                let already_wrapped = i > 0 && line[..i].ends_with('<');
                let in_md_link = i > 0 && line[..i].ends_with('(');
                if already_wrapped || in_md_link {
                    result.push(c);
                } else {
                    result.push('<');
                    result.push_str(url);
                    result.push('>');
                    // Advance past the URL characters we've already emitted.
                    for _ in 1..url.len() {
                        chars.next();
                    }
                }
                continue;
            }
        }
        result.push(c);
    }
    result
}

/// Converts a RST hyperlink `` `text <url>`_ `` to a Markdown link `[text](url)`.
/// Returns `None` if the pattern is not recognised.
fn rst_link_to_markdown(rst: &str) -> Option<String> {
    // Pattern: `text <url>`_
    let inner = rst.strip_prefix('`')?.strip_suffix("`_")?;
    let angle_open = inner.rfind(" <")?;
    let angle_close = inner.rfind('>')?;
    if angle_close <= angle_open {
        return None;
    }
    let text = inner[..angle_open].trim();
    let url = inner[angle_open + 2..angle_close].trim();
    Some(format!("[{text}]({url})"))
}

/// Converts all inline RST hyperlinks on a single line to Markdown links.
fn convert_rst_links(line: &str) -> String {
    // Simple scan: find `` `...`_ `` patterns and replace them.
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;
    while let Some(start) = remaining.find('`') {
        result.push_str(&remaining[..start]);
        remaining = &remaining[start..];
        // Find matching closing `_
        if let Some(end) = remaining[1..].find("`_") {
            let rst_span = &remaining[..end + 3]; // includes closing `_
            if let Some(md) = rst_link_to_markdown(rst_span) {
                result.push_str(&md);
            } else {
                result.push_str(rst_span);
            }
            remaining = &remaining[end + 3..];
        } else {
            // No matching close — emit as-is.
            result.push_str(remaining);
            remaining = "";
        }
    }
    result.push_str(remaining);
    result
}

// ─── Formatting ───────────────────────────────────────────────────────────────

/// Parses the generated Rust source with `syn` and formats it with `prettyplease`.
pub fn format_source(raw: String) -> Result<String> {
    let file = syn::parse_file(&raw)
        .map_err(|e| anyhow::anyhow!("syn parse error: {e}\n--- source ---\n{raw}"))?;
    Ok(prettyplease::unparse(&file))
}

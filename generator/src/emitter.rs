use anyhow::Result;
use heck::ToUpperCamelCase;
use std::collections::{BTreeMap, HashSet};

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
    s.push_str("/// Deserialization requires the `json` feature: the payload is buffered once,\n");
    s.push_str("/// the `\"_typ\"` discriminant is read from it, and the buffer is then\n");
    s.push_str("/// dispatched to the concrete type.  Serialization requires only `serde`.\n");
    s.push_str("///\n");
    s.push_str("/// # Performance\n");
    s.push_str("///\n");
    s.push_str("/// Because the concrete type is not known until `\"_typ\"` has been read, this\n");
    s.push_str("/// buffers an intermediate `serde_json::Value`.  Deserializing a concrete BO\n");
    s.push_str("/// type directly skips that step, so prefer it on hot paths where the type is\n");
    s.push_str("/// known ahead of time.\n");
    s.push_str("///\n");
    s.push_str("/// # Example\n");
    s.push_str("/// ```\n");
    s.push_str("/// # #[cfg(feature = \"json\")] {\n");
    s.push_str("/// use rubo4e::current::AnyBo;\n");
    s.push_str("///\n");
    s.push_str("/// let json = r#\"{\"_typ\":\"MARKTLOKATION\",\"marktlokationsId\":\"51238696781\"}\"#;\n");
    s.push_str("/// let bo: AnyBo = serde_json::from_str(json).unwrap();\n");
    s.push_str("///\n");
    s.push_str(
        "/// let AnyBo::Marktlokation(malo) = bo else { panic!(\"expected a Marktlokation\") };\n",
    );
    s.push_str(
        "/// assert_eq!(malo.marktlokations_id.as_ref().map(|id| id.as_ref()), Some(\"51238696781\"));\n",
    );
    s.push_str("/// # }\n");
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

    // ── Deserialize — buffer through the *incoming* deserializer, then dispatch.
    //
    // This must not re-parse the input with `serde_json::from_str`.  Doing so
    // discards whatever deserializer the caller handed us, and the crate wraps
    // that deserializer with the two things `AnyBo` most needs:
    //
    //   * the snake_case ↔ German key transform — bypassing it made
    //     `from_json_snake_case` return `Ok` with every typed field empty and the
    //     values diverted into `_additional` (silent data loss);
    //   * the nesting-depth limiter — bypassing it made
    //     `from_json_german_hardened`'s `max_nesting_depth` silently unenforced,
    //     since a `RawValue` capture is only one level deep to the wrapper.
    //
    // Buffering into a `serde_json::Value` costs an intermediate tree, which the
    // old `Box<RawValue>` capture avoided.  That is the price of routing through
    // the caller's deserializer, and callers on a hot path should deserialize the
    // concrete BO type rather than `AnyBo`.
    s.push_str("#[cfg(all(feature = \"serde\", feature = \"json\"))]\n");
    s.push_str("impl<'de> serde::Deserialize<'de> for AnyBo {\n");
    s.push_str(
        "    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\n",
    );
    s.push_str("        // Buffer via `d` so any wrapping deserializer (key transform,\n");
    s.push_str("        // depth limit) is applied exactly once, here.\n");
    s.push_str("        let value = serde_json::Value::deserialize(d)?;\n");
    s.push_str("        let typ_str = value\n");
    s.push_str("            .get(\"_typ\")\n");
    s.push_str("            .and_then(serde_json::Value::as_str)\n");
    s.push_str("            .unwrap_or(\"\");\n");
    s.push_str("        match typ_str {\n");
    for name in bo_names {
        let typ_key = name.to_ascii_uppercase();
        s.push_str(&format!(
            "            \"{typ_key}\" => serde_json::from_value::<{name}>(value)\n"
        ));
        s.push_str(&format!(
            "                .map(|v| AnyBo::{name}(Box::new(v)))\n"
        ));
        s.push_str("                .map_err(serde::de::Error::custom),\n");
    }
    s.push_str("            _ => Ok(AnyBo::Unknown {\n");
    s.push_str("                typ: typ_str.to_owned(),\n");
    s.push_str("                data: value,\n");
    s.push_str("            }),\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n\n");

    // ── Bo4eJsonExt — gives AnyBo all the from_json_german / to_json_german API
    s.push_str("#[cfg(feature = \"json\")]\n");
    s.push_str("impl crate::json::sealed::Sealed for AnyBo {}\n");
    s.push_str("#[cfg(feature = \"json\")]\n");
    s.push_str("impl crate::json::Bo4eJsonExt for AnyBo {}\n\n");

    // ── Bo4eStrict — delegate to the inner BO; an unresolved `_typ` is itself
    // an out-of-schema value and is reported at the `_typ` path.
    s.push_str("#[cfg(feature = \"versioned\")]\n");
    s.push_str("impl crate::Bo4eStrict for AnyBo {\n");
    s.push_str("    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {\n");
    s.push_str("        match self {\n");
    for name in bo_names {
        s.push_str(&format!(
            "            AnyBo::{name}(v) => crate::Bo4eStrict::collect_unknown_enums(&**v, path, out),\n"
        ));
    }
    s.push_str("            #[cfg(feature = \"json\")]\n");
    s.push_str(
        "            AnyBo::Unknown { .. } => out.push(crate::strict::field_path(path, \"_typ\")),\n",
    );
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n");

    s
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Emits the Rust source file for a single schema node.
/// Returns `(filename, source_code)`.
///
/// `schema_version` is the full schema version tag, e.g. `"v202607.0.0"`, used to
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

    // ── Bo4eObject sealed-trait impls ─────────────────────────────────────────
    // Implement the sealing supertrait for every BO type so they satisfy the
    // `Bo4eObject: bo4e_object_sealed::Sealed` bound.  The whole `generated`
    // module is already gated on `#[cfg(feature = "versioned")]` in lib.rs, so
    // a single cfg gate here covers the entire block cleanly.
    if !bo_names.is_empty() {
        s.push_str(
            "// ── Bo4eObject sealed-trait impls ──────────────────────────────────────────\n",
        );
        s.push_str("// These implement the sealing supertrait for all BO types that carry\n");
        s.push_str(
            "// `impl Bo4eObject for Type`.  External crates cannot implement this trait.\n",
        );
        s.push_str("#[cfg(feature = \"versioned\")]\n");
        s.push_str("const _: () = {\n");
        for bo_name in &bo_names {
            s.push_str(&format!(
                "    impl crate::bo4e_object_sealed::Sealed for {bo_name} {{}}\n"
            ));
        }
        s.push_str("};\n");
    }

    format_source(s)
}

// ─── Wire-key ↔ snake_case map emission ──────────────────────────────────────

/// Emits `src/generated/key_map.rs` — the exact, bidirectional mapping between
/// BO4E wire property names (German camelCase) and the Rust snake_case field
/// names the generator derives from them.
///
/// # Why a table instead of a heuristic
///
/// `to_json_snake_case` / `from_json_snake_case` need to convert keys in both
/// directions. Deriving the reverse direction with a heuristic is impossible to
/// get right: `hoechstpreis_ht` could come from `hoechstpreisHt` or
/// `hoechstpreisHT`, and `a` could come from `a` or `A`. BO4E uses all three
/// forms (`Sigmoidparameter.A`, `Tarifberechnungsparameter.hoechstpreisHT`,
/// `PreisblattKonzessionsabgabe.kundengruppeKA`), so a heuristic silently drops
/// those fields into the extension-data bag on a snake_case round-trip.
///
/// The generator already knows both names for every field, so it emits the
/// mapping directly. Lookups are exact, allocation-free, and cannot drift from
/// the generated structs.
///
/// # What is excluded
///
/// - Pairs whose two sides are identical — the lookup falls through to the
///   identity, so storing them would only bloat the table.
/// - Underscore-prefixed wire keys (`_typ`, `_version`, `_id`). These are BO4E
///   metadata keys that stay verbatim in every output mode, so they must not be
///   rewritten to their Rust field names (`typ`, `version`, `id`).
pub fn emit_key_map(nodes: &[SchemaNode]) -> Result<String> {
    // BTreeMap keeps the emitted arrays sorted, which the runtime binary search
    // relies on and which keeps codegen output stable across platforms.
    let mut wire_to_snake: BTreeMap<&str, &str> = BTreeMap::new();
    for node in nodes {
        let fields = match node {
            SchemaNode::Bo(n) => &n.fields,
            SchemaNode::Com(n) => &n.fields,
            SchemaNode::Enum(_) => continue,
        };
        for f in fields {
            if f.name == f.rust_name || f.name.starts_with('_') {
                continue;
            }
            wire_to_snake.insert(&f.name, &f.rust_name);
        }
    }

    // The reverse direction must be a function too: two distinct wire keys that
    // collapse onto the same snake name would make `from_json_snake_case`
    // ambiguous. Fail generation rather than emit a table that silently picks one.
    let mut snake_to_wire: BTreeMap<&str, &str> = BTreeMap::new();
    for (wire, snake) in &wire_to_snake {
        if let Some(previous) = snake_to_wire.insert(snake, wire) {
            anyhow::bail!(
                "ambiguous snake_case key map: wire names {previous:?} and {wire:?} both map to \
                 {snake:?}; from_json_snake_case could not decide which field to fill"
            );
        }
    }

    let mut s = String::from(
        "// @generated — do not edit by hand.\n\
         // This file is maintained by the code generator (`just generate`).\n\n\
         //! Exact BO4E wire-key ↔ Rust snake_case field-name mapping.\n\
         //!\n\
         //! Used by `crate::json::key_transform` to convert JSON keys for\n\
         //! `to_json_snake_case` / `from_json_snake_case`. Both tables are sorted by\n\
         //! their first element so lookups can binary-search, and every entry is\n\
         //! `&'static str`, so a hit never allocates.\n\
         //!\n\
         //! Only pairs whose two sides differ are listed; any key that is absent maps\n\
         //! to itself. Underscore-prefixed BO4E metadata keys (`_typ`, `_version`,\n\
         //! `_id`) are deliberately absent so they survive verbatim in every mode.\n\n",
    );

    s.push_str("/// BO4E wire (camelCase) → Rust snake_case, sorted by wire name.\n");
    s.push_str("pub(crate) static WIRE_TO_SNAKE: &[(&str, &str)] = &[\n");
    for (wire, snake) in &wire_to_snake {
        s.push_str(&format!("    ({wire:?}, {snake:?}),\n"));
    }
    s.push_str("];\n\n");

    s.push_str("/// Rust snake_case → BO4E wire (camelCase), sorted by snake name.\n");
    s.push_str("pub(crate) static SNAKE_TO_WIRE: &[(&str, &str)] = &[\n");
    for (snake, wire) in &snake_to_wire {
        s.push_str(&format!("    ({snake:?}, {wire:?}),\n"));
    }
    s.push_str("];\n");

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

    // BO types with a `_typ` field get a custom Default impl  so that
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

    emit_struct_derives(
        &mut s,
        emits_custom_default(is_bo, fields),
        has_typ_field,
        name,
        schema_version,
    );

    // Doc comment: strip RST directives (from BO4E Python docs) and convert to Markdown.
    if let Some(doc) = description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("/// {}\n", line));
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
            s.push_str("    #[cfg_attr(feature = \"builder\", builder(default, setter(into)))]\n");
            s.push_str("    pub typ: Option<ComTyp>,\n");
        } else {
            emit_field(&mut s, field, schema_version);
        }
    }

    emit_extension_field(&mut s);
    s.push_str("}\n");

    if emits_custom_default(is_bo, fields) {
        emit_default_impl(&mut s, name, &bo_typ_variant, fields, is_bo, schema_version);
    }

    emit_struct_impls(&mut s, name, is_bo, &bo_typ_variant, schema_version, fields);

    format_source(s)
}

/// Emits the `#[derive(...)]` and `#[cfg_attr(...)]` attribute block for a struct.
/// Whether this struct gets a hand-written `Default` instead of the derive.
///
/// A generated `Default` exists to pre-fill the two BO4E metadata fields the
/// generator knows statically: `_typ` (BO types only) and `_version` (every BO
/// and COM). Both reference implementations stamp them, so a value built in Rust
/// must serialize the same way.
///
/// BO types carrying a required non-metadata field are the one exception: they
/// have no valid empty state, so they get no `Default` at all — the derive would
/// not compile either, since a required field's enum type need not implement
/// `Default`. Their builder still stamps `_version`.
fn emits_custom_default(is_bo: bool, fields: &[Field]) -> bool {
    let has_typ_field = is_bo && fields.iter().any(|f| f.name == "_typ");
    let has_version_field = fields.iter().any(|f| f.name == "_version");
    if has_typ_field && fields.iter().any(|f| f.name != "_typ" && !f.is_optional) {
        return false;
    }
    has_typ_field || has_version_field
}

fn emit_struct_derives(
    s: &mut String,
    custom_default: bool,
    has_typ_field: bool,
    name: &str,
    schema_version: &str,
) {
    // Types with generator-populated metadata omit `Default` here; a handwritten
    // impl is emitted below.  BO types with a required field get neither: see
    // `emits_custom_default`.
    if custom_default || has_typ_field {
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
    // The path is versioned so every emitted schema series gets correct impls.
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
    // Single `_additional` field using a cfg-adaptive type rather than
    // two cfg-gated duplicate declarations.  When `json` is on this is the real
    // `LimitedExtensionMap`; when off it is the ZST stub — same field name, same
    // serde skip/flatten semantics, no cfg branch in struct body.
    s.push_str("    /// Unknown JSON fields captured during deserialization for round-trip preservation.\n");
    s.push_str("    /// `None` when no unknown fields were present (zero heap allocation).\n");
    s.push_str("    #[cfg_attr(feature = \"json\", serde(flatten))]\n");
    s.push_str("    #[cfg_attr(feature = \"json\", serde(skip_serializing_if = \"crate::json::ext_map_is_empty\"))]\n");
    s.push_str("    #[cfg_attr(not(feature = \"json\"), serde(skip))]\n");
    s.push_str("    #[cfg_attr(feature = \"builder\", builder(default, setter(skip)))]\n");
    // #[doc(hidden)] keeps the field out of rustdoc while making it `pub` so that
    // external crates can use functional-update syntax (`..Default::default()`).
    // The leading underscore signals that direct mutation is discouraged.
    s.push_str("    #[doc(hidden)]\n");
    s.push_str("    pub _additional: crate::LimitedExtensionMap,\n");
}

/// Emits a custom `Default` impl for a BO struct that pre-fills `typ` with the correct variant.
/// Emits a `Default` impl that pre-fills the BO4E metadata fields.
///
/// `Default::default()` must produce a value that serializes to the same JSON the
/// Python and Go implementations emit for an equivalently empty object. Both
/// stamp `_version` on every BO and COM, and `_typ` on BOs; leaving either unset
/// makes a Rust-built payload distinguishable from every other implementation's.
///
/// Every other field is `Default::default()`, exactly as the derive would have
/// produced, so this impl compiles wherever the derive did.
fn emit_default_impl(
    s: &mut String,
    name: &str,
    bo_typ_variant: &str,
    fields: &[Field],
    is_bo: bool,
    schema_version: &str,
) {
    let has_typ_field = is_bo && fields.iter().any(|f| f.name == "_typ");

    s.push_str(&format!(
        "\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self {{\n"
    ));
    if has_typ_field {
        s.push_str(&format!(
            "            typ: Some(BoTyp::{bo_typ_variant}),\n"
        ));
    }
    for field in fields {
        if has_typ_field && field.name == "_typ" {
            continue; // already emitted as `typ` above
        }
        if field.name == "_version" {
            s.push_str(&format!(
                "            {}: Some(\"{schema_version}\".to_owned()),\n",
                field.rust_name
            ));
            continue;
        }
        s.push_str(&format!(
            "            {}: Default::default(),\n",
            field.rust_name
        ));
    }
    s.push_str("            _additional: Default::default(),\n");
    s.push_str("        }\n    }\n}\n");
}

/// Emits the recursive `Bo4eStrict` walker for a struct.
///
/// The generated `collect_unknown_enums` descends into every enum, BO, and COM
/// field (through `Option`s and `Vec`s), recording the JSON-path of any enum that
/// decoded to its `Unknown` catch-all.  Scalar / identifier / date / decimal /
/// raw-JSON fields carry no schema enum and are skipped, so the walker body is
/// identical regardless of the `time` / `decimal` / `json` feature set.  The
/// structural `_typ` field is skipped (it is set by construction).
fn emit_strict_struct_impl(s: &mut String, name: &str, fields: &[Field]) {
    let mut stmts: Vec<String> = Vec::new();
    for field in fields {
        if let Some(stmt) = strict_field_stmt(field) {
            stmts.push(stmt);
        }
    }
    s.push_str(&format!("\nimpl crate::Bo4eStrict for {name} {{\n"));
    if stmts.is_empty() {
        // No enum/BO/COM fields to descend into — `path`/`out` would be unused.
        s.push_str("    #[allow(unused_variables)]\n");
        s.push_str("    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {}\n");
    } else {
        s.push_str("    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {\n");
        for stmt in &stmts {
            s.push_str("        ");
            s.push_str(stmt);
            s.push('\n');
        }
        s.push_str("    }\n");
    }
    s.push_str("}\n");
}

/// Returns the recursion statement for one struct field, or `None` when the field
/// carries no descendable schema type (scalar, identifier, date, decimal, JSON).
fn strict_field_stmt(field: &Field) -> Option<String> {
    // `_typ` is a structural discriminant set at construction; never a data enum.
    if field.name == "_typ" {
        return None;
    }
    // Whether this schema type is one the walker can descend into at all.
    fn descendable(ft: &FieldType) -> bool {
        matches!(
            ft,
            FieldType::Bo(_) | FieldType::Com(_) | FieldType::BoEnum(_)
        )
    }
    let json = &field.name; // BO4E wire (camelCase) name for the reported path
    let rust = &field.rust_name; // struct accessor
                                 // `Bo4eStrict::collect_unknown_enums` takes `&self`, and BO references are
                                 // boxed, so each emitted expression must resolve to exactly `&impl Bo4eStrict`:
                                 //   BO field  `Box<T>`  → `&**` ;  COM/enum field `T` → `&` ;
                                 //   `Vec` element from `.iter()` is already `&E`  → `item` (or `&**item` for BO).
    match &field.field_type {
        FieldType::Array(inner) if descendable(inner) => {
            let elem = if matches!(inner.as_ref(), FieldType::Bo(_)) {
                "&**item"
            } else {
                "item"
            };
            let loop_body = format!(
                "let child = crate::strict::field_path(path, \"{json}\"); \
                 for (i, item) in items.iter().enumerate() {{ \
                 crate::Bo4eStrict::collect_unknown_enums({elem}, &crate::strict::index_path(&child, i), out); }}"
            );
            if field.is_optional {
                Some(format!(
                    "if let Some(items) = &self.{rust} {{ {loop_body} }}"
                ))
            } else {
                Some(format!("{{ let items = &self.{rust}; {loop_body} }}"))
            }
        }
        other if descendable(other) => {
            let is_bo = matches!(other, FieldType::Bo(_));
            if field.is_optional {
                // `v` from `if let Some(v) = &self.f` is `&Inner`; a BO is `&Box<T>`.
                let expr = if is_bo { "&**v" } else { "v" };
                Some(format!(
                    "if let Some(v) = &self.{rust} {{ crate::Bo4eStrict::collect_unknown_enums({expr}, &crate::strict::field_path(path, \"{json}\"), out); }}"
                ))
            } else {
                // `self.f` is `Box<T>` (BO) or `T` (COM/enum).
                let expr = if is_bo {
                    format!("&**self.{rust}")
                } else {
                    format!("&self.{rust}")
                };
                Some(format!(
                    "crate::Bo4eStrict::collect_unknown_enums({expr}, &crate::strict::field_path(path, \"{json}\"), out);"
                ))
            }
        }
        _ => None,
    }
}

/// Emits all trait impls for a generated struct: `Bo4eObject`, `Bo4eJsonExt`, `Sealed`,
/// `Bo4eExtensionData`, `Display`, and `Bo4eStrict`.
fn emit_struct_impls(
    s: &mut String,
    name: &str,
    is_bo: bool,
    bo_typ_variant: &str,
    schema_version: &str,
    fields: &[Field],
) {
    // Bo4eObject impl — only BO types carry the BoTyp discriminant.
    // `type BoTyp = BoTyp;` binds the associated type from crate::Bo4eObject to the
    // local version-specific BoTyp enum so the impl compiles and dyn usage works as
    // `dyn Bo4eObject<BoTyp = v202607::BoTyp>`.
    if is_bo {
        // Return the runtime `typ` field value so callers doing dynamic dispatch
        // see the actual discriminant from the payload (e.g. "BUENDELVERTRAG"), not the
        // hardcoded struct name.  `unwrap_or` falls back to the compile-time constant only
        // when the field was explicitly set to `None` after construction.
        s.push_str(&format!("\nimpl Bo4eObject for {name} {{\n    type BoTyp = BoTyp;\n    fn bo_type(&self) -> BoTyp {{\n        self.typ.unwrap_or(BoTyp::{bo_typ_variant})\n    }}\n    fn schema_version(&self) -> &'static str {{\n        \"{schema_version}\"\n    }}\n}}\n"));
    }

    // Sealed marker + Bo4eJsonExt impl — restricts trait to BO4E types only.
    s.push_str(&format!(
        "\n#[cfg(feature = \"json\")]\nimpl crate::json::sealed::Sealed for {name} {{}}\n"
    ));
    s.push_str(&format!(
        "#[cfg(feature = \"json\")]\nimpl crate::json::Bo4eJsonExt for {name} {{}}\n"
    ));

    // Bo4eExtensionData: lazy-init getter returns static empty map for None case.
    s.push_str(&format!(
        "\n#[cfg(feature = \"json\")]\nimpl crate::json::Bo4eExtensionData for {name} {{\n"
    ));
    s.push_str(
        "    fn extension_data(&self) -> &indexmap::IndexMap<String, serde_json::Value> {\n",
    );
    // Use the single crate-level EMPTY_EXTENSION_MAP instead of a per-struct
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

    // Bo4eStrict: recursive out-of-schema (Unknown) enum-value detection.
    emit_strict_struct_impl(s, name, fields);
}

fn emit_field(s: &mut String, field: &Field, schema_version: &str) {
    if let Some(doc) = &field.description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("    /// {}\n", line));
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
        // Builder: `setter(into)` accepts both `T` and `Option<T>` — `T: Into<Option<T>>`
        // via `From<T> for Option<T>`, and `Option<T>: Into<Option<T>>` via identity.
        // This replaces `strip_option` and allows mapping from `Option`-valued sources
        // (e.g. EDIFACT parsing) without verbose `if let Some(v) = opt { b = b.f(v); }`.
        //
        // `_version` defaults to the schema version this module was generated from,
        // matching the Python and Go implementations, which stamp it on every BO and
        // COM.  The setter stays available so a caller re-emitting a payload received
        // under a different series can preserve its provenance.
        if field.name == "_version" {
            s.push_str(&format!(
                "    #[cfg_attr(feature = \"builder\", builder(default = Some(\"{schema_version}\".to_owned()), setter(into)))]\n"
            ));
        } else {
            s.push_str("    #[cfg_attr(feature = \"builder\", builder(default, setter(into)))]\n");
        }
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
    let has_date = matches!(&field.field_type, FieldType::Primitive(PrimitiveType::Date))
        || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::Primitive(PrimitiveType::Date)));
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

        // + serde bug-fix: without `#[serde(with = "time::serde::rfc3339")]`, the
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

    if has_date {
        let schema_fn = if field.is_optional {
            "crate::schema_helpers::opt_date_schema"
        } else {
            "crate::schema_helpers::date_schema"
        };
        if field.is_optional {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"schemars\", schemars(schema_with = \"{schema_fn}\"))]\n"
        ));
        let date_serde_with = if field.is_optional {
            "crate::time_serde::opt_date_serde"
        } else {
            "crate::time_serde::date_serde"
        };
        s.push_str(&format!(
            "    #[cfg_attr(all(feature = \"serde\", feature = \"time\"), serde(with = \"{date_serde_with}\"))]\n"
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
            None,
        );
    } else if has_offset_datetime {
        emit_feature_gated_field(
            s,
            field,
            "time",
            &type_str,
            "    /// Requires the `time` feature for the `time::OffsetDateTime` representation.\n    /// Without `time`, stores the ISO-8601 string value unchanged.\n",
            Some(("crate::schema_helpers::opt_datetime_schema", "crate::schema_helpers::datetime_schema")),
        );
    } else if has_date {
        emit_feature_gated_field(
            s,
            field,
            "time",
            &type_str,
            "    /// Requires the `time` feature for the `time::Date` representation.\n    /// Without `time`, stores the ISO 8601 date string (`YYYY-MM-DD`) unchanged.\n",
            Some(("crate::schema_helpers::opt_date_schema", "crate::schema_helpers::date_schema")),
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
            None,
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
/// `fallback_schema_fns` — when `Some((opt_fn, req_fn))`, `emit_fallback_attrs` emits
///    `schemars(schema_with)` on the fallback field using the correct function path.
///    Pass `None` for types that need no special schemars treatment (Decimal, JsonValue).
fn emit_feature_gated_field(
    s: &mut String,
    field: &Field,
    feature: &str,
    primary_type: &str,
    fallback_doc: &str,
    fallback_schema_fns: Option<(&'static str, &'static str)>,
) {
    let fallback_type = if field.is_optional {
        "Option<String>".to_owned()
    } else {
        "String".to_owned()
    };
    s.push_str(&format!("    #[cfg(feature = \"{feature}\")]\n"));
    s.push_str(&format!("    pub {}: {primary_type},\n", field.rust_name));
    s.push_str(fallback_doc);
    emit_fallback_attrs(s, field, fallback_schema_fns);
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
/// `fallback_schema_fns`: when `Some((opt_fn, req_fn))`, emits
/// `schemars(schema_with)` on the fallback field so JSON Schema annotations
/// (e.g. `"format": "date-time"` or `"format": "date"`) are preserved even
/// when the native time type is absent.
fn emit_fallback_attrs(
    s: &mut String,
    field: &Field,
    fallback_schema_fns: Option<(&'static str, &'static str)>,
) {
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
        if fallback_schema_fns.is_some() {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        // Builder: same `setter(into)` semantics as primary fields — accepts `T` or `Option<T>`.
        s.push_str("    #[cfg_attr(feature = \"builder\", builder(default, setter(into)))]\n");
    }
    // Retain the JSON Schema format annotation on the fallback field so that schemars
    // produces the correct `"format"` annotation even when the native type is absent.
    if let Some((opt_fn, req_fn)) = fallback_schema_fns {
        let schema_fn = if field.is_optional { opt_fn } else { req_fn };
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
        // these are the `time`-feature-active type strings; the emitter emits
        // cfg-conditional fallbacks to `String` when the features are absent.
        PrimitiveType::Decimal => "rust_decimal::Decimal",
        PrimitiveType::OffsetDateTime => "time::OffsetDateTime",
        PrimitiveType::Date => "time::Date",
    }
}

// ─── Enum emission ────────────────────────────────────────────────────────────

fn emit_enum(en: &EnumNode) -> Result<String> {
    let mut s = String::from("// @generated — do not edit by hand\n\n");

    s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    s.push_str("#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n");
    // `Display` and `AsRef<str>` are emitted as always-on hand-written impls below
    // (via `as_wire`), so they are NOT derived from strum here — deriving both would
    // collide.  strum remains optional for `FromStr`, iteration, and `&'static str`.
    s.push_str("#[cfg_attr(feature = \"strum\", derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr))]\n");
    s.push_str("#[cfg_attr(feature = \"schemars\", derive(schemars::JsonSchema))]\n");
    s.push_str("#[cfg_attr(feature = \"utoipa\", derive(utoipa::ToSchema))]\n");

    if let Some(doc) = &en.description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("/// {}\n", line));
        }
    }
    // Curated type-level provenance / interop notes for enums whose real-world
    // usage carries a caveat not captured by the terse schema `description`
    // (codelist provenance, upstream gaps, forward-compat wire strings).
    if let Some(note) = enum_type_note(&en.name) {
        s.push_str("///\n");
        for line in note.lines() {
            if line.is_empty() {
                s.push_str("///\n");
            } else {
                s.push_str(&format!("/// {line}\n"));
            }
        }
    }
    // Prevents downstream exhaustive match arms; complements the `Unknown` catch-all
    // by enforcing compile-time forward-compatibility for external crates.
    s.push_str("#[non_exhaustive]\n");
    s.push_str(&format!("pub enum {} {{\n", en.name));

    let mut seen_variants: HashSet<String> = HashSet::new();
    // Collected `(rust_variant_ident, wire_string)` pairs — drives the generated
    // `VARIANTS`, `as_wire`, and `from_wire` members below.
    let mut variant_pairs: Vec<(String, String)> = Vec::new();

    for (variant, doc) in &en.variants {
        if let Some(d) = doc {
            for line in clean_description(d).lines() {
                s.push_str(&format!("    /// {}\n", line));
            }
        }
        // Curated per-variant interop note (e.g. the cross-BO "Messsystem" spelling
        // discrepancy) rendered right where a developer selecting the variant sees it.
        if let Some(note) = enum_variant_note(&en.name, variant) {
            for line in note.lines() {
                s.push_str(&format!("    /// {line}\n"));
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
        variant_pairs.push((rust_variant.clone(), variant.clone()));
        // Always emit serde(rename) so the serialized value is the canonical JSON string.
        // Also emit strum(serialize) so strum::Display / AsRef / EnumString
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
    // convention of all other variants (\"Unknown\" was the only variant
    // that did not match the BO4E SCREAMING_SNAKE_CASE serialization convention).
    s.push_str("    #[cfg_attr(feature = \"strum\", strum(serialize = \"UNKNOWN\"))]\n");
    s.push_str("    Unknown,\n");

    s.push_str("}\n");

    // Feature-independent introspection & strict-parsing surface.  Emitted for
    // every enum without requiring `strum`, and mirrored by the `Bo4eEnum` trait
    // impl below for generic use.  Provides strict `from_wire` and stable
    // `VARIANTS` / `COUNT` for drift-guarding SQL CHECK lists.
    let enum_name = &en.name;
    // `Self::A, Self::B, …` — known variants only, in schema declaration order.
    let variants_list = variant_pairs
        .iter()
        .map(|(rust, _)| format!("Self::{rust}"))
        .collect::<Vec<_>>()
        .join(", ");
    // `Self::A => "WIRE_A",` arms for the canonical wire string.
    let as_wire_arms = variant_pairs
        .iter()
        .map(|(rust, wire)| format!("            Self::{rust} => \"{wire}\","))
        .collect::<Vec<_>>()
        .join("\n");
    // `"WIRE_A" => Ok(Self::A),` arms for strict parsing.
    let from_wire_arms = variant_pairs
        .iter()
        .map(|(rust, wire)| format!("            \"{wire}\" => Ok(Self::{rust}),"))
        .collect::<Vec<_>>()
        .join("\n");
    // A real variant to anchor the `from_wire` doctest, so the example asserts an
    // actual wire→variant mapping rather than only the rejection path.  Every BO4E
    // enum has at least one schema variant, but fall back gracefully if that ever
    // stops holding.
    let from_wire_positive = variant_pairs
        .first()
        .map(|(rust, wire)| {
            format!(
                "/// assert_eq!({enum_name}::from_wire(\"{wire}\"), Ok({enum_name}::{rust}));\n    "
            )
        })
        .unwrap_or_default();
    s.push_str(&format!(
        r#"
impl {enum_name} {{
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`{enum_name}::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[{variants_list}];

    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`{enum_name}::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();

    /// Returns an iterator over all **known** variants of `{enum_name}`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`{enum_name}::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::{enum_name};
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!({enum_name}::iter_known().count(), {enum_name}::COUNT);
    /// assert!({enum_name}::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {{
        Self::VARIANTS.iter().copied()
    }}

    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`{enum_name}::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {{
        match self {{
{as_wire_arms}
            Self::Unknown => "UNKNOWN",
        }}
    }}

    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`{enum_name}::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::{enum_name};
    /// {from_wire_positive}// Out-of-schema values are rejected rather than degraded:
    /// assert!({enum_name}::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!({enum_name}::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {{
        match s {{
{from_wire_arms}
            other => Err(crate::error::UnknownVariant::new(other)),
        }}
    }}

    /// Returns `true` if this value is the forward-compatibility
    /// [`{enum_name}::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {{
        matches!(self, Self::Unknown)
    }}

    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {{
        !self.is_unknown()
    }}
}}

// `Display` / `AsRef<str>` — always available (no `strum` needed), yielding the
// canonical BO4E wire string.  This gives non-`strum` builds ergonomic printing
// and `&str` access, and lets the sqlx encode path avoid a `serde_json` round-trip.
impl std::fmt::Display for {enum_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        f.write_str(self.as_wire())
    }}
}}
impl AsRef<str> for {enum_name} {{
    fn as_ref(&self) -> &str {{
        self.as_wire()
    }}
}}

// Uniform generic surface: `Bo4eEnum` forwards to the inherent members above so
// callers can be generic over any BO4E enum (e.g. `fn coverage<T: Bo4eEnum>()`).
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for {enum_name} {{}}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for {enum_name} {{
    const VARIANTS: &'static [Self] = Self::VARIANTS;
    const COUNT: usize = Self::COUNT;
    fn as_wire(&self) -> &'static str {{
        Self::as_wire(self)
    }}
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {{
        Self::from_wire(s)
    }}
    fn is_unknown(&self) -> bool {{
        Self::is_unknown(self)
    }}
}}

// Leaf of the recursive strict walk: an enum reports itself when it holds the
// `Unknown` catch-all (an out-of-schema value produced by a lenient decode).
#[cfg(feature = "versioned")]
impl crate::Bo4eStrict for {enum_name} {{
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {{
        if self.is_unknown() {{
            out.push(path.to_owned());
        }}
    }}
}}
"#
    ));

    // SQLx impls for PostgreSQL.  Both directions go through the `as_wire` /
    // `from_wire` pair, which is available without any optional dependency — so
    // these need only the `sqlx` feature, not `json`.
    let enum_name = &en.name;
    s.push_str(&format!(
        r#"
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for {enum_name} {{
    fn type_info() -> sqlx::postgres::PgTypeInfo {{
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }}
}}

/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for {enum_name} {{
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {{
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }}
}}

/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`{enum_name}::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`{enum_name}::from_wire`] on a `String` column, or check
/// [`{enum_name}::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for {enum_name} {{
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {{
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }}
}}
"#
    ));

    // Proptest Arbitrary impl — samples from the known-variant table.  No longer
    // requires `strum`, since `VARIANTS` is now feature-independent.
    s.push_str(&format!(
        r#"
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for {enum_name} {{
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {{
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }}
}}
"#
    ));

    format_source(s)
}

/// Returns the fully-qualified path to a cross-field validator function, if any.
///
/// The returned path is used in `#[garde(custom(...))]` on the struct.
/// `ver` is the module version prefix, e.g. `"v202607"`.
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

// ─── Curated interop / provenance notes ──────────────────────────────────────

/// Returns extra type-level rustdoc for an enum whose real-world use carries a
/// caveat the terse BO4E schema `description` does not capture: codelist
/// provenance, an upstream gap, or a forward-compat wire string.
///
/// Keyed by the UpperCamelCase Rust enum name.  Blank lines are preserved so the
/// note renders as proper Markdown paragraphs in rustdoc.
fn enum_type_note(name: &str) -> Option<&'static str> {
    match name {
        // State the provenance so downstream can decide whether to keep a
        // parse-coverage guard.  BO4E does not tag its enums with a BDEW Codeliste
        // release, so we document the schema tag that is the actual source of truth.
        "BdewArtikelnummer" => Some(
            "# Provenance\n\
             \n\
             The variants are transcribed 1:1 from the `BdewArtikelnummer` enum of the\n\
             pinned BO4E schema release (see the module's schema-version tag, e.g.\n\
             `v202607.0.0`).  BO4E does not annotate this enum with the corresponding\n\
             *BDEW Codeliste der Artikelnummern und Artikel-IDs* release, so treat the\n\
             BO4E schema tag — not a BDEW Codeliste version — as the authoritative\n\
             coverage signal.  New codes arrive only via a schema bump; the per-release\n\
             CHANGELOG records enum additions.  Values absent from this version decode\n\
             to [`BdewArtikelnummer::Unknown`]; use `from_wire` to reject them strictly.",
        ),
        // Gasqualitaet has no H2-blend variant in the current schema.
        "Gasqualitaet" => Some(
            "# Forward compatibility (H2 blends)\n\
             \n\
             As of the current schema this enum models only `H_GAS` / `L_GAS`.  Hydrogen\n\
             blend qualities expected from the 2026–2028 DVGW G 260 / BNetzA wave are not\n\
             yet standardized in BO4E; until they are, such wire values decode to\n\
             [`Gasqualitaet::Unknown`].  Do **not** hard-code a speculative wire string —\n\
             when BO4E adds the variant it will appear here (and in the CHANGELOG) with\n\
             its canonical spelling, and lenient decoding will start resolving it.",
        ),
        // Rechnungstyp has no correction/reversal value.
        "Rechnungstyp" => Some(
            "# Correction / reversal invoices\n\
             \n\
             BO4E does not model a Korrektur/Storno value in this enum.  The sanctioned\n\
             representation is a process label carried as a `ZusatzAttribut` on the\n\
             `Rechnung` (e.g. `rechnungsart = \"KORREKTURRECHNUNG\"`) rather than a\n\
             dedicated `Rechnungstyp` variant.  This is an upstream BO4E modelling gap;\n\
             if a future schema introduces a correction value it will surface here.",
        ),
        _ => None,
    }
}

/// Returns extra rustdoc for a specific `(enum, wire_value)` variant, rendered
/// directly above the variant so it is visible at the point of selection.
///
/// Keyed by the UpperCamelCase enum name and the raw JSON wire value.
fn enum_variant_note(enum_name: &str, wire_value: &str) -> Option<&'static str> {
    match (enum_name, wire_value) {
        // The same real-world "intelligentes Messsystem" concept is spelled with
        // three `s` here but two `s` in `Geraetetyp` — faithful to BO4E v202607
        // upstream, which is internally inconsistent.  Flag both sides so a payload
        // is never built with the wrong spelling for the wrong BO.
        ("Zaehlertyp", "INTELLIGENTES_MESSSYSTEM") => Some(
            "\n**Wire spelling:** `INTELLIGENTES_MESSSYSTEM` (three `s`).  ⚠ BO4E spells the\n\
             *same* iMSys concept differently across BOs: `Geraetetyp::IntelligentesMessystem`\n\
             uses `INTELLIGENTES_MESSYSTEM` (two `s`).  This divergence is upstream, not a\n\
             `rubo4e` transcription error; each BO keeps its own canonical spelling.",
        ),
        ("Geraetetyp", "INTELLIGENTES_MESSYSTEM") => Some(
            "\n**Wire spelling:** `INTELLIGENTES_MESSYSTEM` (two `s`).  ⚠ BO4E spells the\n\
             *same* iMSys concept differently across BOs: `Zaehlertyp::IntelligentesMesssystem`\n\
             uses `INTELLIGENTES_MESSSYSTEM` (three `s`).  This divergence is upstream, not a\n\
             `rubo4e` transcription error; each BO keeps its own canonical spelling.",
        ),
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
        output.push(converted);
    }

    // Post-process: indent list-item continuation lines so rustdoc can parse the
    // list correctly and clippy::doc_lazy_continuation /
    // doc_list_item_without_indentation don't fire.
    //
    // Rules (Markdown / rustdoc):
    //   Bullet list  (`* ` / `- `)  → continuation indented 2 spaces
    //   Numbered list (`N. `)        → continuation indented 3 spaces (len of "1. ")
    //
    // A line is a "continuation" when it is non-empty, not blank, not a new list
    // item, and immediately follows a list-item line.
    let mut result: Vec<String> = Vec::with_capacity(output.len());
    // None = not in list; Some(n) = in list, continuation indent = n spaces
    let mut list_indent: Option<usize> = None;
    for line in &output {
        let is_blank = line.trim().is_empty();

        // Detect bullet list item: starts with `* ` or `- `
        let is_bullet = line.starts_with("* ") || line.starts_with("- ");
        // Detect numbered list item: one or more digits followed by `. `
        let is_numbered = {
            let dot_pos = line.find(". ");
            dot_pos.is_some_and(|i| i > 0 && line[..i].chars().all(|c| c.is_ascii_digit()))
        };
        // Already-indented line (from the schema source or a prior pass)
        let already_indented = line.starts_with("  ");

        if is_bullet {
            list_indent = Some(2);
            result.push(line.clone());
        } else if is_numbered {
            list_indent = Some(3);
            result.push(line.clone());
        } else if is_blank {
            list_indent = None;
            result.push(line.clone());
        } else if let Some(indent) = list_indent {
            if already_indented {
                // Already indented enough — keep as-is but stay in list context.
                result.push(line.clone());
            } else {
                // Continuation line — prepend the required indent.
                result.push(format!("{}{line}", " ".repeat(indent)));
            }
        } else {
            result.push(line.clone());
        }
    }

    // Trim trailing blank lines.
    while result.last().is_some_and(|l: &String| l.trim().is_empty()) {
        result.pop();
    }

    result.join("\n")
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

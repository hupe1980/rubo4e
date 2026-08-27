use anyhow::Result;
use heck::ToUpperCamelCase;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::ast::{EnumNode, Field, FieldType, PrimitiveType, SchemaNode, StructKind, StructNode};
use crate::naming::{needs_non_camel_case_allow, screaming_to_camel};

// ─── AnyBo emission ──────────────────────────────────────────────────────────

/// One BO type as `AnyBo` needs it: the Rust struct name and the `_typ` wire
/// string that selects it during dispatch.
struct BoDispatch {
    rust_name: String,
    typ_wire: String,
}

/// Emits the `AnyBo` sum type for a given schema version.
///
/// `bos` must be sorted by `rust_name` and contain every BO type in the version.
fn emit_any_bo(bos: &[BoDispatch]) -> String {
    if bos.is_empty() {
        return String::new();
    }
    let bo_names: Vec<&String> = bos.iter().map(|b| &b.rust_name).collect();

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
    // Same rule as the generated structs: `Eq` and `Hash` together, and only when
    // `json` is off — the `Unknown` variant carries a `serde_json::Value`.
    s.push_str("#[cfg_attr(not(feature = \"json\"), derive(Eq, Hash))]\n");
    s.push_str("#[non_exhaustive]\n");
    s.push_str("pub enum AnyBo {\n");
    for name in &bo_names {
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
    s.push_str("    /// Delegates to the inner type's [`Bo4eTyped::TYP`] for all known\n");
    s.push_str("    /// variants; returns [`BoTyp::Unknown`] for the `Unknown` catch-all.\n");
    s.push_str("    pub fn bo_type(&self) -> BoTyp {\n");
    s.push_str("        match self {\n");
    for name in &bo_names {
        s.push_str(&format!(
            "            AnyBo::{name}(_) => <{name} as Bo4eTyped>::TYP,\n"
        ));
    }
    s.push_str("            #[cfg(feature = \"json\")]\n");
    s.push_str("            AnyBo::Unknown { .. } => BoTyp::Unknown,\n");
    s.push_str("        }\n");
    s.push_str("    }\n\n");

    // The remaining `Bo4eTyped` facts, so `AnyBo` serves the heterogeneous case
    // the associated constants make impossible for a trait object.
    s.push_str("    /// Returns the `_typ` wire string for this BO object.\n");
    s.push_str("    ///\n");
    s.push_str("    /// Total: for the `Unknown` catch-all it is the value the payload\n");
    s.push_str("    /// carried, which is the whole reason that variant keeps it.\n");
    s.push_str("    pub fn typ_wire(&self) -> &str {\n");
    s.push_str("        match self {\n");
    for name in &bo_names {
        s.push_str(&format!(
            "            AnyBo::{name}(_) => <{name} as Bo4eTyped>::TYP_WIRE,\n"
        ));
    }
    s.push_str("            #[cfg(feature = \"json\")]\n");
    s.push_str("            AnyBo::Unknown { typ, .. } => typ,\n");
    s.push_str("        }\n");
    s.push_str("    }\n\n");

    for (accessor, konst, what) in [
        (
            "schema_version",
            "SCHEMA_VERSION",
            "the exact BO4E release these types were generated from (e.g. `\"202607.1.0\"`)",
        ),
        (
            "schema_series",
            "SCHEMA_SERIES",
            "the schema series (e.g. `\"202607\"`) — the right key for version dispatch",
        ),
    ] {
        s.push_str(&format!("    /// Returns {what}.\n"));
        s.push_str("    ///\n");
        s.push_str(
            "    /// `None` for the `Unknown` catch-all: this crate generated no type for\n",
        );
        s.push_str("    /// it, so it has no release to report.\n");
        s.push_str(&format!(
            "    pub fn {accessor}(&self) -> Option<&'static str> {{\n"
        ));
        s.push_str("        match self {\n");
        for name in &bo_names {
            s.push_str(&format!(
                "            AnyBo::{name}(_) => Some(<{name} as Bo4eTyped>::{konst}),\n"
            ));
        }
        s.push_str("            #[cfg(feature = \"json\")]\n");
        s.push_str("            AnyBo::Unknown { .. } => None,\n");
        s.push_str("        }\n");
        s.push_str("    }\n\n");
    }
    s.push_str("}\n\n");

    // ── From<T> for AnyBo ─────────────────────────────────────────────────
    for name in &bo_names {
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
    for name in &bo_names {
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
    for bo in bos {
        let (name, typ_key) = (&bo.rust_name, &bo.typ_wire);
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
    for name in &bo_names {
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
    s.push_str("}\n\n");

    // ── Bo4eExtensions — the same delegation for out-of-schema *fields*.
    // `Unknown` holds raw JSON for a `_typ` no generated type matches, so every
    // key in it is by definition undefined here; reporting each one individually
    // would be noise, and the `_typ` is the finding.
    s.push_str("#[cfg(all(feature = \"json\", feature = \"versioned\"))]\n");
    s.push_str("impl crate::json::Bo4eExtensions for AnyBo {\n");
    s.push_str("    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {\n");
    s.push_str("        match self {\n");
    for name in &bo_names {
        s.push_str(&format!(
            "            AnyBo::{name}(v) => crate::json::Bo4eExtensions::collect_extension_paths(&**v, path, out),\n"
        ));
    }
    s.push_str(
        "            AnyBo::Unknown { .. } => out.push(crate::strict::field_path(path, \"_typ\")),\n",
    );
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n");

    s
}

// ─── Discriminant naming ─────────────────────────────────────────────────────

/// Maps each `BoTyp` / `ComTyp` wire value to the Rust name of the struct it
/// identifies.
///
/// Those two enums are the one place where a SCREAMING_SNAKE_CASE value has a
/// *known* word split: `"PREISBLATTKONZESSIONSABGABE"` is the discriminant of the
/// `PreisblattKonzessionsabgabe` schema in the same release, so the variant can
/// take that name instead of the `Preisblattkonzessionsabgabe` a mechanical
/// conversion gives.
///
/// Values with no corresponding schema (`GESCHAEFTSOBJEKT`,
/// `NETZNUTZUNGSRECHNUNG`, `PREISBLATTUMLAGEN` — discriminants BO4E declares
/// without shipping a type) fall back to [`screaming_to_camel`].
#[derive(Debug, Default)]
pub struct DiscriminantNames {
    bo: BTreeMap<String, String>,
    com: BTreeMap<String, String>,
}

impl DiscriminantNames {
    /// Builds the registry from every struct node in a schema release.
    pub fn from_nodes(nodes: &[SchemaNode]) -> Self {
        let mut names = Self::default();
        for node in nodes.iter().filter_map(SchemaNode::as_struct) {
            let Some(wire) = &node.typ_const else {
                continue;
            };
            let table = match node.kind {
                StructKind::Bo => &mut names.bo,
                StructKind::Com => &mut names.com,
            };
            table.insert(wire.clone(), node.name.to_upper_camel_case());
        }
        names
    }

    /// Returns the Rust variant name for `wire` in the enum called `enum_name`.
    fn variant(&self, enum_name: &str, wire: &str) -> String {
        let table = match enum_name {
            "BoTyp" => Some(&self.bo),
            "ComTyp" => Some(&self.com),
            _ => None,
        };
        table
            .and_then(|t| t.get(wire))
            .cloned()
            .unwrap_or_else(|| screaming_to_camel(wire))
    }
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Emits the Rust source file for a single schema node.
/// Returns `(filename, source_code)`.
///
/// `schema_version` is the full schema version tag, e.g. `"v202607.0.0"`, used to
/// populate [`Bo4eTyped::SCHEMA_VERSION`] on generated types.  The `_typ` and
/// `_version` values written into the JSON come from the schema itself, not from
/// this tag.
pub fn emit_node(
    node: &SchemaNode,
    schema_version: &str,
    names: &DiscriminantNames,
) -> Result<(String, String)> {
    // Normalize the type name to UpperCamelCase so it matches the type references
    // produced by the inference module (which also calls to_upper_camel_case() on
    // $ref-derived names like "BDEWArtikelnummer" → "BdewArtikelnummer").
    let rust_name = node.name().to_upper_camel_case();
    let source = match node {
        SchemaNode::Struct(st) => emit_struct(&rust_name, st, schema_version, names),
        SchemaNode::Enum(en) => {
            let mut en2 = en.clone();
            en2.name = rust_name.clone();
            emit_enum(&en2, names)
        }
    }?;
    let filename = format!("{}.rs", heck::AsSnakeCase(&rust_name));
    Ok((filename, source))
}

/// Emits a `mod.rs` that re-exports every node name in `nodes` and re-exports
/// the crate-level discriminant traits so struct files can implement them.
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
    // ── Discriminant-trait re-exports ─────────────────────────────────────────
    s.push_str(
        "// Re-exported so the struct files can name the traits they implement.\n         pub use crate::{Bo4eComponent, Bo4eObject, Bo4eTyped};\n",
    );

    // ── AnyBo: heterogeneous dispatch enum ───────────────────────────────────
    let bos: Vec<BoDispatch> = sorted_nodes
        .iter()
        .filter_map(|n| n.as_struct())
        .filter(|st| st.kind.is_bo())
        .map(|st| BoDispatch {
            rust_name: st.name.to_upper_camel_case(),
            // Dispatch on the discriminant the schema declares, not on an
            // upper-cased struct name that only happens to agree with it.
            typ_wire: st
                .typ_const
                .clone()
                .unwrap_or_else(|| st.name.to_ascii_uppercase()),
        })
        .collect();
    s.push_str(&emit_any_bo(&bos));

    format_source(s)
}

// ─── Wire-key ↔ snake_case map emission ──────────────────────────────────────

/// Emits `src/generated/key_map.rs` — the exact, bidirectional mapping between
/// BO4E wire property names (German camelCase) and the Rust snake_case field
/// names the generator derives from them.
///
/// A table rather than a heuristic, because the reverse direction has no
/// algorithmic inverse: `hoechstpreis_ht` could come from `hoechstpreisHt` or
/// `hoechstpreisHT`, and `a` from `a` or `A` — and BO4E uses all of those forms.
/// The generator knows both names for every field, so it emits the mapping
/// directly; lookups are exact and allocation-free.
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
        let Some(fields) = node.as_struct().map(|n| &n.fields) else {
            continue;
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

    // Every key the schema *defines*, in both spellings — not just the pairs that
    // differ.  `key_transform` needs it to tell a schema field from extension
    // data: the value under a key that is not in here is a free-form JSON blob
    // whose own keys belong to whoever wrote them, and must not be renamed.
    //
    // The table is keyed on the name alone, because that is all a streaming key
    // transform has: it renames keys as the parser yields them, long before serde
    // decides which struct they belong to.
    //
    // That is enough to protect *extension* data, which is the round-trip promise
    // — an unknown key is unknown under any struct.  It is not enough for a
    // free-form field the schema *does* define: `ZusatzAttribut.wert` holds
    // arbitrary JSON, but `wert` is also `Betrag`'s decimal and `Messwert`'s
    // nested COM, so the name cannot be excluded without breaking the latter.
    // `key_transform`'s module docs state that limitation; the check below fails
    // generation if a release ever adds a free-form field whose name is *not*
    // shared, at which point excluding it becomes both possible and worthwhile.
    let mut known_keys: BTreeSet<&str> = BTreeSet::new();
    let mut free_form: Vec<(&str, &str)> = Vec::new();
    for node in nodes {
        let Some(st) = node.as_struct() else { continue };
        for f in &st.fields {
            if f.field_type == FieldType::JsonValue {
                free_form.push((&st.name, &f.name));
            }
            known_keys.insert(&f.name);
            known_keys.insert(&f.rust_name);
        }
    }
    for (owner, name) in &free_form {
        let shared = nodes
            .iter()
            .filter_map(SchemaNode::as_struct)
            .flat_map(|st| st.fields.iter().map(move |f| (st, f)))
            .any(|(st, f)| {
                f.name == *name && holds_nested_object(&f.field_type) && st.name != *owner
            });
        anyhow::ensure!(
            shared,
            "{owner}.{name} is free-form JSON and no other schema field of that name holds a \
             nested object, so the key transform could now leave its contents alone. Drop \
             {name:?} from KNOWN_FIELD_KEYS and update the limitation note in \
             `src/json/key_transform.rs`."
        );
    }

    let mut s = String::from(
        "// @generated — do not edit by hand.\n\
         // This file is maintained by the code generator (`just generate`).\n\n\
         //! Exact BO4E wire-key ↔ Rust snake_case field-name mapping.\n\
         //!\n\
         //! Used by `crate::json::key_transform` to convert JSON keys for\n\
         //! `to_json_snake_case` / `from_json_snake_case`. Every table is sorted by\n\
         //! its first element so lookups can binary-search, and every entry is\n\
         //! `&'static str`, so a hit never allocates.\n\
         //!\n\
         //! Only pairs whose two sides differ are listed in the two mapping tables;\n\
         //! any key absent from them maps to itself. Underscore-prefixed BO4E\n\
         //! metadata keys (`_typ`, `_version`, `_id`) are deliberately absent so they\n\
         //! survive verbatim in every mode.\n\
         //!\n\
         //! `KNOWN_FIELD_KEYS` is the separate question of whether a key is a schema\n\
         //! field *at all*, which is what stops the transform from descending into\n\
         //! somebody else's JSON.\n\n",
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
    s.push_str("];\n\n");

    s.push_str("/// Every key the BO4E schema defines, in both spellings, sorted.\n");
    s.push_str("///\n");
    s.push_str("/// A key **absent** from this list is extension data (or a free-form\n");
    s.push_str("/// `JsonValue` field), so its value is somebody else's JSON and the key\n");
    s.push_str("/// transform must not descend into it.\n");
    s.push_str("pub(crate) static KNOWN_FIELD_KEYS: &[&str] = &[\n");
    for key in &known_keys {
        s.push_str(&format!("    {key:?},\n"));
    }
    s.push_str("];\n");

    format_source(s)
}

/// Whether a field's type has validation rules of its own, and so should be
/// dived into by `garde`.
///
/// True for the identifier newtypes (each re-runs its constructor's check) and
/// for every nested BO / COM (each carries its own `derive(Validate)`, and
/// possibly a cross-field validator). False for scalars and enums, which have
/// nothing to check. `Array` is transparent — a `Vec<T>` dives if `T` does.
fn field_carries_validation(ft: &FieldType) -> bool {
    match ft {
        FieldType::Identifier(_) | FieldType::Bo(_) | FieldType::Com(_) => true,
        FieldType::Array(inner) => field_carries_validation(inner),
        FieldType::BoEnum(_) | FieldType::Primitive(_) | FieldType::JsonValue => false,
    }
}

/// Whether a value of this type is a JSON object (or an array of them), i.e.
/// whether it has keys the transform would rename.
fn holds_nested_object(ft: &FieldType) -> bool {
    match ft {
        FieldType::Bo(_) | FieldType::Com(_) => true,
        FieldType::Array(inner) => holds_nested_object(inner),
        FieldType::Identifier(_)
        | FieldType::BoEnum(_)
        | FieldType::Primitive(_)
        | FieldType::JsonValue => false,
    }
}

// ─── Struct emission ──────────────────────────────────────────────────────────

/// Collects the set of sibling type names (exported by the version `mod.rs`)
/// that a generated struct file must explicitly import from `super`.
///
/// Returned names are sorted and deduplicated.  The struct's own name is never
/// included — it is being *defined*, not imported.
fn collect_sibling_imports(name: &str, node: &StructNode) -> Vec<String> {
    let mut set = HashSet::new();

    // A struct with a `_typ` needs its discriminant enum, `Bo4eTyped`, and the
    // marker trait for its kind.  `ZusatzAttribut` — the one schema with no
    // `_typ` — needs none of them.
    if node.fields.iter().any(|f| f.name == "_typ") {
        set.insert(node.kind.typ_enum().to_string());
        set.insert("Bo4eTyped".to_string());
        set.insert(
            if node.kind.is_bo() {
                "Bo4eObject"
            } else {
                "Bo4eComponent"
            }
            .to_string(),
        );
    } else if node.kind.is_bo() {
        // A BO always names `BoTyp` in its field definition.
        set.insert("BoTyp".to_string());
    }

    // Walk field types to discover referenced sibling BO/COM/enum names.
    for field in &node.fields {
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

/// Everything the emitter needs to know about a struct's BO4E metadata fields
/// (`_typ` and `_version`), resolved once from the schema.
struct Metadata<'a> {
    /// Rust name of the discriminant enum (`BoTyp` / `ComTyp`), when this struct
    /// has a `_typ` field.
    typ_enum: Option<&'static str>,
    /// The `BoTyp::X` / `ComTyp::X` path this struct's `_typ` is pinned to.
    typ_path: Option<String>,
    /// The raw `_typ` wire value (e.g. `"MARKTLOKATION"`), when it has that field.
    typ_wire: Option<&'a str>,
    /// The `_version` wire value this schema declares, when it has that field.
    version: Option<&'a str>,
}

impl<'a> Metadata<'a> {
    fn resolve(node: &'a StructNode, names: &DiscriminantNames) -> Self {
        let has_typ = node.fields.iter().any(|f| f.name == "_typ");
        let typ_enum = has_typ.then(|| node.kind.typ_enum());
        let typ_path = typ_enum.zip(node.typ_const.as_deref()).map(|(en, wire)| {
            let variant = names.variant(en, wire);
            format!("{en}::{variant}")
        });
        let typ_wire = has_typ.then_some(node.typ_const.as_deref()).flatten();
        let version = node
            .fields
            .iter()
            .any(|f| f.name == "_version")
            .then_some(node.version_default.as_deref())
            .flatten();
        Self {
            typ_enum,
            typ_path,
            typ_wire,
            version,
        }
    }

    /// Whether the emitter writes a hand-rolled `Default` instead of deriving it.
    ///
    /// A generated `Default` pre-fills the two BO4E metadata fields the schema
    /// pins statically, `_typ` and `_version`, which every implementation stamps
    /// on every BO *and* COM.
    ///
    /// A struct carrying a required non-metadata field gets no `Default` at all:
    /// the derive would not compile either, since a required field's type need
    /// not implement `Default`.  Its builder still stamps the metadata.
    fn emits_custom_default(&self, fields: &[Field]) -> bool {
        if self.typ_path.is_none() && self.version.is_none() {
            return false;
        }
        !fields
            .iter()
            .any(|f| !f.name.starts_with('_') && !f.is_optional)
    }
}

fn emit_struct(
    name: &str,
    node: &StructNode,
    schema_version: &str,
    names: &DiscriminantNames,
) -> Result<String> {
    let fields = &node.fields[..];
    let is_bo = node.kind.is_bo();
    let meta = Metadata::resolve(node, names);

    let mut s = String::from("// @generated — do not edit by hand\n\n");
    // Emit explicit imports of all sibling types actually referenced by this struct.
    // Avoids the wildcarded `use super::*` (with its suppressed unused-import warning)
    // and makes cross-references visible to tools (rustdoc, IDEs, cargo check).
    let imports = collect_sibling_imports(name, node);
    if !imports.is_empty() {
        s.push_str(&format!("use super::{{{}}};\n\n", imports.join(", ")));
    }

    emit_struct_derives(&mut s, &meta, fields, name, schema_version);

    // Doc comment: strip RST directives (from BO4E Python docs) and convert to Markdown.
    if let Some(doc) = node.description.as_deref() {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("/// {}\n", line));
        }
    }

    s.push_str(&format!("pub struct {name} {{\n"));

    for field in fields {
        // Replace the raw `_typ` property with the typed discriminant enum.
        if field.name == "_typ" {
            let Some(typ_enum) = meta.typ_enum else {
                continue;
            };
            match &meta.typ_path {
                Some(path) => s.push_str(&format!(
                    "    /// BO4E type discriminant — always `{path}` for this struct.\n"
                )),
                None => s.push_str("    /// BO4E type discriminant for this struct.\n"),
            }
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(rename = \"_typ\"))]\n");
            s.push_str(
                "    #[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"Option::is_none\"))]\n",
            );
            match &meta.typ_path {
                // The discriminant is fixed by the schema, so the builder stamps
                // it and offers no setter — a caller cannot make it disagree.
                Some(path) => s.push_str(&format!(
                    "    #[cfg_attr(feature = \"builder\", builder(default = Some({path}), setter(skip)))]\n"
                )),
                None => s.push_str(
                    "    #[cfg_attr(feature = \"builder\", builder(default, setter(into)))]\n",
                ),
            }
            s.push_str(&format!("    pub typ: Option<{typ_enum}>,\n"));
        } else {
            emit_field(&mut s, field, &meta);
        }
    }

    emit_extension_field(&mut s);
    s.push_str("}\n");

    if meta.emits_custom_default(fields) {
        emit_default_impl(&mut s, name, fields, &meta);
    } else if meta.typ_path.is_some() || meta.version.is_some() {
        // No `Default`, because a required field's type need not have one — so
        // there is no `..Default::default()` either, and building the value by
        // hand means writing out every one of its two dozen optional fields.
        // `new` takes the required ones and defaults the rest.
        emit_required_field_constructor(&mut s, name, fields, &meta);
    }

    emit_struct_impls(&mut s, name, is_bo, &meta, schema_version, fields);

    format_source(s)
}

/// Emits the `#[derive(...)]` and `#[cfg_attr(...)]` attribute block for a struct.
fn emit_struct_derives(
    s: &mut String,
    meta: &Metadata<'_>,
    fields: &[Field],
    name: &str,
    schema_version: &str,
) {
    // Types with schema-pinned metadata omit `Default` here; a handwritten impl
    // is emitted below.  Structs with a required field get neither: see
    // `Metadata::emits_custom_default`.
    let has_metadata = meta.typ_path.is_some() || meta.version.is_some();
    if has_metadata {
        s.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    } else {
        s.push_str("#[derive(Debug, Clone, PartialEq, Default)]\n");
    }
    // `Eq` + `Hash` together, or neither: a `Hash` impl is only useful on a type
    // that is also `Eq`, because that is what `HashMap` / `HashSet` keys require.
    //
    // Both are blocked by the same thing — `serde_json::Value`, which reaches a
    // generated struct twice when `json` is on: inside `LimitedExtensionMap`
    // (`_additional`) and as `ZusatzAttribut::wert`.  `Value` is neither `Eq` nor
    // `Hash` because it wraps `f64`.  With `json` off, both of those degrade to a
    // ZST stub and a `String`, and every remaining field type (`String`, `bool`,
    // `i64`, `Decimal`, `time::Date`, `time::OffsetDateTime`, the identifiers, and
    // the generated enums) is `Eq + Hash` — so the whole tree is.
    s.push_str("#[cfg_attr(not(feature = \"json\"), derive(Eq, Hash))]\n");
    s.push_str("#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n");
    s.push_str("#[cfg_attr(feature = \"builder\", derive(typed_builder::TypedBuilder))]\n");
    s.push_str("#[cfg_attr(feature = \"validate\", derive(garde::Validate))]\n");
    // allow_unvalidated: fields without an explicit #[garde(...)] attribute are
    // implicitly accepted.  Identifier and nested BO/COM fields get
    // #[garde(dive)] — see `field_carries_validation`.
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
    let _ = fields;
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

/// Emits a `Default` impl that pre-fills the BO4E metadata fields.
///
/// `Default::default()` has to serialize to the same JSON the Python, Go, and
/// .NET implementations emit for an equivalently empty object, and all of them
/// stamp `_typ` and `_version` on every BO **and** every COM.
///
/// Every other field is `Default::default()`, exactly as the derive would have
/// produced, so this impl compiles wherever the derive did.
fn emit_default_impl(s: &mut String, name: &str, fields: &[Field], meta: &Metadata<'_>) {
    s.push_str(&format!(
        "\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self {{\n"
    ));
    for field in fields {
        let value = match field.name.as_str() {
            "_typ" => match &meta.typ_path {
                Some(path) => format!("Some({path})"),
                None => "Default::default()".to_owned(),
            },
            "_version" => match meta.version {
                Some(v) => format!("Some({v:?}.to_owned())"),
                None => "Default::default()".to_owned(),
            },
            _ => "Default::default()".to_owned(),
        };
        s.push_str(&format!("            {}: {value},\n", field.rust_name));
    }
    s.push_str("            _additional: Default::default(),\n");
    s.push_str("        }\n    }\n}\n");
}

/// Emits `new(...)` for a struct that has required fields and therefore no
/// `Default` — in v202607, `Lastgang` and `Tarif`.
///
/// Without it those two are the only generated types not constructible except
/// through the `builder` feature or by writing out every optional field.
///
/// Parameters are the required fields in declaration order; everything else, the
/// BO4E metadata included, is filled in exactly as `Default` would.
fn emit_required_field_constructor(
    s: &mut String,
    name: &str,
    fields: &[Field],
    meta: &Metadata<'_>,
) {
    let required: Vec<&Field> = fields
        .iter()
        .filter(|f| !f.name.starts_with('_') && !f.is_optional)
        .collect();
    if required.is_empty() {
        return;
    }

    let params = required
        .iter()
        .map(|f| {
            format!(
                "{}: {}",
                f.rust_name,
                field_type_to_rust(&f.field_type, false)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let listed = required
        .iter()
        .map(|f| format!("`{}`", f.rust_name))
        .collect::<Vec<_>>()
        .join(", ");
    let (count, is_are, its_their, type_s) = if required.len() == 1 {
        ("one field".to_owned(), "is", "its", "type")
    } else {
        (
            format!("{} fields", required.len()),
            "are",
            "their",
            "types",
        )
    };

    // Name only the metadata this struct actually carries: `Lastgang` has a
    // plain `version` property rather than BO4E's `_version`, so claiming both
    // would be wrong for it.
    let stamped: Vec<&str> = [
        meta.typ_path.is_some().then_some("`_typ`"),
        meta.version.is_some().then_some("`_version`"),
    ]
    .into_iter()
    .flatten()
    .collect();

    s.push_str(&format!("\nimpl {name} {{\n"));
    s.push_str(&format!(
        "    /// Creates a `{name}` from the {count} the BO4E schema marks `required`,\n\
         \x20   /// defaulting every other field.\n\
         \x20   ///\n\
         \x20   /// `{name}` has no [`Default`]: {listed} {is_are} required, and {its_their}\n\
         \x20   /// {type_s} need not implement `Default` — so this is the\n\
         \x20   /// `..Default::default()` stand-in.\n"
    ));
    if !stamped.is_empty() {
        s.push_str(&format!(
            "    /// {} {} stamped exactly as elsewhere.\n",
            stamped.join(" and "),
            if stamped.len() == 1 { "is" } else { "are" },
        ));
    }
    if required.len() > 3 {
        s.push_str(
            "    ///\n\
             \x20   /// With this many parameters the `builder` feature reads better at a\n\
             \x20   /// call site; this exists so the type is constructible without it.\n",
        );
    }
    s.push_str("    #[must_use]\n");
    // clippy's threshold is 7. `Tarif` genuinely has ten required fields, and a
    // constructor is the only feature-free way to reach them — the doc comment
    // above already points at the builder for readability.
    if required.len() > 7 {
        s.push_str("    #[allow(clippy::too_many_arguments)]\n");
    }
    s.push_str(&format!(
        "    pub fn new({params}) -> Self {{\n        Self {{\n"
    ));
    for field in fields {
        // Field-init shorthand for the parameters: `x: x` trips
        // `clippy::redundant_field_names`.
        if !field.name.starts_with('_') && !field.is_optional {
            s.push_str(&format!("            {},\n", field.rust_name));
            continue;
        }
        let value = match field.name.as_str() {
            "_typ" => match &meta.typ_path {
                Some(path) => format!("Some({path})"),
                None => "Default::default()".to_owned(),
            },
            "_version" => match meta.version {
                Some(v) => format!("Some({v:?}.to_owned())"),
                None => "Default::default()".to_owned(),
            },
            _ => "Default::default()".to_owned(),
        };
        s.push_str(&format!("            {}: {value},\n", field.rust_name));
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

/// Emits the recursive `Bo4eExtensions` walker for a struct.
///
/// The generated `collect_extension_paths` reports every key this struct's
/// `_additional` map holds, then descends into every nested BO and COM (through
/// `Option`s and `Vec`s) and does the same there. Enums are skipped: they carry
/// no fields, so they carry no extension data.
///
/// The whole impl is gated on `json`. Without it `_additional` is a zero-sized
/// stub and serde simply drops an unknown key, so there is nothing left to
/// report and a check that answered "clean" would be lying.
fn emit_extensions_struct_impl(s: &mut String, name: &str, fields: &[Field]) {
    let stmts: Vec<String> = fields.iter().filter_map(extensions_field_stmt).collect();

    s.push_str("#[cfg(feature = \"json\")]\n");
    s.push_str(&format!("impl crate::json::Bo4eExtensions for {name} {{\n"));
    s.push_str("    fn collect_extension_paths(&self, path: &str, out: &mut Vec<String>) {\n");
    // The struct's own undeclared keys, at its own path. Read straight off
    // `_additional` rather than through `Bo4eExtensionData::extension_data`,
    // which substitutes a shared `LazyLock` empty map: a clean struct is the
    // common case, and this way it costs a null check rather than a lazy deref.
    s.push_str("        if let Some(map) = self._additional.as_map() {\n");
    s.push_str("            for key in map.keys() {\n");
    s.push_str("                out.push(crate::strict::extension_path(path, key));\n");
    s.push_str("            }\n");
    s.push_str("        }\n");
    for stmt in &stmts {
        s.push_str("        ");
        s.push_str(stmt);
        s.push('\n');
    }
    s.push_str("    }\n");
    s.push_str("}\n");
}

/// Returns the recursion statement for one struct field, or `None` when the
/// field is not a nested struct.
///
/// Mirrors [`strict_field_stmt`], minus the enum arm: an enum has no fields, so
/// it cannot hold extension data.
fn extensions_field_stmt(field: &Field) -> Option<String> {
    fn descendable(ft: &FieldType) -> bool {
        matches!(ft, FieldType::Bo(_) | FieldType::Com(_))
    }
    let json = &field.name;
    let rust = &field.rust_name;
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
                 crate::json::Bo4eExtensions::collect_extension_paths({elem}, &crate::strict::index_path(&child, i), out); }}"
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
                let expr = if is_bo { "&**v" } else { "v" };
                Some(format!(
                    "if let Some(v) = &self.{rust} {{ crate::json::Bo4eExtensions::collect_extension_paths({expr}, &crate::strict::field_path(path, \"{json}\"), out); }}"
                ))
            } else {
                let expr = if is_bo {
                    format!("&**self.{rust}")
                } else {
                    format!("&self.{rust}")
                };
                Some(format!(
                    "crate::json::Bo4eExtensions::collect_extension_paths({expr}, &crate::strict::field_path(path, \"{json}\"), out);"
                ))
            }
        }
        _ => None,
    }
}

/// Emits all trait impls for a generated struct: `Bo4eTyped` and its kind
/// marker, `Bo4eJsonExt`, `Sealed`, `Bo4eExtensionData`, `Display`, and
/// `Bo4eStrict`.
fn emit_struct_impls(
    s: &mut String,
    name: &str,
    is_bo: bool,
    meta: &Metadata<'_>,
    schema_version: &str,
    fields: &[Field],
) {
    // `Bo4eTyped` — the `_typ` discriminant as constants, for every struct the
    // schema pins one on. `ZusatzAttribut` is the single BO4E schema that
    // declares no `_typ`, so it gets none of this.
    //
    // The marker beside it (`Bo4eObject` / `Bo4eComponent`) is what lets a bound
    // say "any Geschäftsobjekt" or "any component" without naming the
    // version-scoped discriminant enum.
    if let (Some(discriminant), Some(typ_enum), Some(typ_wire)) =
        (meta.typ_path.as_deref(), meta.typ_enum, meta.typ_wire)
    {
        // The BO4E release *without* the `v` the git tag prefixes it with — the
        // same string the `_version` field carries, so the two cannot disagree.
        let wire_version = meta
            .version
            .unwrap_or_else(|| schema_version.trim_start_matches('v'));
        // The series is the `YYYYMM` prefix — the granularity at which this crate
        // exposes a module, and the only part of the version a dispatcher can
        // match on without breaking every time BO4E ships a patch inside a series.
        let series = wire_version.split('.').next().unwrap_or(wire_version);
        // `TYP_WIRE` is a literal rather than `TYP.as_wire()`, because
        // `Bo4eEnum::as_wire` is a trait method and cannot run in a const
        // initializer; `tests/generated_contract.rs` pins the two together.
        s.push_str(&format!(
            "\nimpl Bo4eTyped for {name} {{\n             \x20   type Typ = {typ_enum};\n             \x20   const TYP: {typ_enum} = {discriminant};\n             \x20   const TYP_WIRE: &'static str = \"{typ_wire}\";\n             \x20   const SCHEMA_VERSION: &'static str = \"{wire_version}\";\n             \x20   const SCHEMA_SERIES: &'static str = \"{series}\";\n             }}\n"
        ));
        s.push_str(&format!(
            "impl crate::bo4e_typed_sealed::Sealed for {name} {{}}\n"
        ));

        let (marker, marker_sealed) = if is_bo {
            ("Bo4eObject", "bo4e_object_sealed")
        } else {
            ("Bo4eComponent", "bo4e_component_sealed")
        };
        s.push_str(&format!("impl {marker} for {name} {{}}\n"));
        s.push_str(&format!(
            "impl crate::{marker_sealed}::Sealed for {name} {{}}\n"
        ));
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

    // Bo4eStrict: recursive out-of-schema (Unknown) enum-*value* detection.
    emit_strict_struct_impl(s, name, fields);

    // Bo4eExtensions: the sibling walk, over out-of-schema *fields*. A decode
    // cannot detect a renamed key — serde ignores what a struct does not declare
    // and this crate keeps it in `_additional` — so a producer that checks a
    // document by round-tripping it checks nothing. This is the call that does.
    emit_extensions_struct_impl(s, name, fields);
}

/// Whether `ft` is a bare decimal — the shape `crate::decimal_serde` handles.
///
/// An *array* of decimals is deliberately excluded: the module's visitors read a
/// scalar, and no BO4E v202607 field has that shape. [`emit_feature_gated_field`]
/// still emits a correct `Vec<String>` fallback for one, so a future schema that
/// introduces it compiles and round-trips — just without the number-spelling
/// tolerance a scalar gets.
fn is_decimal_scalar(ft: &FieldType) -> bool {
    matches!(ft, FieldType::Primitive(PrimitiveType::Decimal))
}

/// The `crate::decimal_serde` entry point for a field of this optionality.
fn decimal_serde_fn(is_optional: bool) -> &'static str {
    if is_optional {
        "crate::decimal_serde::deserialize_opt"
    } else {
        "crate::decimal_serde::deserialize"
    }
}

fn emit_field(s: &mut String, field: &Field, meta: &Metadata<'_>) {
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
        match (field.name.as_str(), meta.version) {
            ("_version", Some(v)) => s.push_str(&format!(
                "    #[cfg_attr(feature = \"builder\", builder(default = Some({v:?}.to_owned()), setter(into)))]\n"
            )),
            _ => s.push_str(
                "    #[cfg_attr(feature = \"builder\", builder(default, setter(into)))]\n",
            ),
        }
    }

    // garde: dive into anything carrying rules of its own — the identifier
    // newtypes and every nested BO / COM — so `.validate()` covers the tree.
    //
    // `garde` supplies the `Validate` impls for `Option`, `Vec`, and `Box`, so
    // one `dive` reaches through `Option<Vec<Box<T>>>`. Enums and scalars carry
    // no rules. A self-referential type (`Marktlokation` → `Lokationszuordnung`
    // → …) recurses over the data, which is finite because the indirection is
    // `Box`.
    if field_carries_validation(&field.field_type) {
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

    // A decimal arrives as a JSON string (BO4E-python) or a JSON number
    // (go-bo4e).  Neither serde's `String` impl nor `rust_decimal`'s own
    // `Deserialize` covers both *and* reports which spelling it saw, so both
    // builds route through `crate::decimal_serde` — whose return type flips with
    // the feature, which is why the attribute is one string here rather than two
    // cfg-split ones.  `deserialize_with`, not `with`: schemars resolves a
    // `serde(with)` path as a *type*, and this is a function.  Serialization
    // needs no adapter (`rust_decimal` already writes the BO4E string form, and
    // serde already writes a `String` as one).
    if is_decimal_scalar(&field.field_type) {
        // `deserialize_with` makes a missing field an error unless `default` is
        // also given, even for an `Option`.
        if field.is_optional {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(deserialize_with = \"{}\"))]\n",
            decimal_serde_fn(field.is_optional)
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
        emit_feature_gated_field(s, field, "json", &type_str, None);
    } else if has_offset_datetime {
        emit_feature_gated_field(
            s,
            field,
            "time",
            &type_str,
            Some((
                "crate::schema_helpers::opt_datetime_schema",
                "crate::schema_helpers::datetime_schema",
            )),
        );
    } else if has_date {
        emit_feature_gated_field(
            s,
            field,
            "time",
            &type_str,
            Some((
                "crate::schema_helpers::opt_date_schema",
                "crate::schema_helpers::date_schema",
            )),
        );
    } else if matches!(
        &field.field_type,
        FieldType::Primitive(PrimitiveType::Decimal)
    ) || matches!(&field.field_type, FieldType::Array(inner) if matches!(inner.as_ref(), FieldType::Primitive(PrimitiveType::Decimal)))
    {
        emit_feature_gated_field(s, field, "decimal", &type_str, None);
    } else {
        s.push_str(&format!("    pub {}: {type_str},\n", field.rust_name));
    }
}

/// Emits a cfg-gated field pair: primary type under `#[cfg(feature = "…")]` and a
/// `String`/`Option<String>` fallback under `#[cfg(not(feature = "…"))]`.
///
/// `feature` — the Cargo feature name (e.g. `"json"`, `"time"`, `"decimal"`).
/// `primary_type` — the fully resolved type string for the feature-gated variant.
///
/// Both declarations carry the **schema's** description. `schemars` lifts a
/// field's doc comment into the property description, so documenting the fallback
/// with "requires the `time` feature" instead would publish a wire contract that
/// talks about Cargo features and no longer says what the field means. What the
/// feature does to the type is documented once, in the crate-level table.
///
/// `fallback_schema_fns` — when `Some((opt_fn, req_fn))`, `emit_fallback_attrs` emits
///    `schemars(schema_with)` on the fallback field using the correct function path.
///    Pass `None` for types that need no special schemars treatment (Decimal, JsonValue).
fn emit_feature_gated_field(
    s: &mut String,
    field: &Field,
    feature: &str,
    primary_type: &str,
    fallback_schema_fns: Option<(&'static str, &'static str)>,
) {
    // An array field falls back to `Vec<String>`, not `String`: the JSON is still
    // an array, and a scalar fallback would fail to deserialize every payload
    // that carries one.  No v202607 field has this shape; the branch is here so a
    // future schema that adds one is generated correctly rather than silently.
    let inner_fallback = match &field.field_type {
        FieldType::Array(_) => "Vec<String>",
        _ => "String",
    };
    let fallback_type = if field.is_optional {
        format!("Option<{inner_fallback}>")
    } else {
        inner_fallback.to_owned()
    };
    s.push_str(&format!("    #[cfg(feature = \"{feature}\")]\n"));
    s.push_str(&format!("    pub {}: {primary_type},\n", field.rust_name));
    // The fallback carries the *schema's* description, exactly as the primary
    // does. `schemars` lifts a field's doc comment into the property description,
    // so a fallback documented only with "requires the `time` feature" publishes
    // a wire contract that talks about Cargo features and no longer says what the
    // field means.
    if let Some(doc) = &field.description {
        for line in clean_description(doc).lines() {
            s.push_str(&format!("    /// {}\n", line));
        }
    }
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
    // Same decimal adapter as the primary field: the fallback reads the two BO4E
    // number spellings into the lexical `String` form.  See the block in
    // `emit_field` for why one attribute serves both builds.
    if is_decimal_scalar(&field.field_type) {
        if field.is_optional {
            s.push_str("    #[cfg_attr(feature = \"serde\", serde(default))]\n");
        }
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(deserialize_with = \"{}\"))]\n",
            decimal_serde_fn(field.is_optional)
        ));
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

fn emit_enum(en: &EnumNode, names: &DiscriminantNames) -> Result<String> {
    // Resolve every variant name up front: `BoTyp` / `ComTyp` take theirs from the
    // struct each discriminant names, everything else from `screaming_to_camel`.
    let variant_pairs: Vec<(String, String)> = en
        .variants
        .iter()
        .map(|(wire, _)| (names.variant(&en.name, wire), wire.clone()))
        .collect();
    // A `_` survives in a variant name only where dropping it would merge two
    // distinct wire values (`MESSPREIS_G2_5` vs `MESSPREIS_G25`); that trips the
    // `non_camel_case_types` lint, so silence it for exactly those enums.
    let needs_allow = en
        .variants
        .iter()
        .any(|(wire, _)| needs_non_camel_case_allow(wire));

    let mut s = String::from("// @generated — do not edit by hand\n\n");

    if needs_allow {
        // Two BO4E values here differ only by a separator between digit runs
        // (`MESSPREIS_G2_5` = meter size G 2.5 vs `MESSPREIS_G25` = G 25), so the
        // underscore has to survive into the Rust identifier to keep them apart.
        s.push_str("#[allow(non_camel_case_types)]\n");
    }
    // `PartialOrd`/`Ord` order by *declaration* position, which is BO4E's own
    // order in the schema's `enum` array — arbitrary as a business ranking, but a
    // total order, which is what `BTreeMap` / `BTreeSet` keys, `sort()`, and a
    // derived `Ord` on a caller's struct need.  The doc comment below says so.
    s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
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
        // …but keep it out of the wire contract. Both derives lift a type's
        // rustdoc into the schema `description`, and these notes are addressed to
        // whoever maintains the Rust binding — codelist provenance, an upstream
        // gap — not to whoever reads the OpenAPI document. Pinning the schema's
        // own sentence here is the same call the `Ord` note makes by staying on
        // the `Bo4eEnum` trait, and the same one `identifiers::schema` makes for
        // the identifier newtypes.
        if let Some(doc) = &en.description {
            let wire = clean_description(doc).replace('\n', " ");
            let wire = wire.trim();
            s.push_str(&format!(
                "#[cfg_attr(feature = \"schemars\", schemars(description = {wire:?}))]\n"
            ));
            s.push_str(&format!(
                "#[cfg_attr(feature = \"utoipa\", schema(description = {wire:?}))]\n"
            ));
        }
    }
    // What the derived `Ord` means is documented once on the `Bo4eEnum` trait,
    // not here: schemars and utoipa lift a type's rustdoc verbatim into the
    // generated JSON Schema / OpenAPI `description`, and a note about a Rust
    // trait has no business in a wire-contract description every consumer reads.
    //
    // Prevents downstream exhaustive match arms; complements the `Unknown` catch-all
    // by enforcing compile-time forward-compatibility for external crates.
    s.push_str("#[non_exhaustive]\n");
    s.push_str(&format!("pub enum {} {{\n", en.name));

    // A collision would emit a duplicate variant and fail to compile pointing at
    // generated code rather than at its cause.  Fail here instead, naming both
    // offending values.
    let mut seen_variants: HashSet<&str> = HashSet::new();
    for ((rust, wire), (_, doc)) in variant_pairs.iter().zip(&en.variants) {
        if !seen_variants.insert(rust.as_str()) {
            anyhow::bail!(
                "enum {}: wire value {wire:?} produces the Rust variant {rust:?}, which another \
                 value already claims; extend `naming::screaming_to_camel` to keep them apart",
                en.name,
            );
        }
        if let Some(d) = doc {
            for line in clean_description(d).lines() {
                s.push_str(&format!("    /// {}\n", line));
            }
        }
        // Curated per-variant interop note (e.g. the cross-BO "Messsystem" spelling
        // discrepancy) rendered right where a developer selecting the variant sees it.
        if let Some(note) = enum_variant_note(&en.name, wire) {
            for line in note.lines() {
                s.push_str(&format!("    /// {line}\n"));
            }
        }
        // Always emit serde(rename) so the serialized value is the canonical JSON string.
        // Also emit strum(serialize) so strum::Display / AsRef / EnumString
        // produce the same canonical string as serde — not the Rust variant name.
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(rename = \"{wire}\"))]\n"
        ));
        s.push_str(&format!(
            "    #[cfg_attr(feature = \"strum\", strum(serialize = \"{wire}\"))]\n"
        ));
        s.push_str(&format!("    {rust},\n"));
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
    // The template already opens the line with `/// `, so this contributes only
    // the assertion plus the newline+indent that continues the doc block.  A
    // second `/// ` here would comment the assertion out *inside* the code block,
    // leaving a doctest that compiles and asserts nothing.
    let from_wire_positive = variant_pairs
        .first()
        .map(|(rust, wire)| {
            format!("assert_eq!({enum_name}::from_wire(\"{wire}\"), Ok({enum_name}::{rust}));\n    /// ")
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

/// Lets `Vec<{enum_name}>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for {enum_name} {{
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {{
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }}
}}
"#
    ));

    // Proptest Arbitrary impl — samples from the feature-independent
    // known-variant table, so it needs no `strum`.
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

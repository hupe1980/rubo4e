//! Abstract Syntax Tree produced by the JSON Schema parser.
//!
//! Each variant represents one top-level schema definition in a BO4E release.

// ─── Field types ─────────────────────────────────────────────────────────────

/// Primitive Rust scalar that a field maps to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Bool,
    I64,
    /// `rust_decimal::Decimal` (feature = "decimal")
    Decimal,
    /// `time::OffsetDateTime` (feature = "time") — ISO 8601 date-time
    OffsetDateTime,
    /// `time::Date` (feature = "time") — ISO 8601 date-only `YYYY-MM-DD`
    Date,
}

/// The Rust type a BO4E field maps to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    /// A domain identifier newtype (e.g. `MaloId`, `EicCode`).
    Identifier(String),
    /// A reference to another BO4E Geschäftsobjekt.
    Bo(String),
    /// A reference to another BO4E COM type.
    Com(String),
    /// A reference to a BO4E enum.
    BoEnum(String),
    /// An inline scalar.
    Primitive(PrimitiveType),
    /// `Vec<T>`
    Array(Box<FieldType>),
    /// Fallback: raw JSON value (for schema-level `anyOf` / unresolvable unions).
    JsonValue,
}

/// A single field on a BO4E struct.
#[derive(Debug, Clone)]
pub struct Field {
    /// Original JSON Schema property name (camelCase).
    pub name: String,
    /// Snake-case Rust identifier (may differ from `name`).
    pub rust_name: String,
    /// Whether the field is optional in the schema.
    pub is_optional: bool,
    /// Resolved Rust type.
    pub field_type: FieldType,
    /// Markdown documentation from `description` in the schema.
    pub description: Option<String>,
}

// ─── Top-level nodes ──────────────────────────────────────────────────────────

/// Whether a struct schema is a Geschäftsobjekt (BO) or a component (COM).
///
/// The two differ only in which discriminant enum their `_typ` field draws from,
/// and in whether they implement [`Bo4eObject`](../../rubo4e/trait.Bo4eObject.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructKind {
    /// A BO4E Geschäftsobjekt — `_typ` is a `BoTyp`.
    Bo,
    /// A BO4E component — `_typ` is a `ComTyp`.
    Com,
}

impl StructKind {
    /// Name of the discriminant enum this kind's `_typ` field holds.
    pub const fn typ_enum(self) -> &'static str {
        match self {
            Self::Bo => "BoTyp",
            Self::Com => "ComTyp",
        }
    }

    /// Returns `true` for [`StructKind::Bo`].
    pub const fn is_bo(self) -> bool {
        matches!(self, Self::Bo)
    }
}

/// A BO4E struct schema — a Geschäftsobjekt or a component.
///
/// BOs and COMs are structurally identical; [`StructNode::kind`] is the only
/// thing that distinguishes them, so they share one node type.
#[derive(Debug, Clone)]
pub struct StructNode {
    pub name: String,
    pub kind: StructKind,
    pub fields: Vec<Field>,
    pub description: Option<String>,
    /// The `_typ` discriminant this schema pins, as it appears **on the wire**
    /// (e.g. `"MARKTLOKATION"`).  `None` for schemas without a `_typ` property.
    ///
    /// Read from the schema's `const` (or its `default`, which BO4E uses when
    /// `_typ` is declared as a nullable `$ref` to `BoTyp`) rather than derived
    /// from the type name, so a schema that ever disagrees with the convention
    /// still generates the discriminant its own JSON declares.
    pub typ_const: Option<String>,
    /// The `_version` value this schema declares as its default, as it appears
    /// **on the wire** (e.g. `"202607.0.0"` — note: no `v` prefix, unlike the
    /// release tag).  `None` for schemas without a `_version` property.
    pub version_default: Option<String>,
}

/// A BO4E enum — maps to a Rust `enum`.
#[derive(Debug, Clone)]
pub struct EnumNode {
    pub name: String,
    /// `(variant_name, doc_comment)`
    pub variants: Vec<(String, Option<String>)>,
    pub description: Option<String>,
}

/// Any top-level BO4E schema definition.
#[derive(Debug, Clone)]
pub enum SchemaNode {
    Struct(StructNode),
    Enum(EnumNode),
}

impl SchemaNode {
    pub fn name(&self) -> &str {
        match self {
            Self::Struct(n) => &n.name,
            Self::Enum(n) => &n.name,
        }
    }

    /// Returns the struct node, or `None` for enums.
    pub fn as_struct(&self) -> Option<&StructNode> {
        match self {
            Self::Struct(n) => Some(n),
            Self::Enum(_) => None,
        }
    }
}

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
    /// `time::OffsetDateTime` (feature = "time")
    OffsetDateTime,
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

/// A BO4E Geschäftsobjekt (business object) — maps to a Rust `struct`.
#[derive(Debug, Clone)]
pub struct BoNode {
    pub name: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
}

/// A BO4E COM type (component) — maps to a Rust `struct`.
#[derive(Debug, Clone)]
pub struct ComNode {
    pub name: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
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
    Bo(BoNode),
    Com(ComNode),
    Enum(EnumNode),
}

impl SchemaNode {
    pub fn name(&self) -> &str {
        match self {
            Self::Bo(n) => &n.name,
            Self::Com(n) => &n.name,
            Self::Enum(n) => &n.name,
        }
    }
}

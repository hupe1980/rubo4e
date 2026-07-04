/// AST types produced by the parser and consumed by the emitter.
pub mod ast;
/// Code emitter: AST → Rust source.
pub mod emitter;
/// Type-inference table (JSON property name → Rust type override).
pub mod inference;
/// JSON-Schema parser: JSON → AST.
pub mod parser;

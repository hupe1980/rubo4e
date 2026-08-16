//! The `_typ` discriminant is fixed by the schema, so the generated builder
//! declares it `builder(setter(skip))` with a constant default. Attempting to set
//! it must therefore be a compile error rather than silently producing a BO whose
//! `_typ` contradicts its Rust type.

use rubo4e::v202607::{BoTyp, Vertrag};

fn main() {
    let _ = Vertrag::builder().typ(BoTyp::Vertrag).build();
}

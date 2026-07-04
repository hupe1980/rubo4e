use rubo4e::v202501::{BoTyp, Vertrag};

fn main() {
    let _ = Vertrag::builder().typ(BoTyp::Vertrag).build();
}
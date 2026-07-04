//! Demonstrates the typed builder API for constructing BO4E structs.
//!
//! Optional fields use `strip_option` — pass the value directly without wrapping in `Some`.
//!
//! Run with:
//!   cargo run --example builder --features versioned,builder,json

use rubo4e::{
    json::Bo4eJsonExt,
    v202501::{Adresse, Menge, Mengeneinheit, Sparte, Vertrag},
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    // ── COM: Adresse ─────────────────────────────────────────────────────────
    let adresse = Adresse::builder()
        .strasse("Musterstraße".to_string())
        .hausnummer("1a".to_string())
        .postleitzahl("10115".to_string())
        .ort("Berlin".to_string())
        .build();

    println!("Adresse: {}", adresse.to_json_german().unwrap());

    // ── COM: Menge ───────────────────────────────────────────────────────────
    let menge = Menge::builder()
        .wert(Decimal::from_str("100.0").unwrap())
        .einheit(Mengeneinheit::Kwh)
        .build();

    println!("Menge:   {}", menge.to_json_german().unwrap());

    // ── BO: Vertrag ──────────────────────────────────────────────────────────
    let vertrag = Vertrag::builder()
        .sparte(Sparte::Strom)
        .beschreibung("Jahresvertrag Strom".to_string())
        .vertragsnummer("VN-2025-001".to_string())
        .build();

    println!("Vertrag: {}", vertrag.to_json_german().unwrap());
}

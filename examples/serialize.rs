//! Demonstrates BO4E JSON serialization in three different key formats.
//!
//! Run with:
//!   cargo run --example serialize --features versioned,json,decimal

use rubo4e::{
    json::Bo4eJsonExt,
    v202607::{Bilanzierungsmethode, Energierichtung, Marktlokation, Menge, Mengeneinheit, Sparte},
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    // ── Menge (COM) ──────────────────────────────────────────────────────────
    let menge = Menge {
        wert: Some(Decimal::from_str("42.5").unwrap()),
        einheit: Some(Mengeneinheit::Kwh),
        ..Default::default()
    };

    println!("=== Menge — German camelCase (BO4E wire format) ===");
    println!("{}", menge.to_json_german().unwrap());

    println!("\n=== Menge — snake_case (Rust-friendly) ===");
    println!("{}", menge.to_json_snake_case().unwrap());

    // ── Marktlokation (BO) ───────────────────────────────────────────────────
    let malo = Marktlokation {
        sparte: Some(Sparte::Strom),
        energierichtung: Some(Energierichtung::Aussp),
        bilanzierungsmethode: Some(Bilanzierungsmethode::Slp),
        bilanzierungsgebiet: Some("11YE-N-TEST--GAS-8".to_string()),
        ..Default::default()
    };
    // Marktlokation::default() pre-fills typ via the custom Default impl.

    println!("\n=== Marktlokation — German camelCase ===");
    println!("{}", malo.to_json_german().unwrap());

    println!("\n=== Marktlokation — Canonical (sorted keys, stable for hashing) ===");
    println!("{}", malo.to_json_canonical().unwrap());

    // ── Round-trip ────────────────────────────────────────────────────────────
    let json = malo.to_json_german().unwrap();
    let restored = Marktlokation::from_json_german(&json).unwrap();
    assert_eq!(malo.sparte, restored.sparte);
    assert_eq!(malo.bilanzierungsmethode, restored.bilanzierungsmethode);
    println!("\nRound-trip: OK");
}

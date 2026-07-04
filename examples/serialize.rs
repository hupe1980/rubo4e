//! Demonstrates BO4E JSON serialization in three different key formats.
//!
//! Run with:
//!   cargo run --example serialize --features versioned,json

use rubo4e::{
    json::Bo4eJsonExt,
    v202501::{
        Bilanzierungsmethode, BoTyp, Energierichtung, Marktlokation, Menge, Mengeneinheit, Sparte,
    },
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    // ── Menge (COM) ──────────────────────────────────────────────────────────
    let mut menge = Menge::default();
    menge.wert = Some(Decimal::from_str("42.5").unwrap());
    menge.einheit = Some(Mengeneinheit::Kwh);

    println!("=== Menge — German camelCase (BO4E wire format) ===");
    println!("{}", menge.to_json_german().unwrap());

    println!("\n=== Menge — snake_case (Rust-friendly) ===");
    println!("{}", menge.to_json_snake_case().unwrap());

    // ── Marktlokation (BO) ───────────────────────────────────────────────────
    let mut malo = Marktlokation::default();
    malo.typ = Some(BoTyp::Marktlokation);
    malo.sparte = Some(Sparte::Strom);
    malo.energierichtung = Some(Energierichtung::Aussp);
    malo.bilanzierungsmethode = Some(Bilanzierungsmethode::Slp);
    malo.bilanzierungsgebiet = Some("11YE-N-TEST--GAS-8".to_string());

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

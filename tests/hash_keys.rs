//! Generated types as `HashMap` / `BTreeMap` keys.
//!
//! Structs derive `Eq` and `Hash` only without the `json` feature: with it,
//! `serde_json::Value` reaches them through `_additional` and
//! `ZusatzAttribut::wert` and is neither. Only the half matching the current
//! feature set compiles, so a `Hash` without an `Eq` fails here.
//!
//! Enums are `Eq + Ord + Hash` unconditionally.

#![cfg(feature = "versioned")]

// ─── Structs: only without `json` ────────────────────────────────────────────

/// A BO with an identifier field, keyed by value — the ID-keyed lookup the
/// derive exists for.
#[test]
#[cfg(not(feature = "json"))]
fn a_bo_can_key_a_hash_map() {
    use rubo4e::current::{Marktlokation, Sparte};
    use std::collections::{HashMap, HashSet};

    let strom = Marktlokation {
        sparte: Some(Sparte::Strom),
        ..Default::default()
    };
    let gas = Marktlokation {
        sparte: Some(Sparte::Gas),
        ..Default::default()
    };

    let mut seen: HashMap<Marktlokation, u32> = HashMap::new();
    *seen.entry(strom.clone()).or_default() += 1;
    *seen.entry(strom.clone()).or_default() += 1;
    seen.insert(gas.clone(), 7);

    assert_eq!(seen.get(&strom), Some(&2));
    assert_eq!(seen.get(&gas), Some(&7));

    let set: HashSet<Marktlokation> = [strom.clone(), strom, gas].into_iter().collect();
    assert_eq!(set.len(), 2, "equal values must collapse in a HashSet");
}

/// A COM too — `Betrag` and `Menge` end up in dedup sets more often than a BO.
#[test]
#[cfg(not(feature = "json"))]
fn a_com_can_key_a_hash_map() {
    use rubo4e::current::{Menge, Mengeneinheit};
    use std::collections::HashMap;

    let kwh = Menge {
        einheit: Some(Mengeneinheit::Kwh),
        ..Default::default()
    };
    let mut totals: HashMap<Menge, u32> = HashMap::new();
    totals.insert(kwh.clone(), 1);
    assert_eq!(totals.get(&kwh), Some(&1));
}

// ─── Enums: always ───────────────────────────────────────────────────────────

/// Every generated enum keys both map kinds, whatever the feature set.
#[test]
fn an_enum_keys_both_map_kinds() {
    use rubo4e::current::Sparte;
    use std::collections::{BTreeMap, HashMap};

    let mut by_hash: HashMap<Sparte, &str> = HashMap::new();
    by_hash.insert(Sparte::Strom, "electricity");
    assert_eq!(by_hash.get(&Sparte::Strom), Some(&"electricity"));

    let mut by_order: BTreeMap<Sparte, &str> = BTreeMap::new();
    by_order.insert(Sparte::Gas, "gas");
    by_order.insert(Sparte::Strom, "electricity");
    // Declaration order: STROM is listed before GAS in the schema.
    assert_eq!(
        by_order.keys().copied().collect::<Vec<_>>(),
        [Sparte::Strom, Sparte::Gas]
    );
}

/// `Unknown` sorts last, so a sorted report keeps the out-of-schema values
/// together at the end rather than scattered through the known ones.
#[test]
fn the_unknown_catch_all_sorts_last() {
    use rubo4e::current::Sparte;

    let mut all: Vec<Sparte> = Sparte::VARIANTS.to_vec();
    all.push(Sparte::Unknown);
    all.sort();
    assert_eq!(all.last(), Some(&Sparte::Unknown));
}

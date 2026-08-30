//! Drift guard: the prelude must re-export every identifier type.
//!
//! Rust cannot enumerate a module's items at runtime, so this reads the two
//! source files and compares their `pub use` lists — the approach
//! `generated_contract.rs` takes to the generated code.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// The leaf names every `use` statement starting with `prefix` brings into scope.
///
/// Handles both shapes the two files use — `path::{A, B}` and `path::A` — plus
/// the one-per-line wrapping rustfmt applies to a long list, and reports a
/// `use x::y as z` alias under `z`.
fn exported_names(src: &str, prefix: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = src;
    while let Some(at) = rest.find(prefix) {
        let after = &rest[at + prefix.len()..];
        rest = after;
        let Some(end) = after.find(';') else { continue };
        let statement = &after[..end];
        // Everything inside the braces, or the whole tail for a single item.
        let list = match (statement.find('{'), statement.rfind('}')) {
            (Some(open), Some(close)) if open < close => &statement[open + 1..close],
            _ => statement,
        };
        for item in list.split(',') {
            let name = item
                .rsplit(" as ") // `foo as bar` exports `bar`
                .next()
                .unwrap_or_default()
                .rsplit("::") // drop any remaining module path
                .next()
                .unwrap_or_default()
                .trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                found.insert(name.to_owned());
            }
        }
    }
    found
}

/// Whether `name` is a type rather than a constant or a serde adapter module.
///
/// UpperCamelCase, which is what separates `MaloId` from `AKIV_ID_MAX_LEN` and
/// from `marktpartner_id_as_i64`. The prelude carries types; a bound or an
/// adapter is named explicitly at the one call site that needs it.
fn is_type_name(name: &str) -> bool {
    name.starts_with(char::is_uppercase) && name.contains(char::is_lowercase)
}

#[test]
fn prelude_reexports_every_identifier_type() {
    let module = read("src/identifiers/mod.rs");
    let lib = read("src/lib.rs");

    let public: BTreeSet<String> = exported_names(&module, "pub use ")
        .into_iter()
        .filter(|n| is_type_name(n))
        .collect();
    let in_prelude = exported_names(&lib, "pub use crate::identifiers::");

    assert!(
        public.len() >= 16,
        "expected the full identifier family in src/identifiers/mod.rs, found {public:?}"
    );

    let missing: Vec<&String> = public.difference(&in_prelude).collect();
    assert!(
        missing.is_empty(),
        "rubo4e::prelude documents \"every identifier type\" but does not re-export \
         {missing:?} — add them to the prelude, or change what the docs promise"
    );

    // The other direction: a prelude entry that no longer exists would fail to
    // compile, so this only catches a name the prelude invents.
    let unknown: Vec<&String> = in_prelude.difference(&public).collect();
    assert!(
        unknown.is_empty(),
        "prelude re-exports {unknown:?}, which src/identifiers/mod.rs does not export"
    );
}

/// The prelude's own promise, exercised rather than parsed: if any of these
/// stops being reachable through `rubo4e::prelude`, this file stops compiling.
#[test]
fn every_identifier_is_reachable_through_the_prelude() {
    #[allow(unused_imports)]
    use rubo4e::prelude::*;

    fn assert_identifier<T: std::str::FromStr + std::fmt::Display + AsRef<str>>() {}

    assert_identifier::<AkivId>();
    assert_identifier::<BilanzierungsgebietId>();
    assert_identifier::<Bic>();
    assert_identifier::<BilanzkreisId>();
    assert_identifier::<CrId>();
    assert_identifier::<EicCode>();
    assert_identifier::<Iban>();
    assert_identifier::<MaloId>();
    assert_identifier::<MarktpartnerId>();
    assert_identifier::<MeloId>();
    assert_identifier::<NebeId>();
    assert_identifier::<NeloId>();
    assert_identifier::<ObisCode>();
    assert_identifier::<PaketId>();
    assert_identifier::<SgId>();
    assert_identifier::<SrId>();
    assert_identifier::<TrId>();
    assert_identifier::<TranchennummerId>();

    // The helper enums the accessors return come along too, so a caller never
    // has to reach past the prelude to name a return type.
    let _: fn(&MaloId) -> MaloVergabestelle = MaloId::vergabestelle;
    let _: fn(&MarktpartnerId) -> MpIdAuthority = MarktpartnerId::authority;
    let _: fn(&EicCode) -> EicType = EicCode::eic_type;
    let _: fn(&ObisCode) -> ObisComponents = ObisCode::components;
}

/// `sqlx_impls.rs` names its types in a hand-maintained macro invocation, so a
/// new identifier gets its `Type` / `Encode` / `Decode` / `PgHasArrayType` impls
/// only if someone remembers to add it there.
///
/// Nothing fails to compile when they do not — the identifier just quietly
/// cannot be a column. This compares the list against the module's own exports,
/// the same way the prelude guard above does.
/// The types `src/identifiers/` exports that are **not** identifier newtypes.
///
/// Each is a plain helper enum or struct an accessor returns — they wrap no
/// string and cannot be a database column. Listed rather than inferred, so a new
/// export lands in one of the two categories deliberately: an addition here is a
/// statement that the type is not an identifier.
const NON_IDENTIFIER_HELPERS: &[&str] = &[
    "EicType",           // EicCode::eic_type
    "MaloVergabestelle", // MaloId::vergabestelle
    "MpIdAuthority",     // MarktpartnerId::authority
    "ObisComponents",    // ObisCode::components
    // The two below differ from the four above in one way: an `EicType` is read
    // *out of* its identifier (position 3), whereas a Zählpunktart cannot be — a
    // Zählpunkt (eMob) and a MeLo-ID are indistinguishable as strings. They are
    // still classification helpers beside the identifier they classify, not
    // identifiers, so they are no more a SQL column than an `EicType` is.
    "Zaehlpunkt",    // Zaehlpunktbezeichnung + what it names
    "Zaehlpunktart", // Zaehlpunkt::art
];

#[test]
fn every_identifier_has_sqlx_impls() {
    let module = read("src/identifiers/mod.rs");
    let public: BTreeSet<String> = exported_names(&module, "pub use ")
        .into_iter()
        .filter(|n| is_type_name(n))
        .filter(|n| !NON_IDENTIFIER_HELPERS.contains(&n.as_str()))
        .collect();

    // The macro call lists one type per line inside `impl_sqlx_text!( … );`.
    let impls = read("src/identifiers/sqlx_impls.rs");
    let body = impls
        .split_once("impl_sqlx_text!(")
        .and_then(|(_, rest)| rest.split_once(");"))
        .map(|(list, _)| list)
        .expect("sqlx_impls.rs must contain an impl_sqlx_text! invocation");
    let covered: BTreeSet<String> = body
        .split(',')
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_owned)
        .collect();

    let missing: Vec<&String> = public.difference(&covered).collect();
    assert!(
        missing.is_empty(),
        "the sqlx docs promise these impls for *every* identifier, but {missing:?} \
         are not in `impl_sqlx_text!` — add them, or change what the docs promise"
    );

    let unknown: Vec<&String> = covered.difference(&public).collect();
    assert!(
        unknown.is_empty(),
        "impl_sqlx_text! names {unknown:?}, which src/identifiers/mod.rs does not export"
    );
}

/// Every identifier is a string newtype that implements `Borrow<str>`, and
/// `Borrow` carries a contract: the borrowed form must hash and compare exactly
/// as the owned one does. Break it and a `HashMap<Id, _>::get(&str)` lookup
/// silently misses keys that are present — no error, no panic, just a `None`.
///
/// So every identifier must be findable by the string it renders as, in a
/// `HashMap` and a `BTreeMap` alike. `ObisCode` is the one that needs care: it
/// caches its parsed components alongside its string.
#[test]
fn every_identifier_can_be_looked_up_by_its_string() {
    use rubo4e::prelude::*;
    use std::collections::{BTreeMap, HashMap};

    /// Inserts `id` under itself, then looks it up by `&str`.
    fn round_trip<T>(id: T)
    where
        T: std::hash::Hash + Ord + Clone + AsRef<str> + std::borrow::Borrow<str>,
    {
        let key = id.as_ref().to_owned();

        let mut by_hash: HashMap<T, u32> = HashMap::new();
        by_hash.insert(id.clone(), 1);
        assert_eq!(
            by_hash.get(key.as_str()),
            Some(&1),
            "HashMap lookup by &str missed {key:?} — Borrow<str> and Hash disagree"
        );

        let mut by_order: BTreeMap<T, u32> = BTreeMap::new();
        by_order.insert(id, 1);
        assert_eq!(
            by_order.get(key.as_str()),
            Some(&1),
            "BTreeMap lookup by &str missed {key:?} — Borrow<str> and Ord disagree"
        );
    }

    round_trip(AkivId::new("AKIV-2026-00001").unwrap());
    round_trip(Bic::new("COBADEFFXXX").unwrap());
    round_trip(BilanzierungsgebietId::new("11YN-0000-0001-Q").unwrap());
    round_trip(BilanzkreisId::new("11XSUEDWESTSTRO8").unwrap());
    round_trip(CrId::from_base("A000000001").unwrap());
    round_trip(EicCode::new("10YDE-EON------1").unwrap());
    round_trip(Iban::new("DE89370400440532013000").unwrap());
    round_trip(MaloId::new("41373559241").unwrap());
    round_trip(MarktpartnerId::new("9900357000003").unwrap());
    round_trip(MeloId::new("DE0000000000000000000000000000001").unwrap());
    round_trip(NebeId::from_base("F000000001").unwrap());
    round_trip(NeloId::from_base("E000000001").unwrap());
    round_trip(ObisCode::new("1-0:1.8.0*255").unwrap());
    round_trip(PaketId::from_base("P900000001").unwrap());
    round_trip(SgId::from_base("B000000001").unwrap());
    round_trip(SrId::from_base("C000000001").unwrap());
    round_trip(TrId::from_base("D000000001").unwrap());
    round_trip(TranchennummerId::new("42").unwrap());
}

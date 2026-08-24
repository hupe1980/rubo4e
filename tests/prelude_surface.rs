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

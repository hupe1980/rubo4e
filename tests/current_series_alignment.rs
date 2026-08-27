//! Drift guard: every hand-written module must target the series `current` does.
//!
//! `src/generated/` is written by the generator and re-scanned on every run, so
//! adding a schema series there is mechanical. The hand-written modules that hang
//! off it are not: `convenience`, `units`, `timeseries` and the `validation`
//! macro each name a version in their own source, and advancing `current` without
//! advancing them leaves the crate compiling perfectly while shipping accessors,
//! unit arithmetic and validators for a series nobody is using.
//!
//! There is no compile error waiting to catch that — the older module still
//! exists and still type-checks. So the check is here, reading the sources the
//! way [`generated_contract.rs`](generated_contract.rs) reads the schemas.
//!
//! Adding a series deliberately? Then advance every file this test names. That
//! list *is* the checklist — see the Adding a New Schema Series steps in
//! `site/content/docs/versioning.md`, which this keeps honest.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    let path = root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// Every `vYYYYMM` that appears after `generated::` in `src`, with its file.
fn schema_versions_in(src: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = src;
    while let Some(at) = rest.find("generated::v") {
        let tail = &rest[at + "generated::".len()..];
        let end = tail
            .find(|c: char| !c.is_ascii_alphanumeric())
            .unwrap_or(tail.len());
        let version = &tail[..end];
        // `generated::v202607` — but not `generated::v` in prose or a macro
        // metavariable like `generated::$ver`.
        if version.len() > 1 && version[1..].bytes().all(|b| b.is_ascii_digit()) {
            found.push(version.to_owned());
        }
        rest = &tail[end..];
    }
    found
}

/// The series `rubo4e::current` re-exports, read out of `src/lib.rs`.
fn current_series() -> String {
    let lib = read("src/lib.rs");
    let at = lib
        .find("pub mod current {")
        .expect("src/lib.rs must declare `pub mod current`");
    let body = &lib[at..];
    let versions = schema_versions_in(&body[..body.find('}').expect("unterminated mod current")]);
    assert_eq!(
        versions.len(),
        1,
        "`pub mod current` must re-export exactly one series, found {versions:?}"
    );
    versions.into_iter().next().expect("checked len")
}

/// The hand-written source files, i.e. everything under `src/` that the generator
/// does not write.
fn hand_written_sources() -> BTreeMap<String, String> {
    fn walk(dir: &Path, out: &mut BTreeMap<String, String>) {
        for entry in std::fs::read_dir(dir).expect("readable directory") {
            let path = entry.expect("readable entry").path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "generated") {
                    continue;
                }
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let rel = path
                    .strip_prefix(root())
                    .expect("under the manifest dir")
                    .to_string_lossy()
                    .into_owned();
                out.insert(rel, std::fs::read_to_string(&path).expect("readable file"));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(&root().join("src"), &mut out);
    out
}

/// No hand-written module may reach into a series other than the current one.
///
/// A module left behind on an older series is invisible: it compiles, its tests
/// pass, and it silently serves a schema the crate no longer presents as current.
#[test]
fn every_hand_written_module_targets_the_current_series() {
    let current = current_series();
    let mut stale: Vec<(String, String)> = Vec::new();

    for (file, src) in hand_written_sources() {
        for version in schema_versions_in(&src) {
            if version != current {
                stale.push((file.clone(), version));
            }
        }
    }

    assert!(
        stale.is_empty(),
        "`current` re-exports {current}, but these still target another series: {stale:?}\n\
         Advance them together, or give the older series its own module the way \
         `validation` does with `impl_validators!`."
    );
}

/// The list of files that name a series is the checklist for adding one, so it
/// must not grow without the versioning guide growing with it.
///
/// Pinned by name rather than by count: a new file that hard-codes a version is
/// exactly the thing a future series bump would forget, and the failure message
/// is where it gets remembered.
#[test]
fn the_files_that_pin_a_series_are_the_documented_ones() {
    let expected = [
        "src/convenience.rs",
        "src/lib.rs",
        "src/timeseries.rs",
        "src/units.rs",
    ];

    let actual: Vec<String> = hand_written_sources()
        .into_iter()
        .filter(|(_, src)| !schema_versions_in(src).is_empty())
        .map(|(file, _)| file.replace('\\', "/"))
        .collect();

    assert_eq!(
        actual, expected,
        "the set of hand-written files naming a schema series changed.\n\
         Add or remove it here, and update the Adding a New Schema Series steps \
         in site/content/docs/versioning.md so the checklist stays complete."
    );
}

/// `validation` names its series through the `impl_validators!` macro rather than
/// a `generated::v…` path, so it needs its own check — and it is the one module
/// that keeps a *copy* per series, which is why it does not appear above.
#[test]
fn the_validation_module_instantiates_the_current_series() {
    let current = current_series();
    let src = read("src/validation/mod.rs");

    assert!(
        src.contains(&format!("impl_validators!({current});")),
        "src/validation/mod.rs must instantiate the validators for {current}"
    );
    assert!(
        src.contains(&format!("pub use super::{current}::*;")),
        "`validation::current` must re-export {current}, matching `rubo4e::current`"
    );
}

/// The generator's own snapshot directory must hold exactly the series the crate
/// presents, so `just generate` cannot silently rebuild a series the code does
/// not reference.
#[test]
fn the_committed_snapshot_matches_the_current_series() {
    let current = current_series();
    let dir = root().join("generator/schemas");
    let tags: Vec<String> = std::fs::read_dir(&dir)
        .expect("generator/schemas exists")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with('v'))
        .collect();

    assert_eq!(
        tags.len(),
        1,
        "expected one committed snapshot, got {tags:?}"
    );
    assert!(
        tags[0].starts_with(&format!("{current}.")),
        "the committed snapshot is {}, but `current` re-exports {current}",
        tags[0]
    );
}

// ─── The guard's own machinery ───────────────────────────────────────────────
//
// A drift guard that reads source text can only be trusted if its reader is
// itself pinned: a scanner that silently matches nothing would let every check
// above pass vacuously, which is the exact failure mode a guard must not have.

#[test]
fn the_scanner_finds_a_version_and_ignores_what_is_not_one() {
    assert_eq!(
        schema_versions_in("use crate::generated::v202607::Mengeneinheit;"),
        ["v202607"]
    );
    // Several, in order, including through a fully-qualified call.
    assert_eq!(
        schema_versions_in(
            "crate::generated::v202607::Adresse; <crate::generated::v202701::Betrag>::TYP"
        ),
        ["v202607", "v202701"]
    );
    // The macro metavariable `validation` uses is not a version, and neither is
    // prose. Those are why `validation` gets its own check.
    assert!(schema_versions_in("crate::generated::$ver::*").is_empty());
    assert!(schema_versions_in("the generated::vN modules").is_empty());
    assert!(schema_versions_in("nothing here at all").is_empty());
}

#[test]
fn the_scanner_would_notice_a_stale_module() {
    // The shape the first test asserts against, with one file left behind.
    let current = "v202607";
    let sources = [
        (
            "src/units.rs",
            "use crate::generated::v202607::Mengeneinheit;",
        ),
        ("src/convenience.rs", "crate::generated::v202701::Zeitraum"),
    ];
    let stale: Vec<&str> = sources
        .iter()
        .filter(|(_, src)| schema_versions_in(src).iter().any(|v| v != current))
        .map(|(file, _)| *file)
        .collect();
    assert_eq!(stale, ["src/convenience.rs"]);
}

#[test]
fn the_current_series_is_read_out_of_lib_rs_and_is_a_real_series() {
    let current = current_series();
    assert!(
        current.starts_with('v') && current[1..].len() == 6,
        "expected a vYYYYMM series, got {current:?}"
    );
    assert!(
        current[1..].bytes().all(|b| b.is_ascii_digit()),
        "expected a vYYYYMM series, got {current:?}"
    );
}

#[test]
fn the_hand_written_sweep_actually_reaches_the_sources() {
    let sources = hand_written_sources();
    assert!(
        sources.contains_key("src/lib.rs"),
        "the sweep missed src/lib.rs — it is reading the wrong tree"
    );
    assert!(
        !sources.keys().any(|k| k.contains("generated")),
        "the sweep must exclude src/generated/, which the generator owns"
    );
    assert!(
        sources.len() > 20,
        "implausibly few sources: {}",
        sources.len()
    );
}

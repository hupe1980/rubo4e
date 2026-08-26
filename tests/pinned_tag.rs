//! The pinned schema tag and the MSRV each live in exactly one place.
//!
//! Everything else discovers them. A tag spelled out in a workflow, a recipe, or
//! a site config goes stale the next time BO4E ships a patch inside the series,
//! and nothing else notices until a job three steps into CI fails on a directory
//! that no longer exists.
//!
//! Sources of truth:
//!
//! - **schema tag** — the single directory name under `generator/schemas/`
//! - **MSRV** — `rust-version` in `Cargo.toml`

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    let path = root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// The committed snapshot's tag, e.g. `"v202607.1.0"`.
fn pinned_tag() -> String {
    let dir = root().join("generator/schemas");
    let mut tags: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with('v'))
        .collect();
    tags.sort();
    assert_eq!(
        tags.len(),
        1,
        "expected exactly one schema snapshot under generator/schemas/, found {tags:?} — \
         the tag is derived from that directory name, so more than one is ambiguous"
    );
    tags.pop().expect("checked len")
}

/// Every `vYYYYMM.N.N` literal in `text`.
fn schema_tags_in(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    for (i, _) in text.match_indices('v') {
        // v + 6 digits + '.' + digits + '.' + digits
        let rest = &text[i..];
        let mut chars = rest.char_indices().skip(1);
        let digits: String = (&mut chars)
            .take_while(|(_, c)| c.is_ascii_digit())
            .map(|(_, c)| c)
            .collect();
        if digits.len() != 6 {
            continue;
        }
        // Re-scan from the start for the full shape; simpler than threading the
        // iterator, and this runs over a handful of small files.
        let tail = &rest[7..];
        let mut end = 0;
        let mut dots = 0;
        for (j, b) in tail.bytes().enumerate() {
            if b == b'.' {
                dots += 1;
                if dots > 2 {
                    break;
                }
            } else if !b.is_ascii_digit() {
                break;
            }
            end = j + 1;
        }
        if dots == 2 && end > 0 && bytes[i + 7 + end - 1].is_ascii_digit() {
            found.push(format!("v{digits}{}", &tail[..end]));
        }
    }
    found
}

/// Files that drive tooling rather than describe it. A literal tag in one of
/// these is an operational dependency on a value that moves.
const OPERATIONAL: &[&str] = &[
    "justfile",
    "site/zola.toml",
    "scripts/download_schemas.sh",
    "scripts/check_codegen_size.sh",
];

#[test]
fn no_operational_file_pins_a_stale_schema_tag() {
    let pinned = pinned_tag();
    let mut files: Vec<String> = OPERATIONAL.iter().map(|s| (*s).to_owned()).collect();

    let workflows = root().join(".github/workflows");
    if let Ok(entries) = std::fs::read_dir(&workflows) {
        for e in entries.filter_map(Result::ok) {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "yml" || x == "yaml") {
                files.push(format!(
                    ".github/workflows/{}",
                    p.file_name().expect("named").to_string_lossy()
                ));
            }
        }
    }
    files.sort();

    let mut stale = Vec::new();
    for rel in &files {
        if !root().join(rel).exists() {
            continue;
        }
        for tag in schema_tags_in(&read(rel)) {
            if tag != pinned {
                stale.push(format!("{rel}: {tag}"));
            }
        }
    }

    assert!(
        stale.is_empty(),
        "these pin a schema tag other than the committed {pinned}: {stale:#?}\n\
         Derive it from `generator/schemas/` instead of writing it out — \
         `just pinned-tag` prints it."
    );
}

#[test]
fn the_site_advertises_the_committed_schema_tag() {
    let cfg = read("site/zola.toml");
    let declared = value_of(&cfg, "schema_version");
    assert_eq!(
        declared,
        pinned_tag(),
        "site/zola.toml advertises a schema version the crate is not generated from; \
         the landing page and every footer show it"
    );
}

#[test]
fn the_site_advertises_the_declared_msrv() {
    let cargo = read("Cargo.toml");
    let msrv = value_of(&cargo, "rust-version");
    let site = value_of(&read("site/zola.toml"), "msrv");
    assert_eq!(
        site, msrv,
        "site/zola.toml advertises an MSRV that Cargo.toml does not declare"
    );

    // The README states it in prose, where a mismatch is just as visible.
    let readme = read("README.md");
    assert!(
        readme.contains(&format!("**{msrv}**")),
        "README's MSRV section does not state {msrv}"
    );
}

/// The `"…"` value of the first `key = "…"` line in a TOML-ish file.
fn value_of(text: &str, key: &str) -> String {
    text.lines()
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim().trim_matches('"').to_owned())
        .unwrap_or_else(|| panic!("no `{key} = \"…\"` line found"))
}

/// The generator's own CLI help and the download script must name a real
/// directory layout, so a tag in either has to be the committed one.
#[test]
fn the_tag_helper_agrees_with_the_snapshot_directory() {
    let justfile = read("justfile");
    assert!(
        justfile.contains("pinned-tag:"),
        "the `pinned-tag` recipe is what every other recipe and the CI workflow \
         derive the tag from"
    );
    assert!(
        Path::new(&root().join("generator/schemas").join(pinned_tag())).is_dir(),
        "the derived tag does not name a directory"
    );
}

/// Each example's `Run with:` line must name exactly the features its
/// `[[example]]` entry requires.
///
/// `required-features` is what cargo enforces; the header comment is what a
/// reader copies. `examples/builder.rs` documented a command cargo refuses —
/// it omitted `decimal`, which the example's own `rust_decimal` use needs — and
/// nothing noticed, because the header is a comment and the manifest is data.
#[test]
fn example_run_commands_match_their_required_features() {
    let manifest = read("Cargo.toml");

    // Each `[[example]]` block is `name = "..."` followed by `required-features = [...]`.
    let mut examples: Vec<(String, Vec<String>)> = Vec::new();
    for block in manifest.split("[[example]]").skip(1) {
        let field = |key: &str| -> Option<&str> {
            block
                .lines()
                .find_map(|l| l.trim().strip_prefix(key))?
                .split_once('=')
                .map(|(_, v)| v.trim())
        };
        let Some(name) = field("name") else { continue };
        let name = name.trim_matches('"').to_owned();
        let features: Vec<String> = field("required-features")
            .unwrap_or("[]")
            .trim_matches(['[', ']'])
            .split(',')
            .map(|f| f.trim().trim_matches('"').to_owned())
            .filter(|f| !f.is_empty())
            .collect();
        examples.push((name, features));
    }
    assert!(
        !examples.is_empty(),
        "no [[example]] blocks parsed out of Cargo.toml"
    );

    for (name, required) in &examples {
        let source = read(&format!("examples/{name}.rs"));
        let line = source
            .lines()
            .find(|l| l.contains("cargo run --example"))
            .unwrap_or_else(|| {
                panic!("examples/{name}.rs has no `cargo run --example` line to check")
            });
        let documented: Vec<String> = line
            .split("--features")
            .nth(1)
            .unwrap_or_else(|| panic!("examples/{name}.rs: the run line names no --features"))
            .trim()
            .split(',')
            .map(|f| f.trim().to_owned())
            .filter(|f| !f.is_empty())
            .collect();

        let mut want = required.clone();
        let mut got = documented.clone();
        want.sort();
        got.sort();
        assert_eq!(
            got,
            want,
            "examples/{name}.rs documents `--features {}` but Cargo.toml requires {required:?} \
             — cargo will refuse the documented command",
            documented.join(","),
        );
    }
}

//! Every JSON entry point must consume its whole input, and must cap nesting.
//!
//! A reader here that accepts what `serde_json::from_str` rejects is a parser
//! differential: the proxy, schema gate, or signature check in front of the
//! service is most likely using plain `serde_json`, and the bytes the two
//! disagree about are the ones only one of them sees.

#![cfg(all(feature = "versioned", feature = "json"))]

use rubo4e::current::Marktlokation;
use rubo4e::json::{Bo4eJsonExt, JsonParseLimits};

/// A well-formed BO4E body, plus whatever is appended to it.
fn with_tail(tail: &str) -> String {
    format!(r#"{{"marktlokationsId":"51238696781"}}{tail}"#)
}

/// The trailing content each entry point has to refuse. Whitespace alone is
/// legal and must still be accepted — that is the line being drawn.
const TRAILING_GARBAGE: &[&str] = &[
    " GARBAGE",
    "{\"marktlokationsId\":\"41373559241\"}", // a second document
    "[1,2,3]",
    "null",
    "\u{0}",
    ",",
];

#[test]
fn german_str_rejects_trailing_content() {
    for tail in TRAILING_GARBAGE {
        let body = with_tail(tail);
        assert!(
            Marktlokation::from_json_german(&body).is_err(),
            "from_json_german accepted {tail:?} after the document"
        );
    }
}

#[test]
fn german_bytes_rejects_trailing_content() {
    for tail in TRAILING_GARBAGE {
        let body = with_tail(tail);
        assert!(
            Marktlokation::from_json_german_bytes(body.as_bytes()).is_err(),
            "from_json_german_bytes accepted {tail:?} after the document"
        );
    }
}

#[test]
fn snake_case_rejects_trailing_content() {
    for tail in TRAILING_GARBAGE {
        let body = format!(r#"{{"marktlokations_id":"51238696781"}}{tail}"#);
        assert!(
            Marktlokation::from_json_snake_case(&body).is_err(),
            "from_json_snake_case accepted {tail:?} after the document"
        );
        assert!(
            Marktlokation::from_json_snake_case_bytes(body.as_bytes()).is_err(),
            "from_json_snake_case_bytes accepted {tail:?} after the document"
        );
    }
}

#[test]
fn the_hardened_paths_reject_trailing_content_too() {
    let limits = JsonParseLimits::untrusted_defaults();
    for tail in TRAILING_GARBAGE {
        let german = with_tail(tail);
        assert!(
            Marktlokation::from_json_german_hardened(&german, limits).is_err(),
            "from_json_german_hardened accepted {tail:?}"
        );
        assert!(
            Marktlokation::from_json_german_bytes_hardened(german.as_bytes(), limits).is_err(),
            "from_json_german_bytes_hardened accepted {tail:?}"
        );

        let snake = format!(r#"{{"marktlokations_id":"51238696781"}}{tail}"#);
        assert!(
            Marktlokation::from_json_snake_case_hardened(&snake, limits).is_err(),
            "from_json_snake_case_hardened accepted {tail:?}"
        );
        assert!(
            Marktlokation::from_json_snake_case_bytes_hardened(snake.as_bytes(), limits).is_err(),
            "from_json_snake_case_bytes_hardened accepted {tail:?}"
        );
    }
}

/// Trailing *whitespace* is part of a well-formed document and must not be
/// caught by the rule above — a pretty-printer's trailing newline is routine.
#[test]
fn trailing_whitespace_is_still_accepted() {
    for tail in [" ", "\n", "\r\n", "\t  \n"] {
        let body = with_tail(tail);
        assert!(
            Marktlokation::from_json_german(&body).is_ok(),
            "trailing whitespace {tail:?} must be accepted"
        );
        assert!(Marktlokation::from_json_german_bytes(body.as_bytes()).is_ok());
    }
}

/// The whole point of the rule: this crate and `serde_json` agree on what is a
/// document, so a gate in front of the service and the service itself cannot
/// disagree.
#[test]
fn acceptance_matches_serde_json_exactly() {
    for tail in TRAILING_GARBAGE.iter().chain([" ", "\n"].iter()) {
        let body = with_tail(tail);
        assert_eq!(
            Marktlokation::from_json_german(&body).is_ok(),
            serde_json::from_str::<Marktlokation>(&body).is_ok(),
            "disagreed with serde_json about {tail:?}"
        );
    }
}

// ─── Nesting depth ───────────────────────────────────────────────────────────

/// `{"a":{"a":{…}}}` nested `depth` levels below the root object.
fn nested(depth: usize) -> String {
    let mut s = String::from(r#"{"marktlokationsId":"51238696781","x":"#);
    for _ in 0..depth {
        s.push_str(r#"{"a":"#);
    }
    s.push('1');
    for _ in 0..depth {
        s.push('}');
    }
    s.push('}');
    s
}

/// The depth cap applies on *every* path, hardened or not.
#[test]
fn every_path_caps_nesting_depth() {
    let deep = nested(400);

    for (name, err) in [
        (
            "from_json_german",
            Marktlokation::from_json_german(&deep).err(),
        ),
        (
            "from_json_german_bytes",
            Marktlokation::from_json_german_bytes(deep.as_bytes()).err(),
        ),
        (
            "from_json_snake_case",
            Marktlokation::from_json_snake_case(&deep).err(),
        ),
        (
            "from_json_snake_case_bytes",
            Marktlokation::from_json_snake_case_bytes(deep.as_bytes()).err(),
        ),
    ] {
        assert!(err.is_some(), "{name} accepted a 400-level payload");
    }
}

/// A hardened call with a *tighter* depth must use it; one that sets no depth
/// still gets the default rather than no cap at all.
#[test]
fn a_hardened_depth_limit_lowers_the_default() {
    let deep = nested(8);

    let tight = JsonParseLimits::unlimited().with_max_nesting_depth(Some(4));
    let err = Marktlokation::from_json_german_hardened(&deep, tight)
        .expect_err("8 levels must exceed a cap of 4");
    assert!(
        err.to_string().contains("nesting depth"),
        "unexpected error: {err}"
    );

    // …and the same payload is fine under the default.
    assert!(Marktlokation::from_json_german_hardened(&deep, JsonParseLimits::unlimited()).is_ok());

    // The snake_case reader honours the same setting.
    let err = Marktlokation::from_json_snake_case_hardened(&deep, tight)
        .expect_err("8 levels must exceed a cap of 4");
    assert!(
        err.to_string().contains("nesting depth"),
        "unexpected error: {err}"
    );
}

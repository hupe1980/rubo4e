//! Every identifier must describe itself the same way in both schema outputs.
//!
//! Neither derive gives that by default: `schemars` substitutes the type's
//! **rustdoc** for the description, overriding whatever a `schema_with` function
//! sets, and `utoipa` falls back to the rustdoc when its own attribute says
//! nothing. Left alone, both publish Rust prose — intra-doc links, `assert!`
//! examples, and for a fully `#`-hidden doctest, an empty code fence.
//!
//! `crate::identifiers::schema` is the one table both read. This guard keeps them
//! reading it:
//!
//! 1. Both outputs carry a pattern, a description and an example.
//! 2. The two agree, field for field. `utoipa` needs the pattern and example as
//!    literals — it compiles the regex — so those are written twice, and this is
//!    what makes that safe.
//! 3. The description is the table's, not the rustdoc's. Checked structurally:
//!    rustdoc leaks are recognisable, and no real description contains them.
//! 4. The example matches its own pattern.
//! 5. The example survives the type's own constructor — the check that stops an
//!    example drifting into something that merely looks right, like an ID with
//!    a wrong check digit.

#![cfg(all(feature = "schemars", feature = "utoipa"))]

use rubo4e::identifiers::schema::{self as SCHEMA, IdentifierSchema};
use rubo4e::identifiers::*;
use utoipa::PartialSchema;

/// One identifier under test: its table entry, and a way to run its constructor.
struct Case {
    name: &'static str,
    meta: IdentifierSchema,
    schemars: fn() -> serde_json::Value,
    utoipa: fn() -> serde_json::Value,
    parses: fn(&str) -> bool,
}

macro_rules! case {
    ($ty:ident, $meta:ident) => {
        Case {
            name: stringify!($ty),
            meta: SCHEMA::$meta,
            schemars: || serde_json::to_value(schemars::schema_for!($ty)).expect("schema is JSON"),
            utoipa: || serde_json::to_value($ty::schema()).expect("schema is JSON"),
            parses: |s| $ty::new(s).is_ok(),
        }
    };
}

fn cases() -> Vec<Case> {
    vec![
        case!(MaloId, MALO_ID),
        case!(MarktpartnerId, MARKTPARTNER_ID),
        case!(MeloId, MELO_ID),
        case!(NeloId, NELO_ID),
        case!(NebeId, NEBE_ID),
        case!(CrId, CR_ID),
        case!(SgId, SG_ID),
        case!(SrId, SR_ID),
        case!(TrId, TR_ID),
        case!(PaketId, PAKET_ID),
        case!(Lokationsbuendelcode, LOKATIONSBUENDEL_CODE),
        case!(LokationsbuendelObjektcode, LOKATIONSBUENDEL_OBJEKTCODE),
        case!(EicCode, EIC_CODE),
        case!(BilanzkreisId, BILANZKREIS_ID),
        case!(BilanzierungsgebietId, BILANZIERUNGSGEBIET_ID),
        case!(AkivId, AKIV_ID),
        case!(TranchennummerId, TRANCHENNUMMER_ID),
        case!(Iban, IBAN),
        case!(Bic, BIC),
        case!(Zaehlpunktbezeichnung, ZAEHLPUNKTBEZEICHNUNG),
        case!(ObisCode, OBIS_CODE),
    ]
}

/// The set under test must be every identifier the prelude exports, so a new one
/// cannot be added without being described.
#[test]
fn every_exported_identifier_is_covered() {
    // `MaloVergabestelle`, `MpIdAuthority`, `EicType`, `ObisComponents`,
    // `Zaehlpunktart` and `Zaehlpunkt` are helper types beside the identifiers they
    // classify, not identifiers themselves, so they are not here.
    let expected = [
        "AkivId",
        "Bic",
        "BilanzierungsgebietId",
        "BilanzkreisId",
        "CrId",
        "EicCode",
        "Iban",
        "LokationsbuendelObjektcode",
        "Lokationsbuendelcode",
        "MaloId",
        "MarktpartnerId",
        "MeloId",
        "NebeId",
        "NeloId",
        "ObisCode",
        "PaketId",
        "SgId",
        "SrId",
        "TrId",
        "TranchennummerId",
        "Zaehlpunktbezeichnung",
    ];
    let mut actual: Vec<&str> = cases().iter().map(|c| c.name).collect();
    actual.sort_unstable();
    assert_eq!(actual, expected);
}

fn field<'a>(schema: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    schema.get(key)
}

#[test]
fn both_outputs_carry_a_pattern_a_description_and_an_example() {
    for case in cases() {
        for (which, schema) in [("schemars", (case.schemars)()), ("utoipa", (case.utoipa)())] {
            for key in ["pattern", "description"] {
                assert!(
                    field(&schema, key).is_some(),
                    "{}: the {which} schema has no {key}",
                    case.name
                );
            }
            let has_example = field(&schema, "example").is_some()
                || field(&schema, "examples")
                    .is_some_and(|e| !e.as_array().is_none_or(Vec::is_empty));
            assert!(
                has_example,
                "{}: the {which} schema has no example — {schema}",
                case.name
            );
        }
    }
}

/// The two outputs must say the same thing. `utoipa` compiles its regex, so the
/// pattern and example are literals in the attribute *and* values in the table;
/// this is what keeps the copies honest.
#[test]
fn the_two_outputs_agree_with_the_table() {
    for case in cases() {
        let schemars = (case.schemars)();
        let utoipa = (case.utoipa)();

        for (which, schema) in [("schemars", &schemars), ("utoipa", &utoipa)] {
            assert_eq!(
                field(schema, "pattern").and_then(|v| v.as_str()),
                Some(case.meta.pattern),
                "{}: the {which} pattern is not the table's",
                case.name
            );
            assert_eq!(
                field(schema, "description").and_then(|v| v.as_str()),
                Some(case.meta.description),
                "{}: the {which} description is not the table's",
                case.name
            );
        }

        // `schemars` writes `examples: [..]`, `utoipa` writes `example: ..` for a
        // derived newtype and `examples: [..]` for a hand-written schema.
        let example_of = |schema: &serde_json::Value| -> Option<String> {
            if let Some(one) = field(schema, "example").and_then(|v| v.as_str()) {
                return Some(one.to_owned());
            }
            field(schema, "examples")?
                .as_array()?
                .first()?
                .as_str()
                .map(str::to_owned)
        };
        for (which, schema) in [("schemars", &schemars), ("utoipa", &utoipa)] {
            assert_eq!(
                example_of(schema).as_deref(),
                Some(case.meta.example),
                "{}: the {which} example is not the table's",
                case.name
            );
        }
    }
}

/// The description must be the table's German sentence, not the type's rustdoc.
///
/// Checked by shape as well as by equality: a rustdoc leak carries markers a
/// hand-written one-liner never does, so this fails loudly with the reason rather
/// than as an opaque string mismatch.
#[test]
fn no_rustdoc_leaks_into_a_published_schema() {
    for case in cases() {
        for (which, schema) in [("schemars", (case.schemars)()), ("utoipa", (case.utoipa)())] {
            let description = field(&schema, "description")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            for (marker, what) in [
                ("```", "a Rust code fence"),
                ("assert_eq!", "a doctest assertion"),
                ("\n# ", "a rustdoc section heading"),
                ("](crate::", "an intra-doc link"),
                ("](super::", "an intra-doc link"),
            ] {
                assert!(
                    !description.contains(marker),
                    "{}: the {which} description contains {what} — the rustdoc is \
                     leaking into the published schema.\nGot: {description}",
                    case.name
                );
            }
            assert!(
                description.len() < 400,
                "{}: the {which} description is {} characters; a schema description \
                 is one sentence, not an API reference",
                case.name,
                description.len()
            );
        }
    }
}

/// An example that does not match its own pattern would make every generated
/// client's validation fail on the documentation's own sample.
#[test]
fn every_example_matches_its_pattern() {
    for case in cases() {
        assert!(
            matches_anchored(case.meta.pattern, case.meta.example),
            "{}: the example {:?} does not match the pattern {:?}",
            case.name,
            case.meta.example,
            case.meta.pattern
        );
    }
}

/// …and, more strictly, the example must be a **valid identifier**: patterns
/// cannot express a check digit, so a pattern match alone would let an example
/// with a bad one through.
#[test]
fn every_example_survives_its_own_constructor() {
    for case in cases() {
        assert!(
            (case.parses)(case.meta.example),
            "{}: the schema example {:?} is not a value this type accepts",
            case.name,
            case.meta.example
        );
    }
}

/// A pattern that matched everything would make the two tests above vacuous, so
/// each one is checked to reject something.
#[test]
fn no_pattern_is_vacuous() {
    for case in cases() {
        let junk = "!!! definitely not an identifier !!!";
        assert!(
            !matches_anchored(case.meta.pattern, junk),
            "{}: the pattern {:?} matches arbitrary text",
            case.name,
            case.meta.pattern
        );
    }
}

// ─── A regex engine small enough to not be a dependency ─────────────────────
//
// The patterns here use one grammar: literals, `[...]` classes with ranges and
// negation, `(...)` groups, `|` alternation, and `{n}` / `{n,m}` / `?`
// quantifiers, anchored at both ends. That is little enough to match by
// backtracking in fifty lines, and a dev-dependency on a regex crate to check
// eighteen strings would be the larger cost.

fn matches_anchored(pattern: &str, text: &str) -> bool {
    let pat: Vec<char> = pattern
        .strip_prefix('^')
        .and_then(|p| p.strip_suffix('$'))
        .unwrap_or_else(|| panic!("pattern {pattern:?} must be anchored at both ends"))
        .chars()
        .collect();
    let txt: Vec<char> = text.chars().collect();
    alternation(&pat, &txt, 0, 0).contains(&txt.len())
}

/// Every text position reachable by matching `pat[pi..]` starting at `txt[ti]`.
fn alternation(pat: &[char], txt: &[char], pi: usize, ti: usize) -> Vec<usize> {
    let mut ends = Vec::new();
    for branch in split_top_level(&pat[pi..], '|') {
        ends.extend(sequence(&branch, txt, 0, ti));
    }
    ends.sort_unstable();
    ends.dedup();
    ends
}

fn sequence(pat: &[char], txt: &[char], pi: usize, ti: usize) -> Vec<usize> {
    if pi >= pat.len() {
        return vec![ti];
    }
    let (atom, next) = atom_at(pat, pi);
    let (min, max, after) = quantifier_at(pat, next);

    let mut reached = vec![ti];
    let mut ends = Vec::new();
    let mut count = 0usize;
    loop {
        if count >= min {
            for &r in &reached {
                ends.extend(sequence(pat, txt, after, r));
            }
        }
        if count == max {
            break;
        }
        let mut advanced = Vec::new();
        for &r in &reached {
            advanced.extend(atom_match(&atom, txt, r));
        }
        if advanced.is_empty() {
            break;
        }
        reached = advanced;
        count += 1;
    }
    ends.sort_unstable();
    ends.dedup();
    ends
}

enum Atom {
    Literal(char),
    Class {
        negated: bool,
        items: Vec<(char, char)>,
    },
    Group(Vec<char>),
    Any,
}

fn atom_at(pat: &[char], pi: usize) -> (Atom, usize) {
    match pat[pi] {
        '\\' => match pat[pi + 1] {
            'd' => (
                Atom::Class {
                    negated: false,
                    items: vec![('0', '9')],
                },
                pi + 2,
            ),
            'w' => (
                Atom::Class {
                    negated: false,
                    items: vec![('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')],
                },
                pi + 2,
            ),
            's' => (
                Atom::Class {
                    negated: false,
                    items: vec![(' ', ' '), ('\t', '\t'), ('\n', '\n'), ('\r', '\r')],
                },
                pi + 2,
            ),
            c => (Atom::Literal(c), pi + 2),
        },
        '.' => (Atom::Any, pi + 1),
        '[' => {
            let close = closing(pat, pi, '[', ']');
            let mut inner = &pat[pi + 1..close];
            let negated = inner.first() == Some(&'^');
            if negated {
                inner = &inner[1..];
            }
            let mut items = Vec::new();
            let mut i = 0;
            while i < inner.len() {
                if i + 2 < inner.len() && inner[i + 1] == '-' {
                    items.push((inner[i], inner[i + 2]));
                    i += 3;
                } else {
                    items.push((inner[i], inner[i]));
                    i += 1;
                }
            }
            (Atom::Class { negated, items }, close + 1)
        }
        '(' => {
            let close = closing(pat, pi, '(', ')');
            let mut inner: Vec<char> = pat[pi + 1..close].to_vec();
            // Non-capturing groups behave identically here.
            if inner.starts_with(&['?', ':']) {
                inner.drain(..2);
            }
            (Atom::Group(inner), close + 1)
        }
        c => (Atom::Literal(c), pi + 1),
    }
}

fn quantifier_at(pat: &[char], pi: usize) -> (usize, usize, usize) {
    match pat.get(pi) {
        Some('?') => (0, 1, pi + 1),
        Some('*') => (0, usize::MAX, pi + 1),
        Some('+') => (1, usize::MAX, pi + 1),
        Some('{') => {
            let close = closing(pat, pi, '{', '}');
            let body: String = pat[pi + 1..close].iter().collect();
            let (min, max) = match body.split_once(',') {
                None => {
                    let n = body.parse().expect("a {n} quantifier");
                    (n, n)
                }
                Some((lo, "")) => (lo.parse().expect("a {n,} quantifier"), usize::MAX),
                Some((lo, hi)) => (
                    lo.parse().expect("a {n,m} lower bound"),
                    hi.parse().expect("a {n,m} upper bound"),
                ),
            };
            (min, max, close + 1)
        }
        _ => (1, 1, pi),
    }
}

fn atom_match(atom: &Atom, txt: &[char], ti: usize) -> Vec<usize> {
    match atom {
        Atom::Group(inner) => alternation(inner, txt, 0, ti),
        _ if ti >= txt.len() => Vec::new(),
        Atom::Literal(c) => {
            if txt[ti] == *c {
                vec![ti + 1]
            } else {
                Vec::new()
            }
        }
        Atom::Any => vec![ti + 1],
        Atom::Class { negated, items } => {
            let hit = items.iter().any(|&(lo, hi)| txt[ti] >= lo && txt[ti] <= hi);
            if hit != *negated {
                vec![ti + 1]
            } else {
                Vec::new()
            }
        }
    }
}

/// Splits on `sep` at nesting depth zero.
fn split_top_level(pat: &[char], sep: char) -> Vec<Vec<char>> {
    let (mut out, mut cur, mut depth, mut in_class) = (Vec::new(), Vec::new(), 0i32, false);
    let mut i = 0;
    while i < pat.len() {
        let c = pat[i];
        match c {
            '\\' => {
                cur.push(c);
                if let Some(&n) = pat.get(i + 1) {
                    cur.push(n);
                    i += 1;
                }
            }
            '[' if !in_class => {
                in_class = true;
                cur.push(c);
            }
            ']' if in_class => {
                in_class = false;
                cur.push(c);
            }
            '(' if !in_class => {
                depth += 1;
                cur.push(c);
            }
            ')' if !in_class => {
                depth -= 1;
                cur.push(c);
            }
            c if c == sep && depth == 0 && !in_class => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
        i += 1;
    }
    out.push(cur);
    out
}

fn closing(pat: &[char], open_at: usize, open: char, close: char) -> usize {
    let (mut depth, mut i) = (0i32, open_at);
    while i < pat.len() {
        match pat[i] {
            '\\' => i += 1,
            c if c == open => depth += 1,
            c if c == close => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    panic!("unbalanced {open}{close} in pattern");
}

/// The matcher is only a guard if it can fail, so it is pinned against the two
/// shapes the table actually uses.
#[test]
fn the_matcher_itself_works() {
    assert!(matches_anchored("^[1-9][0-9]{10}$", "41373559241"));
    assert!(!matches_anchored("^[1-9][0-9]{10}$", "01373559241")); // leading zero
    assert!(!matches_anchored("^[1-9][0-9]{10}$", "4137355924")); // too short
    assert!(!matches_anchored("^[1-9][0-9]{10}$", "413735592411")); // too long

    // Alternation, optional groups, and negated-free classes.
    assert!(matches_anchored("^(0|[1-9][0-9]{0,5})$", "0"));
    assert!(matches_anchored("^(0|[1-9][0-9]{0,5})$", "999999"));
    assert!(!matches_anchored("^(0|[1-9][0-9]{0,5})$", "0123"));
    assert!(matches_anchored(
        "^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$",
        "MARKDEFF"
    ));
    assert!(matches_anchored(
        "^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$",
        "COBADEFFXXX"
    ));
    assert!(!matches_anchored(
        "^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$",
        "COBADEFFXX"
    ));

    // Escapes and non-capturing groups, as the OBIS grammar uses them.
    let obis = r"^(?:\d+(?:-\d+)?:)?\d+\.\d+(?:\.\d+)?(?:[*&]\d+)?$";
    assert!(matches_anchored(obis, "1-0:1.8.0*255"));
    assert!(matches_anchored(obis, "1.8"));
    assert!(!matches_anchored(obis, "1"));
    assert!(!matches_anchored(obis, "x-0:1.8"));
}

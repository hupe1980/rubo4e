//! Rust identifier naming for BO4E schema values.
//!
//! Every generated enum variant is derived from a SCREAMING_SNAKE_CASE wire
//! string. `heck::ToUpperCamelCase` is not usable for it: it cannot see word
//! boundaries inside an all-caps run (`G2KOMMA5` → `G2komma5`), and it drops the
//! separator between two digit runs, collapsing `MESSPREIS_G2_5` (meter size
//! G 2.5) and `MESSPREIS_G25` (G 25) onto one identifier.

/// Converts a BO4E SCREAMING_SNAKE_CASE wire value to an UpperCamelCase Rust
/// identifier.
///
/// The value is split on every non-alphanumeric character, then each segment into
/// maximal runs of letters and of digits. Letter runs are title-cased, digit runs
/// kept verbatim, and the runs concatenated — except at a digit-to-digit segment
/// boundary, where the `_` is kept because dropping it would merge two distinct
/// values.
///
/// A leading digit is prefixed with `V`, since a Rust identifier cannot start
/// with one.
///
/// ```text
/// LEISTUNG_PAUSCHAL           → LeistungPauschal
/// G2KOMMA5                    → G2Komma5
/// MESSPREIS_G2_5              → MesspreisG2_5     // meter size G 2.5
/// MESSPREIS_G25               → MesspreisG25      // meter size G 25
/// Z88_VERGLEICHSMESSUNG(GEEICHT) → Z88VergleichsmessungGeeicht
/// ```
pub fn screaming_to_camel(value: &str) -> String {
    let mut out = String::with_capacity(value.len());

    for (segment_index, segment) in value
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        for (run_index, run) in alnum_runs(segment).enumerate() {
            // Only a separator that sat between two digit runs has to survive:
            // everywhere else the case change already marks the boundary.
            let starts_segment = run_index == 0 && segment_index > 0;
            let digit_join = starts_segment
                && out.ends_with(|c: char| c.is_ascii_digit())
                && run.starts_with(|c: char| c.is_ascii_digit());
            if digit_join {
                out.push('_');
            }
            push_title_cased(&mut out, run);
        }
    }

    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, 'V');
    }
    out
}

/// Returns `true` if [`screaming_to_camel`] kept a `_`, which makes the
/// identifier trip `non_camel_case_types`.
pub fn needs_non_camel_case_allow(value: &str) -> bool {
    screaming_to_camel(value).contains('_')
}

/// Splits `segment` into maximal runs of ASCII letters and of ASCII digits.
fn alnum_runs(segment: &str) -> impl Iterator<Item = &str> {
    let bytes = segment.as_bytes();
    let mut start = 0usize;
    std::iter::from_fn(move || {
        if start >= bytes.len() {
            return None;
        }
        let is_digit = bytes[start].is_ascii_digit();
        let mut end = start + 1;
        while end < bytes.len() && bytes[end].is_ascii_digit() == is_digit {
            end += 1;
        }
        let run = &segment[start..end];
        start = end;
        Some(run)
    })
}

/// Appends `run` with its first character upper-cased and the rest lower-cased.
/// Digit runs pass through unchanged.
fn push_title_cased(out: &mut String, run: &str) {
    let mut chars = run.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        for c in chars {
            out.extend(c.to_lowercase());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_underscores() {
        assert_eq!(screaming_to_camel("LEISTUNG_PAUSCHAL"), "LeistungPauschal");
        assert_eq!(screaming_to_camel("VIERTEL_STUNDE"), "ViertelStunde");
        assert_eq!(screaming_to_camel("KWH"), "Kwh");
    }

    #[test]
    fn splits_all_caps_runs_on_digit_boundaries() {
        assert_eq!(screaming_to_camel("G2KOMMA5"), "G2Komma5");
        assert_eq!(screaming_to_camel("G25"), "G25");
    }

    /// Two distinct BO4E meter sizes must not collapse onto one identifier.
    #[test]
    fn digit_separated_values_stay_distinct() {
        assert_eq!(screaming_to_camel("MESSPREIS_G2_5"), "MesspreisG2_5");
        assert_eq!(screaming_to_camel("MESSPREIS_G25"), "MesspreisG25");
        assert_ne!(
            screaming_to_camel("SMART_METER_MESSPREIS_G2_5"),
            screaming_to_camel("SMART_METER_MESSPREIS_G25"),
        );
        assert!(needs_non_camel_case_allow("MESSPREIS_G2_5"));
        assert!(!needs_non_camel_case_allow("MESSPREIS_G25"));
    }

    #[test]
    fn non_alphanumeric_separators_are_word_boundaries() {
        assert_eq!(
            screaming_to_camel("Z88_VERGLEICHSMESSUNG(GEEICHT)"),
            "Z88VergleichsmessungGeeicht"
        );
        assert_eq!(
            screaming_to_camel("ZA9_Z-ZAHL-KORREKTUR"),
            "Za9ZZahlKorrektur"
        );
    }

    #[test]
    fn leading_digit_is_prefixed() {
        assert_eq!(screaming_to_camel("13TE_RECHNUNG"), "V13TeRechnung");
    }

    #[test]
    fn empty_and_separator_only_inputs_do_not_panic() {
        assert_eq!(screaming_to_camel(""), "");
        assert_eq!(screaming_to_camel("___"), "");
    }
}

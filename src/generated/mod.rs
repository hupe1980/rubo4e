// @generated — do not edit by hand.
// This file is maintained by the code generator (`just generate`).

// `key_map` is gated on `json` rather than `versioned`: the JSON key
// transforms need it whenever the `json` entry points are compiled, and
// `json` does not imply `versioned`.
#[cfg(feature = "json")]
pub(crate) mod key_map;

#[cfg(feature = "versioned")]
pub mod v202607;

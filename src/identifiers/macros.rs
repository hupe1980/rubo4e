//! Macros that stamp out the parts of an identifier newtype which are identical
//! for every type in this module.
//!
//! Each identifier is a `Box<str>` newtype whose only interesting piece is its
//! `validate` function. Everything else — the conversion traits, the wire-format
//! traits, and the `serde` impls — is mechanical. Writing those by hand once per
//! type produced ~60 lines of duplicated code per identifier and made it easy for
//! a type to silently drift (e.g. gain a `Deserialize` that skips validation).
//!
//! [`impl_identifier_traits!`] generates that shared surface, and
//! [`bdew_ascii_identifier!`] generates a whole §8.2 ASCII-Verfahren identifier
//! (NeLo-ID, NeBe-ID, TR/SR/SG/CR-ID, Paket-ID), which differ only in their
//! Codetyp prefix and documentation.

/// Implements the conversion, wire-format, and `serde` traits shared by every
/// identifier newtype.
///
/// The type must be a single-field tuple struct wrapping `Box<str>` and provide
/// an inherent `fn new(&str) -> Result<Self, IdentifierError>` that performs all
/// validation. Deserialization routes through `new`, so a value that exists can
/// always be trusted to have been validated.
macro_rules! impl_identifier_traits {
    // Tuple-struct form: the string lives in field `.0`.
    ($ty:ident, $expecting:expr) => {
        impl_identifier_traits!($ty, $expecting, field = 0);
    };
    // Named-field form, for identifiers that also cache parsed data alongside
    // the string (e.g. `ObisCode`).
    ($ty:ident, $expecting:expr, field = $field:tt) => {
        impl TryFrom<String> for $ty {
            type Error = $crate::error::IdentifierError;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::new(&s)
            }
        }

        impl TryFrom<&str> for $ty {
            type Error = $crate::error::IdentifierError;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::new(s)
            }
        }

        impl AsRef<str> for $ty {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.$field
            }
        }

        impl std::borrow::Borrow<str> for $ty {
            #[inline]
            fn borrow(&self) -> &str {
                &self.$field
            }
        }

        impl std::ops::Deref for $ty {
            type Target = str;
            #[inline]
            fn deref(&self) -> &str {
                &self.$field
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.$field)
            }
        }

        impl std::str::FromStr for $ty {
            type Err = $crate::error::IdentifierError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s)
            }
        }

        impl From<$ty> for String {
            fn from(id: $ty) -> String {
                String::from(id.$field)
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.$field)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl serde::de::Visitor<'_> for Visitor {
                    type Value = $ty;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str($expecting)
                    }
                    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<$ty, E> {
                        $ty::new(v).map_err(|e| {
                            $crate::identifiers::trace_identifier_deser_error(
                                stringify!($ty),
                                v,
                                &e,
                            );
                            serde::de::Error::custom(e)
                        })
                    }
                }
                d.deserialize_str(Visitor)
            }
        }
    };
}

/// Defines a complete BDEW §8.2 "ASCII-Verfahren" identifier newtype.
///
/// These identifiers are all 11 characters: a fixed Codetyp prefix, an
/// uppercase-alphanumeric body, and a numeric check digit at position 11. They
/// differ only in the prefix, so everything else is generated here.
///
/// Parameters:
/// - `$ty` — the newtype name.
/// - `$prefix` — the Codetyp byte string (e.g. `b"E"`, `b"P9"`).
/// - `$schema_fn` — path to the `schemars` schema function.
/// - `$expecting` — the `serde` "expecting" message.
/// - `$example_base` / `$example_full` — a doctest vector; `$example_full` must be
///   `$example_base` plus its check digit.
macro_rules! bdew_ascii_identifier {
    (
        $(#[$meta:meta])*
        $ty:ident,
        prefix     = $prefix:expr,
        schema     = $schema_fn:literal,
        expecting  = $expecting:expr,
        example    = ($example_base:literal, $example_full:literal),
        check_fn   = $check_fn:ident $(,)?
    ) => {
        $(#[$meta])*
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use rubo4e::identifiers::", stringify!($ty), ";")]
        ///
        #[doc = concat!("let id = ", stringify!($ty), "::new(\"", $example_full, "\").unwrap();")]
        #[doc = concat!("assert_eq!(id.to_string(), \"", $example_full, "\");")]
        ///
        /// // The check digit is derived, so it never has to be typed by hand:
        #[doc = concat!("let id = ", stringify!($ty), "::from_base(\"", $example_base, "\").unwrap();")]
        #[doc = concat!("assert_eq!(id.as_ref(), \"", $example_full, "\");")]
        /// ```
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "validate", derive(garde::Validate))]
        #[cfg_attr(feature = "validate", garde(allow_unvalidated))]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schemars", schemars(schema_with = $schema_fn))]
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[cfg_attr(feature = "utoipa", schema(value_type = String))]
        pub struct $ty(#[cfg_attr(feature = "validate", garde(custom($check_fn)))] Box<str>);

        #[cfg(feature = "validate")]
        fn $check_fn(value: &str, _: &()) -> Result<(), garde::Error> {
            $crate::identifiers::checksum::validate_ascii_id(value, $prefix)
                .map_err(garde::Error::from)
        }

        impl $ty {
            /// The fixed Codetyp prefix for this identifier type.
            pub const CODETYP: &'static str = match std::str::from_utf8($prefix) {
                Ok(s) => s,
                Err(_) => panic!("Codetyp prefix must be valid UTF-8"),
            };

            #[doc = concat!("Creates a new `", stringify!($ty), "` after full validation.")]
            ///
            /// # Errors
            /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 11 characters.
            #[doc = concat!("- [`IdentifierError::InvalidFormat`] if `s` does not start with `\"", $example_base, "\"`'s Codetyp.")]
            /// - [`IdentifierError::InvalidCharacter`] if the body is not `[A-Z0-9]`,
            ///   or position 11 is not a decimal digit.
            /// - [`IdentifierError::InvalidChecksum`] if position 11 does not match the
            ///   ASCII-Verfahren check digit computed from positions 1–10.
            ///
            /// [`IdentifierError::InvalidLength`]: crate::error::IdentifierError::InvalidLength
            /// [`IdentifierError::InvalidFormat`]: crate::error::IdentifierError::InvalidFormat
            /// [`IdentifierError::InvalidCharacter`]: crate::error::IdentifierError::InvalidCharacter
            /// [`IdentifierError::InvalidChecksum`]: crate::error::IdentifierError::InvalidChecksum
            #[must_use = "the validated identifier is returned; ignoring it discards the result"]
            pub fn new(s: &str) -> Result<Self, $crate::error::IdentifierError> {
                $crate::identifiers::checksum::validate_ascii_id(s, $prefix)?;
                Ok(Self(Box::from(s)))
            }

            #[doc = concat!("Builds a `", stringify!($ty), "` from its 10-character base by computing")]
            /// and appending the ASCII-Verfahren check digit.
            ///
            /// # Errors
            /// Same as [`new`](Self::new), minus the checksum error — the check digit
            /// is computed rather than verified.
            pub fn from_base(base: &str) -> Result<Self, $crate::error::IdentifierError> {
                let full =
                    $crate::identifiers::checksum::compute_ascii_id_from_base(base, $prefix)?;
                Ok(Self(full.into_boxed_str()))
            }

            /// Computes the ASCII-Verfahren check digit (`0`–`9`) for a 10-character
            /// base without constructing the identifier.
            ///
            /// # Errors
            /// Same as [`from_base`](Self::from_base).
            pub fn check_digit(base: &str) -> Result<u8, $crate::error::IdentifierError> {
                let full =
                    $crate::identifiers::checksum::compute_ascii_id_from_base(base, $prefix)?;
                Ok(full.as_bytes()[10] - b'0')
            }

            /// Returns the 10-character base (everything except the check digit).
            #[must_use]
            pub fn base(&self) -> &str {
                &self.0[..10]
            }
        }

        impl_identifier_traits!($ty, $expecting);
    };
}

/// Defines an identifier newtype that is an [`EicCode`] pinned to one ENTSO-E
/// object type.
///
/// The German market reuses the EIC namespace for several distinct roles that
/// differ *only* in the position-3 object-type character — a Bilanzkreis is
/// `11X…` (party), a Bilanzierungsgebiet is `11Y…` (area).  Each gets its own
/// Rust type so the two cannot be swapped at a call site, and everything except
/// the pinned character and the documentation is generated here.
///
/// Parameters:
/// - `$ty` — the newtype name.
/// - `$eic_type` — the [`EicType`] variant this identifier is restricted to.
/// - `$schema_fn` — path to the `schemars` schema function.
/// - `$expecting` — the `serde` "expecting" message.
/// - `$example` — a real 16-character code used in the generated doctest.
///
/// [`EicCode`]: crate::identifiers::EicCode
/// [`EicType`]: crate::identifiers::EicType
macro_rules! eic_restricted_identifier {
    (
        $(#[$meta:meta])*
        $ty:ident,
        eic_type   = $eic_type:expr,
        schema     = $schema_fn:literal,
        expecting  = $expecting:expr,
        example    = $example:literal,
        check_fn   = $check_fn:ident $(,)?
    ) => {
        $(#[$meta])*
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use rubo4e::identifiers::{", stringify!($ty), ", EicCode, EicType};")]
        ///
        #[doc = concat!("let id = ", stringify!($ty), "::new(\"", $example, "\").unwrap();")]
        #[doc = concat!("assert_eq!(id.to_string(), \"", $example, "\");")]
        #[doc = concat!("assert_eq!(", stringify!($ty), "::EIC_TYPE, ", stringify!($eic_type), ");")]
        ///
        /// // Widening to the general EIC type is infallible.
        /// let eic: EicCode = id.into();
        #[doc = concat!("assert_eq!(eic.eic_type(), ", stringify!($eic_type), ");")]
        /// ```
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "validate", derive(garde::Validate))]
        #[cfg_attr(feature = "validate", garde(allow_unvalidated))]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schemars", schemars(schema_with = $schema_fn))]
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[cfg_attr(feature = "utoipa", schema(value_type = String))]
        pub struct $ty(#[cfg_attr(feature = "validate", garde(custom($check_fn)))] Box<str>);

        #[cfg(feature = "validate")]
        fn $check_fn(value: &str, _: &()) -> Result<(), garde::Error> {
            $ty::validate(value).map_err(garde::Error::from)
        }

        impl $ty {
            /// The ENTSO-E object type every value of this type carries in position 3.
            pub const EIC_TYPE: $crate::identifiers::EicType = $eic_type;

            fn validate(s: &str) -> Result<(), $crate::error::IdentifierError> {
                // Full EIC validation first: length, alphabet, object type, check character.
                let eic = $crate::identifiers::EicCode::new(s)?;
                if eic.eic_type() != Self::EIC_TYPE {
                    return Err($crate::error::IdentifierError::InvalidFormat {
                        description: format!(
                            "{} requires EIC object type '{}' ({}) at position 3, found '{}' ({})",
                            stringify!($ty),
                            Self::EIC_TYPE.as_char(),
                            Self::EIC_TYPE.description(),
                            eic.type_char(),
                            eic.eic_type().description(),
                        )
                        .into(),
                    });
                }
                Ok(())
            }

            #[doc = concat!("Creates a new `", stringify!($ty), "` after full EIC validation,")]
            #[doc = concat!("requiring object type `", stringify!($eic_type), "` at position 3.")]
            ///
            /// # Errors
            /// - [`IdentifierError::InvalidLength`] if `s` is not exactly 16 characters.
            /// - [`IdentifierError::InvalidCharacter`] if any character is outside `[A-Z0-9-]`.
            /// - [`IdentifierError::InvalidFormat`] if position 3 is not this type's
            ///   object-type character.
            /// - [`IdentifierError::InvalidChecksum`] if position 16 is not the correct
            ///   ENTSO-E check character.
            ///
            /// [`IdentifierError::InvalidLength`]: crate::error::IdentifierError::InvalidLength
            /// [`IdentifierError::InvalidFormat`]: crate::error::IdentifierError::InvalidFormat
            /// [`IdentifierError::InvalidCharacter`]: crate::error::IdentifierError::InvalidCharacter
            /// [`IdentifierError::InvalidChecksum`]: crate::error::IdentifierError::InvalidChecksum
            #[must_use = "the validated identifier is returned; ignoring it discards the result"]
            pub fn new(s: &str) -> Result<Self, $crate::error::IdentifierError> {
                Self::validate(s)?;
                Ok(Self(Box::from(s)))
            }

            #[doc = concat!("Builds a `", stringify!($ty), "` from its 15-character prefix by")]
            /// computing and appending the ENTSO-E check character.
            ///
            /// # Errors
            /// - [`IdentifierError::InvalidLength`] if `prefix` is not exactly 15 characters.
            /// - [`IdentifierError::InvalidFormat`] if `prefix` is not ASCII or position 3
            ///   is not this type's object-type character.
            /// - [`IdentifierError::InvalidChecksum`] if the check character cannot be
            ///   computed (ENTSO-E prohibits `'-'` as a check character).
            ///
            /// [`IdentifierError::InvalidLength`]: crate::error::IdentifierError::InvalidLength
            /// [`IdentifierError::InvalidFormat`]: crate::error::IdentifierError::InvalidFormat
            /// [`IdentifierError::InvalidChecksum`]: crate::error::IdentifierError::InvalidChecksum
            pub fn from_prefix(prefix: &str) -> Result<Self, $crate::error::IdentifierError> {
                let full = $crate::identifiers::EicCode::complete_prefix(prefix)?;
                Self::new(&full)
            }

            /// Returns this value as a general [`EicCode`](crate::identifiers::EicCode).
            #[must_use]
            pub fn to_eic_code(&self) -> $crate::identifiers::EicCode {
                $crate::identifiers::EicCode::new(&self.0)
                    .expect(concat!(stringify!($ty), " is always a valid EicCode"))
            }
        }

        impl From<$ty> for $crate::identifiers::EicCode {
            fn from(id: $ty) -> Self {
                id.to_eic_code()
            }
        }

        impl TryFrom<$crate::identifiers::EicCode> for $ty {
            type Error = $crate::error::IdentifierError;
            fn try_from(eic: $crate::identifiers::EicCode) -> Result<Self, Self::Error> {
                Self::new(eic.as_ref())
            }
        }

        impl_identifier_traits!($ty, $expecting);
    };
}

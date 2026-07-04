//! SQLx integration for BO4E identifier types.
//!
//! Implements [`sqlx::Type`], [`sqlx::Encode`], and [`sqlx::Decode`] for all
//! identifier newtypes, targeting PostgreSQL.  All implementations are gated
//! behind the `sqlx` feature flag.
//!
//! Identifiers are stored and retrieved as `TEXT`.  Decoding runs the same
//! validation as [`TryFrom<String>`], so an invalid value stored in the
//! database returns a `BoxDynError` wrapping [`IdentifierError`](crate::error::IdentifierError).

/// Stamps out `sqlx::Type + Encode + Decode` for a newtype that wraps a
/// validated string and implements `TryFrom<String>` + `AsRef<str>`.
macro_rules! impl_sqlx_text {
    ($($T:ty),+ $(,)?) => {$(
        impl sqlx::Type<sqlx::Postgres> for $T {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <String as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $T {
            fn encode_by_ref(
                &self,
                buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                let s: &str = self.as_ref();
                <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $T {
            fn decode(
                value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                Self::try_from(s).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
            }
        }
    )+};
}

use crate::identifiers::{EicCode, MaloId, MarktpartnerId, MeloId, NeloId, ObisCode, SrId, TrId};

impl_sqlx_text!(
    MaloId,
    MeloId,
    NeloId,
    SrId,
    TrId,
    EicCode,
    MarktpartnerId,
    ObisCode
);

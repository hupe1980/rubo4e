//! SQLx integration for BO4E identifier types.
//!
//! ## What is implemented
//!
//! When the `sqlx` feature is enabled the following traits are implemented for
//! **all** identifier types (`MaloId`, `MeloId`, `NeloId`, `SrId`, `TrId`,
//! `EicCode`, `MarktpartnerId`, `ObisCode`):
//!
//! | Trait | Effect |
//! |-------|--------|
//! | `sqlx::Type<Postgres>` | maps to PostgreSQL `TEXT` |
//! | `sqlx::Encode<'_, Postgres>` | binds as `&str` (zero-copy) |
//! | `sqlx::Decode<'_, Postgres>` | reads `TEXT`, validates, returns typed ID |
//!
//! ## Usage
//!
//! Use the identifier type directly as a bind parameter or result column:
//!
//! ```rust,ignore
//! use rubo4e::identifiers::{MaloId, MarktpartnerId};
//!
//! // As a query bind parameter:
//! sqlx::query("INSERT INTO malo (id) VALUES ($1)")
//!     .bind(&malo_id)          // MaloId implements Encode
//!     .execute(&pool).await?;
//!
//! // As a result column via try_get:
//! let id: MaloId = row.try_get("malo_id")?;
//! let mp: MarktpartnerId = row.try_get("mp_id")?;
//!
//! // As a struct field in query_as!:
//! #[derive(sqlx::FromRow)]
//! struct MpRow {
//!     mp_id: MarktpartnerId,   // decoded + validated automatically
//! }
//! let rows: Vec<MpRow> = sqlx::query_as("SELECT mp_id FROM parties")
//!     .fetch_all(&pool).await?;
//! ```
//!
//! ## Error behaviour
//!
//! Decoding validates the value using the same rules as `TryFrom<String>`.
//! An invalid value stored in the database (e.g. a MaLo-ID with a wrong check
//! digit) causes `row.try_get(...)` to return `Err(...)` wrapping an
//! [`IdentifierError`](crate::error::IdentifierError).
//!
//! ## `PgHasArrayType`
//!
//! `PgHasArrayType` is **not** implemented — PostgreSQL array columns (`TEXT[]`)
//! are not needed for energy-market identifiers and the blanket impl would pull
//! in additional SQLx internals.  If you need array support, implement it locally:
//!
//! ```rust,ignore
//! impl sqlx::postgres::PgHasArrayType for MaloId {
//!     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//!         <String as sqlx::postgres::PgHasArrayType>::array_type_info()
//!     }
//! }
//! ```

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

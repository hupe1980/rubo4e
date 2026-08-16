//! SQLx integration for BO4E identifier types.
//!
//! ## What is implemented
//!
//! When the `sqlx` feature is enabled the following traits are implemented for
//! **every** identifier type in [`crate::identifiers`]:
//!
//! | Trait | Effect |
//! |-------|--------|
//! | `sqlx::Type<Postgres>` | maps to PostgreSQL `TEXT` |
//! | `sqlx::Encode<'_, Postgres>` | binds as `&str` (zero-copy) |
//! | `sqlx::Decode<'_, Postgres>` | reads `TEXT`, validates, returns typed ID |
//! | `sqlx::postgres::PgHasArrayType` | maps `Vec<Id>` to a `TEXT[]` column |
//!
//! ## Usage
//!
//! Use the identifier type directly as a bind parameter or result column:
//!
//! ```no_run
//! use rubo4e::identifiers::{MaloId, MarktpartnerId};
//! use sqlx::Row as _;
//!
//! # async fn demo(pool: sqlx::PgPool, malo_id: MaloId) -> Result<(), sqlx::Error> {
//! // As a query bind parameter:
//! sqlx::query("INSERT INTO malo (id) VALUES ($1)")
//!     .bind(&malo_id)          // MaloId implements Encode
//!     .execute(&pool).await?;
//!
//! // As a result column via try_get:
//! let row = sqlx::query("SELECT malo_id, mp_id FROM parties LIMIT 1")
//!     .fetch_one(&pool).await?;
//! let id: MaloId = row.try_get("malo_id")?;
//! let mp: MarktpartnerId = row.try_get("mp_id")?;
//!
//! // As a struct field with FromRow:
//! #[derive(sqlx::FromRow)]
//! struct MpRow {
//!     mp_id: MarktpartnerId,   // decoded + validated automatically
//! }
//! let rows: Vec<MpRow> = sqlx::query_as("SELECT mp_id FROM parties")
//!     .fetch_all(&pool).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error behaviour
//!
//! Decoding validates the value using the same rules as `TryFrom<String>`.
//! An invalid value stored in the database (e.g. a MaLo-ID with a wrong check
//! digit) causes `row.try_get(...)` to return `Err(...)` wrapping an
//! [`IdentifierError`](crate::error::IdentifierError).
//!
//! ## Array columns
//!
//! `PgHasArrayType` is implemented for every identifier, so `Vec<MaloId>` binds
//! to a `TEXT[]` column directly:
//!
//! ```no_run
//! use rubo4e::identifiers::MaloId;
//!
//! # async fn demo(pool: sqlx::PgPool, ids: Vec<MaloId>) -> Result<(), sqlx::Error> {
//! sqlx::query("SELECT * FROM malo WHERE id = ANY($1)")
//!     .bind(&ids)
//!     .fetch_all(&pool).await?;
//! # Ok(())
//! # }
//! ```
//!
//! This has to live here rather than in downstream code: both `PgHasArrayType`
//! and the identifier types are foreign to any consuming crate, so the orphan
//! rule makes a local impl impossible.

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

        // Lets `Vec<$T>` bind to a `TEXT[]` column.  Only this crate can provide
        // it: the trait and the type are both foreign to any consumer, so the
        // orphan rule rules out a downstream impl.
        impl sqlx::postgres::PgHasArrayType for $T {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <String as sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    )+};
}

use crate::identifiers::{
    AkivId, BilanzierungsgebietId, BilanzkreisId, CrId, EicCode, MaloId, MarktpartnerId, MeloId,
    NebeId, NeloId, ObisCode, PaketId, SgId, SrId, TrId, TranchennummerId,
};

impl_sqlx_text!(
    AkivId,
    BilanzierungsgebietId,
    BilanzkreisId,
    CrId,
    EicCode,
    MaloId,
    MarktpartnerId,
    MeloId,
    NebeId,
    NeloId,
    ObisCode,
    PaketId,
    SgId,
    SrId,
    TrId,
    TranchennummerId,
);

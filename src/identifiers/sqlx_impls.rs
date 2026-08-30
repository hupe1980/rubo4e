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
//! Bind and read identifiers directly — no `.parse()` step, no `as _` override:
//!
//! ```no_run
//! use rubo4e::identifiers::{MaloId, MarktpartnerId};
//! use sqlx::Row as _;
//!
//! # async fn demo(pool: sqlx::PgPool, malo_id: MaloId, ids: Vec<MaloId>) -> Result<(), sqlx::Error> {
//! sqlx::query("INSERT INTO malo (id) VALUES ($1)")
//!     .bind(&malo_id)
//!     .execute(&pool).await?;
//!
//! let row = sqlx::query("SELECT malo_id FROM parties LIMIT 1")
//!     .fetch_one(&pool).await?;
//! let id: MaloId = row.try_get("malo_id")?;   // validated on decode
//!
//! // `Vec<Id>` binds to a `TEXT[]` column.
//! sqlx::query("SELECT * FROM malo WHERE id = ANY($1)")
//!     .bind(&ids)
//!     .fetch_all(&pool).await?;
//!
//! #[derive(sqlx::FromRow)]
//! struct MpRow {
//!     mp_id: MarktpartnerId,
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Error behaviour
//!
//! Decoding validates the value using the same rules as `TryFrom<&str>` — the
//! constructor, so a decoded identifier is as trustworthy as a constructed one.
//! An invalid value stored in the database (e.g. a MaLo-ID with a wrong check
//! digit) causes `row.try_get(...)` to return `Err(...)` wrapping an
//! [`IdentifierError`](crate::error::IdentifierError).
//!
//! ## Array columns
//!
//! `Vec<MaloId>` binds to a `TEXT[]` column directly, so `= ANY($1)` works.
//! `PgHasArrayType` has to be implemented here rather than downstream: both it
//! and the identifier types are foreign to any consuming crate, so the orphan
//! rule rules out a local impl.

/// Stamps out `sqlx::Type + Encode + Decode` for a newtype that wraps a
/// validated string and implements `TryFrom<&str>` + `AsRef<str>`.
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
                // Borrow out of the row buffer rather than decoding a `String`
                // first: the constructor copies into a `Box<str>` either way, so
                // the owned intermediate is pure overhead. This also matches the
                // enum impls the generator emits.
                let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
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
    AkivId, Bic, BilanzierungsgebietId, BilanzkreisId, CrId, EicCode, Iban,
    LokationsbuendelObjektcode, Lokationsbuendelcode, MaloId, MarktpartnerId, MeloId, NebeId,
    NeloId, ObisCode, PaketId, SgId, SrId, TrId, TranchennummerId, Zaehlpunktbezeichnung,
};

impl_sqlx_text!(
    AkivId,
    Bic,
    BilanzierungsgebietId,
    BilanzkreisId,
    CrId,
    EicCode,
    Iban,
    Lokationsbuendelcode,
    LokationsbuendelObjektcode,
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
    Zaehlpunktbezeichnung,
);

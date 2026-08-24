//! Compile-time surface tests for the `sqlx` feature.
//!
//! Nothing here talks to a database; the point is that every trait impl the docs
//! promise actually exists, for identifiers and enums alike.
//!
//! ```text
//! cargo test --test sqlx_compile --features sqlx,versioned
//! ```

#![cfg(all(feature = "sqlx", feature = "versioned"))]

use sqlx::postgres::PgHasArrayType;
use sqlx::{Decode, Encode, Postgres, Type};

fn assert_column<T>()
where
    T: Type<Postgres> + for<'q> Encode<'q, Postgres> + for<'r> Decode<'r, Postgres>,
{
}

fn assert_array_column<T: PgHasArrayType>() {}

#[test]
fn identifiers_bind_as_columns_and_arrays() {
    use rubo4e::identifiers::*;

    macro_rules! check {
        ($($T:ty),+ $(,)?) => {$(
            assert_column::<$T>();
            assert_array_column::<$T>();
        )+};
    }
    check!(
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
}

/// Enums get the same surface as identifiers, arrays included: `Vec<Sparte>`
/// binds to a `TEXT[]` column the way `Vec<MaloId>` does.
#[test]
fn enums_bind_as_columns_and_arrays() {
    use rubo4e::current::*;

    macro_rules! check {
        ($($T:ty),+ $(,)?) => {$(
            assert_column::<$T>();
            assert_array_column::<$T>();
        )+};
    }
    check!(
        BoTyp,
        ComTyp,
        Sparte,
        Marktrolle,
        Zaehlertyp,
        Waehrungscode,
        Mengeneinheit,
        BdewArtikelnummer,
        Messpreistyp,
    );
}

/// The `sqlx` feature must stand on its own: both directions round-trip through
/// `&str` via `as_wire` / `from_wire`, so nothing here needs `json`.
#[test]
fn sqlx_surface_needs_no_json_feature() {
    use rubo4e::current::Sparte;

    assert_eq!(Sparte::Strom.as_wire(), "STROM");
    assert_eq!(Sparte::from_wire("STROM"), Ok(Sparte::Strom));
    // Decode is lenient by design, matching the serde path.
    assert!(Sparte::from_wire("PLASMA").is_err());
}

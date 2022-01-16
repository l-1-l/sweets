// use crate::models::{AggregateRoot, Id};
pub use deadpool_postgres::{Client, Pool, PoolError};
pub use rust_decimal::Decimal;
pub use tokio_postgres::{Error as PgError, Row};

// pub trait FromRows {
//     fn from_rows(rows: Vec<Row>) -> Result<Vec<Self>>
//     where
//         Self: Sized + TryFrom<Row>,
//         // tokio_postgres::Error: std::convert::From<
//         //     <Self as std::convert::TryFrom<tokio_postgres::Row>>::Error,
//         // >,
//     {
//         let iter = rows.into_iter();
//         let mut v = Vec::with_capacity(iter.len());
//         for row in iter.into_iter() {
//             v.push(Self::try_from(row)?);
//         }

//         Ok(v)
//     }
// }

pub fn map_rows<T, E>(
    rows: Vec<Row>,
    f: fn(Row) -> Result<T, E>,
) -> Result<Vec<T>, E> {
    let iter = rows.into_iter();
    let mut v = Vec::with_capacity(iter.len());
    for row in iter.into_iter() {
        v.push(f(row)?)
    }

    Ok(v)
}

// impl TryFrom<&Row> for AggregateRoot {
//     type Error = PgError;

//     fn try_from(row: &Row) -> Result<Self, Self::Error> {
//         let id: Vec<u8> = row.try_get("id")?;
//         let created_at: DateTime<Utc> = row.try_get("created_at")?;
//         let updated_at: Option<DateTime<Utc>> = row.try_get("updated_at")?;

//         Ok(AggregateRoot::build(Id::from(id), created_at, updated_at))
//     }
// }

// pub fn get_id<'a>(r: &'a Row) -> impl Fn(&'a str) -> Result<Id, PgError> {
//     move |idx| Ok(Id::from(r.try_get::<&str, Vec<u8>>(idx)?))
// }

// pub fn try_get_id<'a>(
//     r: &'a Row,
// ) -> impl Fn(&'a str) -> Result<Option<Id>, PgError> {
//     move |idx| Ok(r.try_get::<&str, Option<Vec<u8>>>(idx)?.map(Id::from))
// }

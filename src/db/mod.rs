#[allow(dead_code)]
mod db_home;
use actix_web::{error, web, Error};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type DbEntityResult = Result<Vec<DbEntity>, rusqlite::Error>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum DbEntity {
    /*
    AnnualAgg { year: i32, total: f64 },
    MonthAgg { year: i32, month: i32, total: f64 },
    */
    Genre(String),
}

#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetGenres,
}

// TODO: since instead of failure I prefere to have some defaults and logging this
// method must return stright error from db. Another point do I even need such a complex
// approach for handling db requests? Maybe do simple async methods for each req?
pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<DbEntity>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(|| {
        match query {
            Queries::GetGenres =>  db_home::get_genres(conn),
        }
    }
).await?.map_err(error::ErrorInternalServerError)
}
mod db_home;
use actix_web::{error, web, Error};
// use rusqlite::Statement;
use r2d2_sqlite::{self, SqliteConnectionManager};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type DbEntityResult = Result<Vec<DbEntity>, rusqlite::Error>;

// TODO: need my custom types here
#[derive(Debug, Serialize, Deserialize)]
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

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<DbEntity>, Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        match query {
            Queries::GetGenres => db_home::get_genres(conn),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)

}
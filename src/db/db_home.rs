use crate::db::{Connection, DbEntity, DbEntityResult};

pub fn get_genres(conn: Connection) -> DbEntityResult {
    let mut stmt = conn.prepare(
        "SELECT Name FROM genres ORDER BY Name",
    )?;

    // couldn't use provided methods complaining when use get.
    // let mut rows = stmt.query_map([], |row| row.get::<String>(0i64)?);
    // let res: Vec<String> = rows.mapped(|row| row.get(0)? as String).collect();
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(row) = rows.next()? {
        res.push(row.get(0)?);
    }
    Ok(DbEntity::Genre(res))
}

#[cfg(test)]
mod db_home_test {
use super::*;
use r2d2_sqlite::{self, SqliteConnectionManager};
use crate::db::{Pool, DbEntity};

fn get_pool() -> Pool {
    let manager = SqliteConnectionManager::file("/home/ferrislav/projects/sql/sqlite/chinook/chinook.db");
    Pool::new(manager).unwrap()
}

#[test]
fn get_genres_test(){
    let pool = get_pool();
    let conn = pool.get().unwrap();
    let res = get_genres(conn).unwrap();
    // instead of writing long vec probably should rather take few items by index and check. 
    let expect = vec![
        "Alternative".to_string(),
        "Alternative & Punk".to_string(),
        "Blues".to_string(),
        "Bossa Nova".to_string(),
        "Classical".to_string(),
        "Comedy".to_string(),
        "Drama".to_string(),
        "Easy Listening".to_string(),
        "Electronica/Dance".to_string(),
        "Heavy Metal".to_string(),
        "Hip Hop/Rap".to_string(),
        "Jazz".to_string(),
        "Latin".to_string(),
        "Metal".to_string(),
        "Opera".to_string(),
        "Pop".to_string(),
        "R&B/Soul".to_string(),
        "Reggae".to_string(),
        "Rock".to_string(),
        "Rock And Roll".to_string(),
        "Sci Fi & Fantasy".to_string(),
        "Science Fiction".to_string(),
        "Soundtrack".to_string(),
        "TV Shows".to_string(),
        "World".to_string(),
];
    assert_eq!(res, DbEntity::Genre(expect));
}
}
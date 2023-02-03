use crate::db::{Connection, DbEntity, DbEntityResult};

pub fn get_genres(conn: Connection) -> DbEntityResult {
    let mut stmt = conn.prepare(
        "SELECT Name FROM genres ORDER BY Name",
    )?;
    // what is the result? Need vector of entities.
    stmt
        .query_map([], |row| {
            Ok(DbEntity::Genre(row.get(0)?))
        }).and_then(Iterator::collect)

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
    let expect = vec![
        DbEntity::Genre("Alternative".to_string()),
        DbEntity::Genre("Alternative & Punk".to_string()),
        DbEntity::Genre("Blues".to_string()),
        DbEntity::Genre("Bossa Nova".to_string()),
        DbEntity::Genre("Classical".to_string()),
        DbEntity::Genre("Comedy".to_string()),
        DbEntity::Genre("Drama".to_string()),
        DbEntity::Genre("Easy Listening".to_string()),
        DbEntity::Genre("Electronica/Dance".to_string()),
        DbEntity::Genre("Heavy Metal".to_string()),
        DbEntity::Genre("Hip Hop/Rap".to_string()),
        DbEntity::Genre("Jazz".to_string()),
        DbEntity::Genre("Latin".to_string()),
        DbEntity::Genre("Metal".to_string()),
        DbEntity::Genre("Opera".to_string()),
        DbEntity::Genre("Pop".to_string()),
        DbEntity::Genre("R&B/Soul".to_string()),
        DbEntity::Genre("Reggae".to_string()),
        DbEntity::Genre("Rock".to_string()),
        DbEntity::Genre("Rock And Roll".to_string()),
        DbEntity::Genre("Sci Fi & Fantasy".to_string()),
        DbEntity::Genre("Science Fiction".to_string()),
        DbEntity::Genre("Soundtrack".to_string()),
        DbEntity::Genre("TV Shows".to_string()),
        DbEntity::Genre("World".to_string()),
];
    assert_eq!(res, expect);
}
// prepare connections.
}
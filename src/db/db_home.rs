use crate::db::DbEntity;

pub fn get_genres(conn: super::Connection) -> super::DbEntityResult {
    let mut stmt = conn.prepare(
        "SELECT Name FROM genres ORDER BY Name",
    )?;
    stmt
        .query_map(["All"], |row| {
            Ok(DbEntity::Genre(row.get(0)?))
        })
        .and_then(Iterator::collect)
}
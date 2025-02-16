use crate::error::DbError;
use crate::utils;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    utils::db::create_table(
        &conn,
        "map_channel_group_to_channel",
        include_str!("sql/create_map_channel_group_to_channel.sql"),
        clean,
    )?;

    Ok(())
}

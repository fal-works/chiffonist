use crate::error::DbError;
use crate::utils;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    utils::db::create_table(
        &conn,
        "transaction_history",
        include_str!("sql/create_transaction_history.sql"),
        clean,
    )?;
    utils::db::create_view(
        &conn,
        "transaction_view",
        include_str!("sql/create_transaction_view.sql"),
        clean,
    )?;

    Ok(())
}

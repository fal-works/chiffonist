use crate::error::DbError;
use crate::utils;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    utils::create_table(
        &conn,
        "mf_transaction",
        include_str!("sql/create_mf_transaction.sql"),
        clean,
    )?;
    utils::create_table(
        &conn,
        "mapping_mf_financial_institution_to_channel",
        include_str!("sql/create_mapping_mf_financial_institution_to_channel.sql"),
        clean,
    )?;
    utils::create_table(
        &conn,
        "mf_transaction_manual_categorization",
        include_str!("sql/create_mf_transaction_manual_categorization.sql"),
        clean,
    )?;
    utils::create_table(
        &conn,
        "mf_transaction_categorization_rule",
        include_str!("sql/create_mf_transaction_categorization_rule.sql"),
        clean,
    )?;

    Ok(())
}

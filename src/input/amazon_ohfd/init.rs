use crate::error::DbError;
use crate::utils;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    utils::db::create_table(
        &conn,
        "amazon_ohfd",
        include_str!("sql/create_amazon_ohfd.sql"),
        clean,
    )?;
    utils::db::create_table(
        &conn,
        "mapping_amazon_ohfd_credit_card_to_channel",
        include_str!("sql/create_mapping_amazon_ohfd_credit_card_to_channel.sql"),
        clean,
    )?;
    utils::db::create_table(
        &conn,
        "amazon_ohfd_manual_categorization",
        include_str!("sql/create_amazon_ohfd_manual_categorization.sql"),
        clean,
    )?;
    utils::db::create_table(
        &conn,
        "amazon_ohfd_categorization_rule",
        include_str!("sql/create_amazon_ohfd_categorization_rule.sql"),
        clean,
    )?;

    Ok(())
}

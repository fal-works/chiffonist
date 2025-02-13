use crate::error::DbError;

mod transform;
mod init;
mod load;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn load_files() -> Result<(), DbError> {
    load::load_amazon_ohfd()?;
    load::load_mapping_amazon_ohfd_credit_card_to_channel()?;
    load::load_amazon_ohfd_manual_categorization()?;
    load::load_amazon_ohfd_categorization_rules()?;

    Ok(())
}

pub fn transform_transactions() -> Result<bool, DbError> {
    transform::etl_amazon_ohfd_to_transaction_history()
}

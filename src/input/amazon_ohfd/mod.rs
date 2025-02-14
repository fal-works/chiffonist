use crate::error::DbError;

mod init;
mod load;
mod transform;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn load_files() -> Result<(), DbError> {
    load::load_amazon_ohfd("data/input/amazon-ohfd/transactions/")?;
    load::load_mapping_amazon_ohfd_credit_card_to_channel(
        "data/input/amazon-ohfd/channel-mapping-from-credit-card.yaml",
    )?;
    load::load_amazon_ohfd_manual_categorization(
        "data/input/amazon-ohfd/transaction-manual-categorization.yaml",
    )?;
    load::load_amazon_ohfd_categorization_rules(
        "data/input/amazon-ohfd/transaction-categorization-rules/",
    )?;

    Ok(())
}

pub fn transform_transactions() -> Result<bool, DbError> {
    transform::etl_amazon_ohfd_to_transaction_history()
}

use crate::error::DbError;

mod init;
mod load;
mod transform;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn load_files() -> Result<(), DbError> {
    load::load_mf_transactions("data/input/money-forward/transactions/")?;
    load::load_mapping_mf_financial_institution_to_channel(
        "data/input/money-forward/channel-mapping-from-financial-institution.yaml",
    )?;
    load::load_mf_transaction_manual_categorization(
        "data/input/money-forward/transaction-manual-categorization.yaml",
    )?;
    load::load_categorization_rules("data/input/money-forward//transaction-categorization-rules/")?;

    Ok(())
}

pub fn transform_transactions() -> Result<bool, DbError> {
    transform::etl_mf_transaction_to_transaction_history()
}

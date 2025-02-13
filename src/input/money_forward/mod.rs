use crate::error::DbError;

mod transform;
mod init;
mod load;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn load_files() -> Result<(), DbError> {
    load::load_mf_transactions()?;
    load::load_mapping_mf_financial_institution_to_channel()?;
    load::load_mf_transaction_manual_categorization()?;
    load::load_categorization_rules()?;

    Ok(())
}

pub fn transform_transactions() -> Result<bool, DbError> {
    transform::etl_mf_transaction_to_transaction_history()
}

mod db;

fn main() {
    let clean = false;

    db::init::create_tables(clean).unwrap();
    db::load::load_mf_transactions().unwrap();
    db::print::print_mf_transaction_summary().unwrap();
    db::load::load_mapping_mf_financial_institution_to_channel().unwrap();
    db::load::load_categorization_rules().unwrap();
    if db::transform::etl_mf_transaction_to_transaction_history().unwrap() {
        db::print::print_transaction_summary().unwrap();
    }
}

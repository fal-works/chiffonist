mod db;

fn main() {
    let clean = false;

    db::create_tables(clean).unwrap();
    db::insert_csv_to_db().unwrap();
    db::print_mf_transaction_summary().unwrap();
    db::load_categorization_rules().unwrap();
    if db::etl_mf_transaction_to_transaction_history().unwrap() {
        db::print_transaction_summary().unwrap();
    }
}

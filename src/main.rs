mod db;

fn main() {
    create_tables();
    load_csv();
    print_mf_transaction();
    load_categorization_rules();
    etl_mf_transaction_to_transaction_history();
    print_transaction();
}

fn create_tables() {
    if let Err(e) = db::create_tables(false) {
        eprintln!("Error creating table: {}", e);
    }
}

fn load_csv() {
    if let Err(e) = db::insert_csv_to_db() {
        eprintln!("Error loading CSV: {}", e);
    }
}

fn print_mf_transaction() {
    if let Err(e) = db::print_mf_transaction_summary() {
        eprintln!("Error printing mf_transaction: {}", e);
    }
}

fn load_categorization_rules() {
    if let Err(e) = db::load_categorization_rules() {
        eprintln!("Error loading categorization rules: {}", e);
    }
}

fn etl_mf_transaction_to_transaction_history() {
    if let Err(e) = db::etl_mf_transaction_to_transaction_history() {
        eprintln!("Error transferring MF recortds: {}", e);
    }
}

fn print_transaction() {
    if let Err(e) = db::print_transaction_summary() {
        eprintln!("Error printing transaction: {}", e);
    }
}

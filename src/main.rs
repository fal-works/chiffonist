mod db;

fn main() {
    create_tables();
    load_csv();
    print_mf_transaction();
}

fn create_tables() {
    if let Err(e) = db::create_mf_transaction_table() {
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

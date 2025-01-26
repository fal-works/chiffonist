mod db;

fn main() {
    create_tables();
    load_csv();
    print_transactions();
}

fn create_tables() {
    if let Err(e) = db::create_transactions_table() {
        eprintln!("Error creating table: {}", e);
    }
}

fn load_csv() {
    if let Err(e) = db::insert_csv_to_db() {
        
        eprintln!("Error loading CSV: {}", e);
    }
}

fn print_transactions() {
    if let Err(e) = db::print_transactions_summary() {
        eprintln!("Error printing transactions: {}", e);
    }
}

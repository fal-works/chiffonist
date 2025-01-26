mod db;

fn main() {
    if let Err(e) = db::create_transactions_table() {
        eprintln!("Error creating table: {}", e);
    }
}

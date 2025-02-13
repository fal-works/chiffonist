mod consolidation;
mod error;
mod input;
mod utils;

fn main() {
    let clean = false;

    input::common::create_tables(clean).unwrap();
    input::money_forward::create_tables(clean).unwrap();
    input::amazon_ohfd::create_tables(clean).unwrap();
    consolidation::create_tables(clean).unwrap();

    input::common::load_files().unwrap();
    input::money_forward::load_files().unwrap();
    input::amazon_ohfd::load_files().unwrap();

    let mut inserted_transaction_history = false;
    inserted_transaction_history |= input::money_forward::transform_transactions().unwrap();
    inserted_transaction_history |= input::amazon_ohfd::transform_transactions().unwrap();
    if inserted_transaction_history {
        consolidation::print_summary().unwrap();
    }
}

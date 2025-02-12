mod db;

fn main() {
    let clean = false;

    db::init::create_tables(clean).unwrap();

    db::load::load_map_channel_group_to_channel().unwrap();

    let mut inserted_transaction_history = false;

    db::load::load_mf_transactions().unwrap();
    db::print::print_mf_transaction_summary().unwrap();
    db::load::load_mapping_mf_financial_institution_to_channel().unwrap();
    db::load::load_mf_transaction_manual_categorization().unwrap();
    db::load::load_categorization_rules().unwrap();
    inserted_transaction_history |=
        db::transform::etl_mf_transaction_to_transaction_history().unwrap();

    db::load_amazon_ohfd::load_amazon_ohfd().unwrap();
    db::load_amazon_ohfd::load_mapping_amazon_ohfd_credit_card_to_channel().unwrap();
    db::load_amazon_ohfd::load_amazon_ohfd_manual_categorization().unwrap();
    db::load_amazon_ohfd::load_amazon_ohfd_categorization_rules().unwrap();
    inserted_transaction_history |=
        db::transform_amazon_ohfd::etl_amazon_ohfd_to_transaction_history().unwrap();

    if inserted_transaction_history {
        db::print::print_transaction_summary().unwrap();
    }
}

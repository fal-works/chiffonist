use crate::error::DbError;
use crate::utils;

pub fn etl_amazon_ohfd_to_transaction_history() -> Result<bool, DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let mut select_unknown_credit_card = conn.prepare(include_str!(
        "sql/select_amazon_ohfd_unknown_credit_card.sql"
    ))?;

    let to_be_inserted = if select_unknown_credit_card.exists([])? {
        println!("下記のクレカ種類は channel がマッピングされていないため、処理対象外となります。");
        utils::db::print_select_query(&mut select_unknown_credit_card, &[("[保有金融機関]", 0)])?;
        utils::io::confirm_continue()?
    } else {
        conn.execute(
            include_str!("sql/create_tmp_categorized_amazon_ohfd.sql"),
            [],
        )?;
        conn.execute(include_str!("sql/categorize_amazon_ohfd.sql"), [])?;

        let mut select_uncategorized =
            conn.prepare(include_str!("sql/select_uncategorized_amazon_ohfd.sql"))?;

        if select_uncategorized.exists([])? {
            println!("下記の明細が分類できませんでした:");
            utils::db::print_select_query(
                &mut select_uncategorized,
                &[
                    ("注文日", 0),
                    ("注文番号", 0),
                    ("商品名", 0),
                    ("付帯情報", 0),
                    ("金額", 0),
                    ("クレカ種類", 0),
                ],
            )?;
            utils::io::confirm_continue()?
        } else {
            true
        }
    };

    if to_be_inserted {
        conn.execute(
            include_str!("sql/insert_categorized_amazon_ohfd_to_transaction_history.sql"),
            [],
        )?;
        println!(
            "Successfully transferred Amazon Order History Filter records to transaction_history."
        );
    } else {
        println!(
            "Cancelled transferring Amazon Order History Filter records to transaction_history."
        );
    }

    Ok(to_be_inserted)
}

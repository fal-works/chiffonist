use crate::db::error::DbError;
use crate::db::utils;

pub fn etl_mf_transaction_to_transaction_history() -> Result<bool, DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let mut select_unknown_insttitutions = conn.prepare(include_str!(
        "sql/select_mf_unknown_financial_institutions.sql"
    ))?;

    let to_be_inserted = if select_unknown_insttitutions.exists([])? {
        println!("下記の金融機関は channel がマッピングされていないため、処理対象外となります。");
        utils::print_select_query(&mut select_unknown_insttitutions, &[("[保有金融機関]", 0)])?;
        utils::confirm_continue()?
    } else {
        conn.execute(
            include_str!("sql/create_tmp_categorized_mf_transaction.sql"),
            [],
        )?;
        conn.execute(include_str!("sql/categorize_mf_transaction.sql"), [])?;

        let mut select_uncategorized =
            conn.prepare(include_str!("sql/select_uncategorized_mf_transaction.sql"))?;

        if select_uncategorized.exists([])? {
            println!("下記の明細が分類できませんでした:");
            utils::print_select_query(
                &mut select_uncategorized,
                &[
                    ("id", 0),
                    ("計算対象", 0),
                    ("日付", 0),
                    ("内容", 20),
                    ("金額", 0),
                    ("保有金融機関", 0),
                    ("大項目", 0),
                    ("中項目", 0),
                    ("メモ", 0),
                    ("振替", 0),
                    ("MF ID", 0),
                ],
            )?;
            utils::confirm_continue()?
        } else {
            true
        }
    };

    if to_be_inserted {
        conn.execute(
            include_str!("sql/insert_categorized_mf_transaction_to_transaction_history.sql"),
            [],
        )?;
        println!("Successfully transferred MF records to transaction_history.");
    } else {
        println!("Cancelled transferring MF records to transaction_history.");
    }

    Ok(to_be_inserted)
}

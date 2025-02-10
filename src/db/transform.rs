use crate::db::error::DbError;
use crate::db::utils;

pub fn etl_mf_transaction_to_transaction_history() -> Result<bool, DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let mut select_unknown_insttitutions = conn.prepare(include_str!(
        "sql/select_mf_unknown_financial_institutions.sql"
    ))?;

    let to_be_inserted = if select_unknown_insttitutions.exists([])? {
        println!("下記の金融機関は channel がマッピングされていないため、処理対象外となります。");
        utils::print_select_query(&mut select_unknown_insttitutions, &["[保有金融機関]"])?;
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
                    "id",
                    "計算対象",
                    "日付",
                    "内容",
                    "金額（円）",
                    "保有金融機関",
                    "大項目",
                    "中項目",
                    "メモ",
                    "振替",
                    // "MF ID",
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

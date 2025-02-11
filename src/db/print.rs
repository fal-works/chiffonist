use crate::db::error::DbError;
use crate::db::utils;

pub fn print_mf_transaction_summary() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM mf_transaction", [], |row| row.get(0))?;
    println!("mf_transaction total records: {}", count);

    println!("mf_transaction first 10 records:");
    let mut stmt: rusqlite::Statement<'_> =
        conn.prepare("SELECT * FROM mf_transaction LIMIT 10;")?;
    let column_names = [
        "ID",
        "計算対象",
        "日付",
        "内容",
        "金額",
        "金融機関",
        "大項目",
        "中項目",
        "メモ",
        "振替",
        // "MF ID",
    ];
    utils::print_select_query(&mut stmt, &column_names)?;

    Ok(())
}

pub fn print_transaction_summary() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM transaction_history", [], |row| {
        row.get(0)
    })?;
    println!("transaction_history total records: {}", count);

    println!("transaction_history first 10 records:");
    let mut stmt: rusqlite::Statement<'_> =
        conn.prepare("SELECT * FROM transaction_history LIMIT 10;")?;
    let column_names = [
        "ID",
        "年",
        "月",
        "日付",
        "摘要",
        "金額",
        "チャネル",
        "カテゴリー",
        "サブカテゴリー",
        "メモ",
    ];
    utils::print_select_query(&mut stmt, &column_names)?;

    Ok(())
}

use crate::db::error::DbError;
use crate::db::utils;

pub fn print_mf_transaction_summary() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM mf_transaction", [], |row| row.get(0))?;
    println!("mf_transaction total records: {}", count);

    println!("mf_transaction first 10 records:");
    let mut stmt: rusqlite::Statement<'_> =
        conn.prepare("SELECT * FROM mf_transaction LIMIT 10;")?;
    let columns = [
        ("ID", 0),
        ("計算対象", 0),
        ("日付", 0),
        ("内容", 30),
        ("金額", 0),
        ("金融機関", 0),
        ("大項目", 0),
        ("中項目", 0),
        ("メモ", 30),
        ("振替", 0),
        // "MF ID",
    ];
    utils::print_select_query(&mut stmt, &columns)?;

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
    let columns = [
        ("ID", 0),
        ("年", 0),
        ("月", 0),
        ("日付", 0),
        ("摘要", 30),
        ("金額", 0),
        ("チャネル", 0),
        ("カテゴリー", 0),
        ("サブカテゴリー", 0),
        ("メモ", 30),
    ];
    utils::print_select_query(&mut stmt, &columns)?;

    Ok(())
}

use crate::error::DbError;
use crate::utils;

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
    utils::db::print_select_query(&mut stmt, &columns)?;

    Ok(())
}

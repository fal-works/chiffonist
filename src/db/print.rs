use crate::db::error::DbError;

pub fn print_mf_transaction_summary() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM mf_transaction", [], |row| row.get(0))?;
    println!("mf_transaction total records: {}", count);

    let mut stmt = conn.prepare("SELECT * FROM mf_transaction LIMIT 10")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, i32>(9)?,
            row.get::<_, String>(10)?,
        ))
    })?;

    println!("mf_transaction first 10 records:");
    for record in rows {
        let (
            id,
            include_flag,
            occurrence_date,
            particulars,
            amount,
            financial_institution,
            major_category,
            intermediate_category,
            memo,
            transfer_flag,
            mf_original_id,
        ) = record?;
        println!(
          "ID: {}, 計算対象: {}, 日付: {}, 内容: {}, 金額: {}, 金融機関: {}, 大項目: {}, 中項目: {}, メモ: {}, 振替: {}, MF ID: {}",
          id, include_flag, occurrence_date, particulars, amount, financial_institution, major_category, intermediate_category, memo, transfer_flag, mf_original_id
      );
    }

    Ok(())
}

pub fn print_transaction_summary() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM transaction_history", [], |row| {
        row.get(0)
    })?;
    println!("transaction_history total records: {}", count);

    let mut stmt = conn.prepare("SELECT * FROM transaction_history LIMIT 10")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
        ))
    })?;

    println!("transaction_history first 10 records:");
    for record in rows {
        let (id, occurrence_date, particulars, amount, channel, category, sub_category, memo) =
            record?;
        println!(
            "ID: {}, Date: {}, Particulars: {}, Amount: {}, Channel: {}, Category: {}, Sub-category: {}, Memo: {}",
            id, occurrence_date, particulars, amount, channel, category, sub_category, memo
        );
    }

    Ok(())
}

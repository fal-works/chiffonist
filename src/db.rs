use csv::ReaderBuilder;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs;

pub enum DbError {
    Csv(csv::Error),
    Sqlite(rusqlite::Error),
    Std(std::io::Error),
    Other(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Csv(err) => write!(f, "CSV Error: {}", err),
            DbError::Sqlite(err) => write!(f, "SQLite Error: {}", err),
            DbError::Std(err) => write!(f, "IO Error: {}", err),
            DbError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

pub fn create_tables() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;

    create_table(&conn, include_str!("sql/create_mf_transaction.sql"))?;
    create_table(&conn, include_str!("sql/create_transaction_history.sql"))?;

    println!("Tables created successfully.");

    Ok(())
}

fn create_table(conn: &rusqlite::Connection, sql: &str) -> Result<(), DbError> {
    conn.execute(sql, []).map_err(DbError::Sqlite)?;
    Ok(())
}

pub fn insert_csv_to_db() -> Result<(), DbError> {
    let conn =
        rusqlite::Connection::open("data/transactions.db").map_err(|e| DbError::Sqlite(e))?;
    let input_dir = "data/input/";

    for entry in fs::read_dir(input_dir).map_err(|e| DbError::Std(e))? {
        let entry = entry.map_err(|e| DbError::Std(e))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            println!("Processing file: {:?}", path);

            let file = fs::File::open(&path).map_err(|e| DbError::Std(e))?;
            let transcoded_reader = DecodeReaderBytesBuilder::new()
                .encoding(Some(SHIFT_JIS))
                .build(file);

            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(transcoded_reader);

            let headers = reader.headers().map_err(|e| DbError::Csv(e))?.clone();
            let expected_headers = [
                "計算対象",
                "日付",
                "内容",
                "金額（円）",
                "保有金融機関",
                "大項目",
                "中項目",
                "メモ",
                "振替",
                "ID",
            ];
            for i in 0..expected_headers.len() {
                if headers[i] != *expected_headers[i] {
                    return Err(DbError::Other(format!(
                        "Expected header: {}, actual header: {}",
                        &headers[i], expected_headers[i]
                    )));
                }
            }

            for result in reader.records() {
                let record = result.map_err(|e| DbError::Csv(e))?;

                conn.execute(
                    "INSERT INTO mf_transaction (
                  include, date, description, amount, financial_institution,
                  major_category, minor_category, memo, transfer, mf_original_id
              ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    (
                        record[0]
                            .parse::<i32>()
                            .map_err(|e| DbError::Other(e.to_string()))?,
                        &record[1],
                        &record[2],
                        record[3]
                            .parse::<i32>()
                            .map_err(|e| DbError::Other(e.to_string()))?,
                        &record[4],
                        &record[5],
                        &record[6],
                        record.get(7).unwrap_or(""),
                        record[8]
                            .parse::<i32>()
                            .map_err(|e| DbError::Other(e.to_string()))?,
                        &record[9],
                    ),
                )
                .map_err(|e| DbError::Sqlite(e))?;
            }
        }
    }

    println!("CSV files have been inserted into the database.");
    Ok(())
}

pub fn print_mf_transaction_summary() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM mf_transaction", [], |row| row.get(0))?;
    println!("Total records: {}", count);

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

    println!("First 10 records:");
    for record in rows {
        let (
            id,
            include,
            date,
            description,
            amount,
            financial_institution,
            major_category,
            minor_category,
            memo,
            transfer,
            mf_original_id,
        ) = record?;
        println!(
          "ID: {}, Include: {}, Date: {}, Description: {}, Amount: {}, Institution: {}, Major: {}, Minor: {}, Memo: {}, Transfer: {}, MF original ID: {}",
          id, include, date, description, amount, financial_institution, major_category, minor_category, memo, transfer, mf_original_id
      );
    }

    Ok(())
}

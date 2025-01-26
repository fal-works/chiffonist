use csv::ReaderBuilder;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs;

pub fn create_transactions_table() -> Result<(), rusqlite::Error> {
    // SQLiteデータベースを作成または接続
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    // テーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            include INTEGER NOT NULL,
            date TEXT NOT NULL,
            description TEXT NOT NULL,
            amount INTEGER NOT NULL,
            financial_institution TEXT NOT NULL,
            major_category TEXT NOT NULL,
            minor_category TEXT NOT NULL,
            memo TEXT,
            transfer INTEGER NOT NULL,
            mf_id TEXT NOT NULL UNIQUE
        );",
        [],
    )?;

    println!("Table 'transactions' created successfully.");

    Ok(())
}

pub enum LoadingError {
    Csv(csv::Error),
    Sqlite(rusqlite::Error),
    Std(std::io::Error),
    Other(String),
}

impl std::fmt::Display for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingError::Csv(err) => write!(f, "CSV Error: {}", err),
            LoadingError::Sqlite(err) => write!(f, "SQLite Error: {}", err),
            LoadingError::Std(err) => write!(f, "IO Error: {}", err),
            LoadingError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

pub fn insert_csv_to_db() -> Result<(), LoadingError> {
    let conn =
        rusqlite::Connection::open("data/transactions.db").map_err(|e| LoadingError::Sqlite(e))?;
    let input_dir = "data/input/";

    for entry in fs::read_dir(input_dir).map_err(|e| LoadingError::Std(e))? {
        let entry = entry.map_err(|e| LoadingError::Std(e))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            println!("Processing file: {:?}", path);

            let file = fs::File::open(&path).map_err(|e| LoadingError::Std(e))?;
            let transcoded_reader = DecodeReaderBytesBuilder::new()
                .encoding(Some(SHIFT_JIS))
                .build(file);

            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(transcoded_reader);

            let headers = reader.headers().map_err(|e| LoadingError::Csv(e))?.clone();
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
                    return Err(LoadingError::Other(format!(
                        "Expected header: {}, actual header: {}",
                        &headers[i], expected_headers[i]
                    )));
                }
            }

            for result in reader.records() {
                let record = result.map_err(|e| LoadingError::Csv(e))?;

                conn.execute(
                    "INSERT INTO transactions (
                  include, date, description, amount, financial_institution,
                  major_category, minor_category, memo, transfer, mf_id
              ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    (
                        record[0]
                            .parse::<i32>()
                            .map_err(|e| LoadingError::Other(e.to_string()))?,
                        &record[1],
                        &record[2],
                        record[3]
                            .parse::<i32>()
                            .map_err(|e| LoadingError::Other(e.to_string()))?,
                        &record[4],
                        &record[5],
                        &record[6],
                        record.get(7).unwrap_or(""),
                        record[8]
                            .parse::<i32>()
                            .map_err(|e| LoadingError::Other(e.to_string()))?,
                        &record[9],
                    ),
                )
                .map_err(|e| LoadingError::Sqlite(e))?;
            }
        }
    }

    println!("CSV files have been inserted into the database.");
    Ok(())
}

pub fn print_transactions_summary() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))?;
    println!("Total records: {}", count);

    let mut stmt = conn.prepare("SELECT * FROM transactions LIMIT 10")?;
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
            mf_id,
        ) = record?;
        println!(
          "ID: {}, Include: {}, Date: {}, Description: {}, Amount: {}, Institution: {}, Major: {}, Minor: {}, Memo: {}, Transfer: {}, MF_ID: {}",
          id, include, date, description, amount, financial_institution, major_category, minor_category, memo, transfer, mf_id
      );
    }

    Ok(())
}

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

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;

    create_table(
        &conn,
        "mf_transaction",
        include_str!("sql/create_mf_transaction.sql"),
        clean,
    )?;
    create_table(
        &conn,
        "transaction_history",
        include_str!("sql/create_transaction_history.sql"),
        clean,
    )?;

    Ok(())
}

fn create_table(
    conn: &rusqlite::Connection,
    name: &str,
    sql: &str,
    clean: bool,
) -> Result<(), DbError> {
    if clean {
        conn.execute(&format!("DROP TABLE IF EXISTS {name};"), [])
            .map_err(DbError::Sqlite)?;
        println!("Table '{name}' dropped successfully.");
    }

    conn.execute(sql, []).map_err(DbError::Sqlite)?;
    println!("Table '{name}' created successfully.");

    Ok(())
}

pub fn insert_csv_to_db() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;
    let input_dir = "data/input/";

    for entry in fs::read_dir(input_dir).map_err(DbError::Std)? {
        let entry = entry.map_err(DbError::Std)?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            println!("Processing file: {:?}", path);

            let file = fs::File::open(&path).map_err(DbError::Std)?;
            let transcoded_reader = encoding_rs_io::DecodeReaderBytesBuilder::new()
                .encoding(Some(encoding_rs::SHIFT_JIS))
                .build(file);

            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(transcoded_reader);

            let headers = reader.headers().map_err(DbError::Csv)?.clone();
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
                let record = result.map_err(DbError::Csv)?;

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
                .map_err(DbError::Sqlite)?;
            }
        }
    }

    println!("CSV files have been inserted into the database.");
    Ok(())
}

pub fn print_mf_transaction_summary() -> rusqlite::Result<()> {
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

pub fn etl_mf_transaction_to_transaction_history() -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;

    let mut extract_statement = conn
        .prepare(include_str!("sql/extract_mf_transaction.sql"))
        .map_err(DbError::Sqlite)?;
    let mf_transactions = extract_statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,    // id
                row.get::<_, String>(1)?, // date
                row.get::<_, String>(2)?, // description
                row.get::<_, i32>(3)?,    // amount
                row.get::<_, String>(4)?, // major category
                row.get::<_, String>(5)?, // minor category
                row.get::<_, String>(6)?, // memo
            ))
        })
        .map_err(DbError::Sqlite)?;

    let mut load_statement = conn.prepare(
        "INSERT INTO transaction_history (date, description, amount, major_category, minor_category, memo, mf_transaction_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    ).map_err(DbError::Sqlite)?;
    for transaction in mf_transactions {
        let (mf_id, date, description, amount, _major_category, _minor_category, memo) =
            transaction.map_err(DbError::Sqlite)?;
        load_statement
            .execute((&date, &description, amount, "none", "none", &memo, mf_id))
            .map_err(DbError::Sqlite)?;
    }

    println!("Successfully transferred MF records to transaction_history.");
    Ok(())
}

pub fn print_transaction_summary() -> rusqlite::Result<()> {
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
        ))
    })?;

    println!("transaction_history first 10 records:");
    for record in rows {
        let (id, date, description, amount, major_category, minor_category, memo) = record?;
        println!(
            "ID: {}, Date: {}, Description: {}, Amount: {}, Major: {}, Minor: {}, Memo: {}",
            id, date, description, amount, major_category, minor_category, memo
        );
    }

    Ok(())
}

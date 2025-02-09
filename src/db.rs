use std::{fs, io::Write};

#[derive(Debug)]
pub enum DbError {
    Csv(csv::Error),
    Yaml(serde_yaml::Error),
    Sqlite(rusqlite::Error),
    Std(std::io::Error),
    Other(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Csv(err) => write!(f, "CSV Error: {}", err),
            DbError::Yaml(err) => write!(f, "YAML Error: {}", err),
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
    create_table(
        &conn,
        "mf_transaction_categorization_rule",
        include_str!("sql/create_mf_transaction_categorization_rule.sql"),
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
    }

    conn.execute(sql, []).map_err(DbError::Sqlite)?;

    println!("Table '{name}' prepared successfully.");

    Ok(())
}

pub fn insert_csv_to_db() -> Result<(), DbError> {
    let mut conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;
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

            let db_transaction = conn.transaction().map_err(DbError::Sqlite)?;
            for result in reader.records() {
                let record = result.map_err(DbError::Csv)?;

                db_transaction
                    .execute(
                        "INSERT OR IGNORE INTO mf_transaction (
                  include, date, description, amount, financial_institution,
                  major_category, intermediate_category, memo, transfer, mf_original_id
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
            db_transaction.commit().map_err(DbError::Sqlite)?;
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
            intermediate_category,
            memo,
            transfer,
            mf_original_id,
        ) = record?;
        println!(
          "ID: {}, 計算対象: {}, 日付: {}, 内容: {}, 金額: {}, 金融機関: {}, 大項目: {}, 中項目: {}, メモ: {}, 振替: {}, MF ID: {}",
          id, include, date, description, amount, financial_institution, major_category, intermediate_category, memo, transfer, mf_original_id
      );
    }

    Ok(())
}

pub fn load_categorization_rules() -> Result<(), DbError> {
    let yaml_str: String =
        std::fs::read_to_string("data/input/mf-transaction-categorization-rules.yaml")
            .map_err(DbError::Std)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).map_err(DbError::Yaml)?;

    let rules = yaml["rules"].as_sequence().ok_or(DbError::Other(
        "YAML の `rules` が配列ではありません".into(),
    ))?;

    let mut conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;
    let db_transaction = conn.transaction().map_err(DbError::Sqlite)?;

    {
        let mut insert_statement = db_transaction
            .prepare(
                "INSERT INTO mf_transaction_categorization_rule (
            mf_include, mf_date_min, mf_date_max, mf_description_glob,
            mf_amount_min, mf_amount_max, mf_financial_institution,
            mf_major_category, mf_intermediate_category, mf_memo_glob,
            mf_transfer, new_category, new_sub_category
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .map_err(DbError::Sqlite)?;

        for rule in rules {
            let condition = rule.get("if").unwrap_or(&serde_yaml::Value::Null);
            let result = rule
                .get("set")
                .ok_or(DbError::Other("ルールに `set` ブロックがありません".into()))?;

            insert_statement
                .execute(rusqlite::params![
                    condition["計算対象"].as_i64(),
                    condition["日付"]
                        .as_mapping()
                        .and_then(|m| m.get(&serde_yaml::Value::String("min".to_string())))
                        .and_then(|v| v.as_str()),
                    condition["日付"]
                        .as_mapping()
                        .and_then(|m| m.get(&serde_yaml::Value::String("max".to_string())))
                        .and_then(|v| v.as_str()),
                    condition["内容"].as_str(),
                    condition["金額"]
                        .as_mapping()
                        .and_then(|m| m.get(&serde_yaml::Value::String("min".to_string())))
                        .and_then(|v| v.as_i64()),
                    condition["金額"]
                        .as_mapping()
                        .and_then(|m| m.get(&serde_yaml::Value::String("max".to_string())))
                        .and_then(|v| v.as_i64()),
                    condition["金融機関"].as_str(),
                    condition["大項目"].as_str(),
                    condition["中項目"].as_str(),
                    condition["メモ"].as_str(),
                    condition["振替"].as_i64(),
                    result["category"].as_str().ok_or(DbError::Other(
                        "`set` ブロックに `category` がありません".into()
                    ))?,
                    result["sub-category"].as_str()
                ])
                .map_err(DbError::Sqlite)?;
        }
    }

    db_transaction.commit().map_err(DbError::Sqlite)?;

    println!(
        "MF入出金明細の分類ルールを `mf_transaction_categorization_rule` テーブルに挿入しました"
    );
    Ok(())
}

pub fn etl_mf_transaction_to_transaction_history() -> Result<bool, DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db").map_err(DbError::Sqlite)?;

    conn.execute(
        include_str!("sql/create_tmp_categorized_mf_transaction.sql"),
        [],
    )
    .map_err(DbError::Sqlite)?;
    conn.execute(include_str!("sql/categorize_mf_transaction.sql"), [])
        .map_err(DbError::Sqlite)?;

    let mut select_uncategorized = conn
        .prepare(include_str!("sql/select_uncategorized_mf_transaction.sql"))
        .map_err(DbError::Sqlite)?;

    let to_be_inserted = if select_uncategorized.exists([]).map_err(DbError::Sqlite)? {
        println!("下記の明細が分類できませんでした:");
        print_table_tabwriter(
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
        confirm_continue()?
    } else {
        true
    };

    if to_be_inserted {
        conn.execute(
            include_str!("sql/insert_categorized_mf_transaction_to_transaction_history.sql"),
            [],
        )
        .map_err(DbError::Sqlite)?;
        println!("Successfully transferred MF records to transaction_history.");
    } else {
        println!("Cancelled transferring MF records to transaction_history.");
    }

    Ok(to_be_inserted)
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
        let (id, date, description, amount, category, sub_category, memo) = record?;
        println!(
            "ID: {}, Date: {}, Description: {}, Amount: {}, Category: {}, Sub-category: {}, Memo: {}",
            id, date, description, amount, category, sub_category, memo
        );
    }

    Ok(())
}

fn confirm_continue() -> Result<bool, DbError> {
    let mut stdout = std::io::stdout();
    let stdin: std::io::Stdin = std::io::stdin();

    loop {
        stdout
            .write_all(b"Continue? [y/n]\n")
            .map_err(DbError::Std)?;
        stdout.flush().map_err(DbError::Std)?;

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => stdout
                .write_all(b"Please enter 'y' or 'n'.\n")
                .map_err(DbError::Std)?,
        }
    }
}

fn print_table_tabwriter(
    stmt: &mut rusqlite::Statement<'_>,
    column_names: &[&str],
) -> Result<(), DbError> {
    let column_count = column_names.len();
    let rows = stmt
        .query_map([], |row| {
            (0..column_count)
                .map(|i| {
                    let s = db_value_to_string(row.get(i)?);
                    Ok(truncate_string(&s, 30))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .map_err(DbError::Sqlite)?;

    let mut stdout = std::io::stdout();
    stdout.write_all(b"\n").map_err(DbError::Std)?;

    let mut writer = tabwriter::TabWriter::new(std::io::stdout());
    writeln!(writer, "{}", column_names.join("\t")).map_err(DbError::Std)?;
    for row in rows {
        let row_values = row.map_err(DbError::Sqlite)?;
        writeln!(writer, "{}", row_values.join("\t")).map_err(DbError::Std)?;
    }
    writer.flush().map_err(DbError::Std)?;

    stdout.write_all(b"\n").map_err(DbError::Std)?;
    Ok(())
}

fn db_value_to_string(value: rusqlite::types::Value) -> String {
    match value {
        rusqlite::types::Value::Integer(n) => n.to_string(),
        rusqlite::types::Value::Real(f) => f.to_string(),
        rusqlite::types::Value::Text(s) => s,
        rusqlite::types::Value::Null => "[NULL]".to_string(),
        rusqlite::types::Value::Blob(_) => "[BLOB]".to_string(),
    }
}

fn truncate_string(s: &str, max_length: usize) -> String {
    if s.chars().count() <= max_length {
        return s.to_string();
    }

    let truncated: String = s.chars().take(max_length).collect();
    format!("{}...", truncated)
}

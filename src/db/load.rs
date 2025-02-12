use crate::db::error::DbError;
use crate::db::utils;
use std::fs;

pub fn load_mf_transactions() -> Result<(), DbError> {
    println!("MoneyForwardの入出金明細をロードします。");

    let csv_files = utils::list_files_with_extensions("data/input/mf-transactions/", &["csv"])?;

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;

    conn.execute_batch(include_str!("sql/create_mf_transaction_addition_tmp.sql"))?;

    let db_transaction = conn.transaction()?;
    for csv_path in csv_files {
        load_mf_transactions_csv(&db_transaction, &csv_path)?;
    }
    db_transaction.commit()?;

    conn.execute_batch(include_str!(
        "sql/insert_mf_transaction_addition_to_mf_transaction.sql"
    ))?;

    println!("MoneyForwardの入出金明細をロードしました。");
    Ok(())
}

fn load_mf_transactions_csv(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let file = fs::File::open(&path)?;
    let transcoded_reader = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::SHIFT_JIS))
        .build(file);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(transcoded_reader);

    let headers = reader.headers()?.clone();
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
                "Column index: {i}, Expected header: {}, actual header: {}",
                expected_headers[i], &headers[i]
            )));
        }
    }

    for result in reader.records() {
        let record = result?;

        db_transaction.execute(
            "INSERT INTO mf_transaction_addition (
                include_flag, occurrence_date, particulars, amount, financial_institution,
                major_category, intermediate_category, memo, transfer_flag, mf_original_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                record[0]
                    .parse::<i32>()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?,
                utils::normalize_slashed_date(&record[1])?,
                &record[2],
                record[3].parse::<i32>().map_err(|e| e.to_string())?,
                &record[4],
                &record[5],
                &record[6],
                record.get(7).unwrap_or(""),
                record[8].parse::<i32>().map_err(|e| e.to_string())?,
                &record[9],
            ),
        )?;
    }

    Ok(())
}

pub fn load_categorization_rules() -> Result<(), DbError> {
    println!("MF入出金明細の分類規則をロードします。");

    let yaml_files = utils::list_files_with_extensions(
        "data/input/mf-transaction-categorization-rules/",
        &["yaml", "yml"],
    )?;

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    for yaml_path in yaml_files {
        load_categorization_rules_yaml(&db_transaction, &yaml_path)?;
    }

    db_transaction.commit()?;

    println!("MF入出金明細の分類規則をロードしました。");
    Ok(())
}

fn load_categorization_rules_yaml(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::PathBuf,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let yaml_str: String = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;

    let rules = yaml["rules"]
        .as_sequence()
        .ok_or("YAML の `rules` が配列ではありません")?;

    {
        let mut insert_statement = db_transaction.prepare(
            "INSERT INTO mf_transaction_categorization_rule (
          mf_include_flag, mf_occurrence_date_min, mf_occurrence_date_max, mf_particulars_glob,
          mf_amount_min, mf_amount_max, mf_financial_institution,
          mf_major_category, mf_intermediate_category, mf_memo_glob,
          mf_transfer_flag, new_category, new_sub_category
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;

        for rule in rules {
            let condition = rule.get("if").unwrap_or(&serde_yaml::Value::Null);
            let result = rule
                .get("set")
                .ok_or("ルールに `set` ブロックがありません")?;

            insert_statement.execute(rusqlite::params![
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
                result["category"]
                    .as_str()
                    .ok_or("`set` ブロックに `category` がありません")?,
                result["sub-category"].as_str()
            ])?;
        }
    }

    Ok(())
}

pub fn load_mapping_mf_financial_institution_to_channel() -> Result<(), DbError> {
    println!("MF金融機関名からchannelコードへのマッピングをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/mf-financial-institution-to-channel-mapping.yaml";
    load_mapping_mf_financial_institution_to_channel_yaml(
        &db_transaction,
        std::path::Path::new(yaml_path),
    )?;

    db_transaction.commit()?;

    println!("MF金融機関名からchannelコードへのマッピングをロードしました。");
    Ok(())
}

fn load_mapping_mf_financial_institution_to_channel_yaml(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let yaml_str: String = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;

    let mapping = yaml["mapping"]
        .as_mapping()
        .ok_or("YAML の `mapping` がハッシュではありません")?;

    {
        let mut insert_statement = db_transaction.prepare(
            "INSERT INTO mapping_mf_financial_institution_to_channel (
                mf_financial_institution, channel
            ) VALUES (?, ?)",
        )?;

        for (key, value) in mapping {
            insert_statement.execute(rusqlite::params![
                key.as_str()
                    .ok_or_else(|| format!("Invalid key: {:?}", key))?,
                value
                    .as_str()
                    .ok_or_else(|| format!("Invalid value: {:?}", value))?
            ])?;
        }
    }

    Ok(())
}

pub fn load_map_channel_group_to_channel() -> Result<(), DbError> {
    println!("channel group コードから channel コードへのマッピングをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/map-channel-group-to-channel.yaml";
    load_map_channel_group_to_channel_yaml(&db_transaction, std::path::Path::new(yaml_path))?;

    db_transaction.commit()?;

    println!("channel group コードから channel コードへのマッピングをロードしました。");
    Ok(())
}

fn load_map_channel_group_to_channel_yaml(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let yaml_str: String = std::fs::read_to_string(path).map_err(DbError::Std)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).map_err(DbError::Yaml)?;

    let mapping = yaml["mapping"]
        .as_mapping()
        .ok_or_else(|| DbError::Other("YAML の `mapping` がハッシュではありません".into()))?;

    {
        let mut insert_statement = db_transaction
            .prepare(
                "INSERT INTO map_channel_group_to_channel (channel_group, channel) VALUES (?, ?)",
            )
            .map_err(DbError::Sqlite)?;

        for (key, value) in mapping {
            let group = key
                .as_str()
                .ok_or_else(|| DbError::Other(format!("Invalid key: {:?}", key)))?;

            let channels = value.as_sequence().ok_or_else(|| {
                DbError::Other(format!("Invalid value for group '{}': {:?}", group, value))
            })?;

            for channel_value in channels {
                let channel = channel_value.as_str().ok_or_else(|| {
                    DbError::Other(format!(
                        "Invalid channel code in group '{}': {:?}",
                        group, channel_value
                    ))
                })?;

                insert_statement
                    .execute(rusqlite::params![group, channel])
                    .map_err(DbError::Sqlite)?;
            }
        }
    }

    Ok(())
}

pub fn load_mf_transaction_manual_categorization() -> Result<(), DbError> {
    println!("MF入出金明細の手動分類データをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/mf-transaction-manual-categorization.yaml";
    load_mf_transaction_manual_categorization_yaml(
        &db_transaction,
        std::path::Path::new(yaml_path),
    )?;

    db_transaction.commit()?;

    println!("MF入出金明細の手動分類データをロードしました。");
    Ok(())
}

fn load_mf_transaction_manual_categorization_yaml(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let yaml_str: String = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;

    let mapping = yaml["mapping"]
        .as_mapping()
        .ok_or_else(|| DbError::Other("YAML の `mapping` がハッシュではありません".into()))?;

    let mut insert_statement = db_transaction.prepare(
        "INSERT INTO mf_transaction_manual_categorization (
            mf_original_id, category, sub_category
        ) VALUES (?, ?, ?)",
    )?;

    for (key, value) in mapping {
        let id = key
            .as_str()
            .ok_or_else(|| format!("Invalid key: {:?}", key))?;

        let categories = value
            .as_mapping()
            .ok_or_else(|| format!("Invalid value for id '{}': {:?}", id, value))?;
        let category = categories
            .get("category")
            .ok_or_else(|| format!("category がありません"))
            .and_then(|v| {
                v.as_str()
                    .ok_or_else(|| format!("Invalid category for id {}: {:?}", id, v))
            })?;
        let sub_category = categories
            .get("sub-category")
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| format!("Invalid category for id {}: {:?}", id, v))
            })
            .transpose()?;

        insert_statement.execute(rusqlite::params!(id, category, sub_category))?;
    }

    Ok(())
}

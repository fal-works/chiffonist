use crate::error::DbError;
use crate::utils;
use std::fs;

pub fn load_amazon_ohfd() -> Result<(), DbError> {
    println!("アマゾン注文履歴フィルタ (デジタル) のCSVをロードします。");

    let csv_files =
        utils::list_files_with_extensions("data/input/amazon-ohfd/transactions/", &["csv"])?;

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;

    conn.execute_batch(include_str!("sql/create_amazon_ohfd_addition_tmp.sql"))?;

    let db_transaction = conn.transaction()?;
    for csv_path in csv_files {
        load_amazon_ohfd_csv(&db_transaction, &csv_path)?;
    }
    db_transaction.commit()?;

    conn.execute_batch(include_str!(
        "sql/insert_amazon_ohfd_addition_to_amazon_ohfd.sql"
    ))?;

    println!("アマゾン注文履歴フィルタ (デジタル) のCSVをロードしました。");
    Ok(())
}

fn load_amazon_ohfd_csv(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let file = fs::File::open(&path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let headers = reader.headers()?.clone();
    let expected_headers = [
        "注文日",
        "注文番号",
        "商品名",
        "付帯情報",
        "価格",
        "",
        "",
        "注文合計",
        "",
        "",
        "",
        "",
        "",
        "",
        "クレカ種類",
    ];
    for i in 0..expected_headers.len() {
        if expected_headers[i].len() > 0 && headers[i] != *expected_headers[i] {
            return Err(DbError::Other(format!(
                "Column index: {i}, Expected header: {}, actual header: {}",
                expected_headers[i], &headers[i]
            )));
        }
    }

    for result in reader.records() {
        let record = result?;

        db_transaction.execute(
            "INSERT INTO amazon_ohfd_addition (
                order_date, order_no, product_name, product_info, amount, credit_card
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                utils::normalize_slashed_date(&record[0])?,
                &record[1],
                &record[2],
                &record[3],
                (if record[4].len() > 0 {
                    &record[4]
                } else {
                    &record[7]
                })
                .parse::<i32>()
                .map_err(|e| e.to_string())?,
                record.get(14).unwrap_or(""),
            ),
        )?;
    }

    Ok(())
}

pub fn load_amazon_ohfd_categorization_rules() -> Result<(), DbError> {
    println!("アマゾン注文履歴フィルタ (デジタル) の分類規則をロードします。");

    let yaml_files = utils::list_files_with_extensions(
        "data/input/amazon-ohfd/transaction-categorization-rules/",
        &["yaml", "yml"],
    )?;

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    for yaml_path in yaml_files {
        load_categorization_rules_yaml(&db_transaction, &yaml_path)?;
    }

    db_transaction.commit()?;

    println!("アマゾン注文履歴フィルタ (デジタル) の分類規則をロードしました。");
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
            "INSERT INTO amazon_ohfd_categorization_rule (
                order_date_min, order_date_max, product_name_glob, product_info_glob,
                amount_min, amount_max, credit_card,
                new_category, new_sub_category
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;

        for rule in rules {
            let condition = rule.get("if").unwrap_or(&serde_yaml::Value::Null);
            let result = rule
                .get("set")
                .ok_or("ルールに `set` ブロックがありません")?;

            insert_statement.execute(rusqlite::params![
                condition["注文日"]
                    .as_mapping()
                    .and_then(|m| m.get(&serde_yaml::Value::String("min".to_string())))
                    .and_then(|v| v.as_str()),
                condition["注文日"]
                    .as_mapping()
                    .and_then(|m| m.get(&serde_yaml::Value::String("max".to_string())))
                    .and_then(|v| v.as_str()),
                condition["商品名"].as_str(),
                condition["付帯情報"].as_str(),
                condition["金額"]
                    .as_mapping()
                    .and_then(|m| m.get(&serde_yaml::Value::String("min".to_string())))
                    .and_then(|v| v.as_i64()),
                condition["金額"]
                    .as_mapping()
                    .and_then(|m| m.get(&serde_yaml::Value::String("max".to_string())))
                    .and_then(|v| v.as_i64()),
                condition["クレカ種類"].as_str(),
                result["category"]
                    .as_str()
                    .ok_or("`set` ブロックに `category` がありません")?,
                result["sub-category"].as_str()
            ])?;
        }
    }

    Ok(())
}

pub fn load_mapping_amazon_ohfd_credit_card_to_channel() -> Result<(), DbError> {
    println!("アマゾン注文履歴フィルタ (デジタル) の「クレカ種類」からchannelコードへのマッピングをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/amazon-ohfd/channel-mapping-from-credit-card.yaml";
    load_mapping_amazon_ohfd_credit_card_to_channel_yaml(
        &db_transaction,
        std::path::Path::new(yaml_path),
    )?;

    db_transaction.commit()?;

    println!("アマゾン注文履歴フィルタ (デジタル) の「クレカ種類」からchannelコードへのマッピングをロードしました。");
    Ok(())
}

fn load_mapping_amazon_ohfd_credit_card_to_channel_yaml(
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
            "INSERT INTO mapping_amazon_ohfd_credit_card_to_channel (
                credit_card, channel
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

pub fn load_amazon_ohfd_manual_categorization() -> Result<(), DbError> {
    println!("アマゾン注文履歴フィルタ (デジタル) の手動分類データをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/amazon-ohfd/transaction-manual-categorization.yaml";
    load_amazon_ohfd_manual_categorization_yaml(&db_transaction, std::path::Path::new(yaml_path))?;

    db_transaction.commit()?;

    println!("アマゾン注文履歴フィルタ (デジタル) の手動分類データをロードしました。");
    Ok(())
}

fn load_amazon_ohfd_manual_categorization_yaml(
    db_transaction: &rusqlite::Transaction<'_>,
    path: &std::path::Path,
) -> Result<(), DbError> {
    println!("Processing file: {:?}", path);

    let yaml_str: String = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;

    let mapping = yaml["mapping"]
        .as_sequence()
        .ok_or("YAML の `mapping` が配列ではありません")?;

    {
        let mut insert_statement = db_transaction.prepare(
            "INSERT INTO amazon_ohfd_manual_categorization (
                order_no, product_name,
                category, sub_category
            ) VALUES (?, ?, ?, ?)",
        )?;

        for entry in mapping {
            let condition = entry
                .get("if")
                .ok_or("ルールに `if` ブロックがありません")?;
            let result = entry
                .get("set")
                .ok_or("ルールに `set` ブロックがありません")?;

            insert_statement.execute(rusqlite::params![
                condition["注文番号"].as_str(),
                condition["商品名"].as_str(),
                result["category"]
                    .as_str()
                    .ok_or("`set` ブロックに `category` がありません")?,
                result["sub-category"].as_str()
            ])?;
        }
    }

    Ok(())
}

use crate::error::DbError;

pub fn load_map_channel_group_to_channel() -> Result<(), DbError> {
    println!("channel group コードから channel コードへのマッピングをロードします。");

    let mut conn = rusqlite::Connection::open("data/transactions.db")?;
    let db_transaction = conn.transaction()?;

    let yaml_path = "data/input/channel-groups.yaml";
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

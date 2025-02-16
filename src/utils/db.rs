use crate::error::DbError;
use crate::utils;

pub fn create_table(
    conn: &rusqlite::Connection,
    name: &str,
    sql: &str,
    clean: bool,
) -> Result<(), rusqlite::Error> {
    if clean {
        conn.execute(&format!("DROP TABLE IF EXISTS {name};"), [])?;
    }

    conn.execute_batch(sql)?;

    println!("Table '{name}' created successfully.");

    Ok(())
}

pub fn create_view(
    conn: &rusqlite::Connection,
    name: &str,
    sql: &str,
    clean: bool,
) -> Result<(), rusqlite::Error> {
    if clean {
        conn.execute(&format!("DROP VIEW IF EXISTS {name};"), [])?;
    }

    conn.execute_batch(sql)?;

    println!("View '{name}' created successfully.");

    Ok(())
}

pub fn format_row(
    row: &rusqlite::Row<'_>,
    max_len_per_column: impl Iterator<Item = usize>,
) -> Result<Vec<String>, rusqlite::Error> {
    max_len_per_column
        .enumerate()
        .map(|(i, max_len)| {
            let s = db_value_to_string(row.get(i)?);
            if max_len > 0 {
                Ok(utils::str::truncate_string(&s, max_len))
            } else {
                Ok(s)
            }
        })
        .collect::<Result<Vec<_>, rusqlite::Error>>()
}

/// 引数 `columns` では、各カラムについて、ヘッダーの表示名と値の最大文字数をタプルで渡します。
/// 最大文字数が 0 であれば切り詰めを行いません。
pub fn print_select_query(
    select_statement: &mut rusqlite::Statement<'_>,
    columns: &[(&str, usize)],
) -> Result<(), DbError> {
    let rows = select_statement
        .query_map([], |row| format_row(row, columns.iter().map(|col| col.1)))?
        .map(|e| e.map_err(DbError::Sqlite));
    let headers = columns.iter().map(|col| col.0).collect::<Vec<&str>>();

    utils::io::print_table(&headers, rows)
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

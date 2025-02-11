use crate::db::error::DbError;
use std::io::Write;

pub fn list_files_with_extensions(
    dir: &str,
    extensions: &[&str],
) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut entries = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();

        let valid_extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| extensions.iter().any(|&e| ext.eq_ignore_ascii_case(e)))
            .unwrap_or(false);

        if valid_extension {
            entries.push(path);
        }
    }

    entries.sort_by_key(|path| {
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
    });

    Ok(entries)
}

pub fn confirm_continue() -> Result<bool, DbError> {
    let mut stdout = std::io::stdout();
    let stdin: std::io::Stdin = std::io::stdin();

    loop {
        stdout.write_all(b"Continue? [y/n]\n")?;
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => stdout.write_all(b"Please enter 'y' or 'n'.\n")?,
        }
    }
}

pub fn create_table(
    conn: &rusqlite::Connection,
    name: &str,
    sql: &str,
    clean: bool,
) -> Result<(), DbError> {
    if clean {
        conn.execute(&format!("DROP TABLE IF EXISTS {name};"), [])?;
    }

    conn.execute_batch(sql)?;

    println!("Table '{name}' created successfully.");

    Ok(())
}

pub fn print_select_query(
    select_statement: &mut rusqlite::Statement<'_>,
    column_names: &[&str],
) -> Result<(), DbError> {
    let column_count = column_names.len();
    let rows = select_statement.query_map([], |row| {
        (0..column_count)
            .map(|i| {
                let s = db_value_to_string(row.get(i)?);
                Ok(truncate_string(&s, 30))
            })
            .collect::<Result<Vec<_>, _>>()
    })?;

    let mut stdout = std::io::stdout();
    stdout.write_all(b"\n")?;

    let mut writer = tabwriter::TabWriter::new(std::io::stdout());
    writeln!(writer, "{}", column_names.join("\t"))?;
    for row in rows {
        writeln!(writer, "{}", row?.join("\t"))?;
    }
    writer.flush()?;

    stdout.write_all(b"\n")?;
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

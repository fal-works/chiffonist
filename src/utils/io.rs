use crate::error::DbError;
use std::io::Write;

pub fn list_files_with_extensions<P: AsRef<std::path::Path>>(
    dir: P,
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

pub fn print_table<I>(headers: &[&str], rows: I) -> Result<(), DbError>
where
    I: Iterator<Item = Result<Vec<String>, DbError>>,
{
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\n")?;

    let mut writer = tabwriter::TabWriter::new(std::io::stdout());
    write_row(&mut writer, headers.iter().copied())?;
    for row in rows {
        write_row(&mut writer, row?.iter().map(|e| e.as_str()))?;
    }
    writer.flush()?;

    stdout.write_all(b"\n")?;

    Ok(())
}

fn write_row<'a>(
    writer: &mut tabwriter::TabWriter<std::io::Stdout>,
    mut columns: impl Iterator<Item = &'a str>,
) -> Result<(), std::io::Error> {
    match columns.next() {
        Some(first) => write!(writer, "{first}")?,
        None => return Ok(()),
    };

    for col in columns {
        write!(writer, "\t{col}")?;
    }

    writer.write_all(b"\n")?;

    Ok(())
}

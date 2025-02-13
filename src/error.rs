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
            DbError::Std(err) => write!(f, "Std Error: {}", err),
            DbError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<csv::Error> for DbError {
    fn from(err: csv::Error) -> Self {
        DbError::Csv(err)
    }
}
impl From<serde_yaml::Error> for DbError {
    fn from(err: serde_yaml::Error) -> Self {
        DbError::Yaml(err)
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        DbError::Sqlite(err)
    }
}

impl From<std::io::Error> for DbError {
    fn from(err: std::io::Error) -> Self {
        DbError::Std(err)
    }
}

impl From<String> for DbError {
    fn from(err: String) -> Self {
        DbError::Other(err)
    }
}

impl From<&str> for DbError {
    fn from(err: &str) -> Self {
        DbError::Other(err.into())
    }
}

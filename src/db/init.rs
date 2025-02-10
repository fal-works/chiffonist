use crate::db::error::DbError;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    let conn = rusqlite::Connection::open("data/transactions.db")?;

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
        conn.execute(&format!("DROP TABLE IF EXISTS {name};"), [])?;
    }

    conn.execute(sql, [])?;

    println!("Table '{name}' prepared successfully.");

    Ok(())
}

mod init;
mod print;
use crate::error::DbError;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn print_summary() -> Result<(), DbError> {
    print::print_transaction_summary()
}

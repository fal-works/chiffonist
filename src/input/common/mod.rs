use crate::error::DbError;

mod init;
mod load;

pub fn create_tables(clean: bool) -> Result<(), DbError> {
    init::create_tables(clean)
}

pub fn load_files() -> Result<(), DbError> {
    load::load_map_channel_group_to_channel("data/input/channel-groups.yaml")?;

    Ok(())
}

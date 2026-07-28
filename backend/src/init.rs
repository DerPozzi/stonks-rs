use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::Connection;

use crate::database::database::create_tables;

pub fn create_if_no_cfg(config_path: PathBuf) -> Result<()> {
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap())?;
    }

    fs::write(config_path, "")?;
    Ok(())
}

pub fn open_database(home_path: PathBuf) -> Result<Connection> {
    let db_path = home_path.join(".stonks-rs");
    let db_path = format!("{}stonks-rs.db", db_path.display());
    let conn = Connection::open(db_path)?;

    create_tables(&conn)?;

    Ok(conn)
}

use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::Connection;

use crate::database::database::create_tables;

pub fn open_database(home_path: PathBuf) -> Result<Connection> {
    let db_path = home_path.join(".stonks-rs/").join("stonks-rs.db");
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;

    create_tables(&conn)?;

    Ok(conn)
}

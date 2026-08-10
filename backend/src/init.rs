use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::Connection;

use crate::database::database::create_tables;

pub fn open_database(home_path: PathBuf) -> Result<Connection> {
    let db_path = home_path.join(".stonks-rs/");
    let db_path = format!("{}stonks-rs.db", db_path.display());
    let conn = Connection::open(db_path)?;

    create_tables(&conn)?;

    Ok(conn)
}

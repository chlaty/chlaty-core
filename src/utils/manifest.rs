use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::DEFAULT_PLUGIN_DIRECTORY;

/// Ensures the given table exists with the required schema, then returns the DB connection.
pub fn get_db(table: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    let plugin_dir = PathBuf::from(env::var("CHLATY_PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }

    // Open or create the SQLite database file
    let conn = Connection::open(plugin_dir.join("manifest.db"))?;

    // Prepare the SQL statement with dynamic table name
    let create_stmt = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            plugin_id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            version TEXT NOT NULL,
            plugin_path TEXT NOT NULL
        )",
        table
    );

    // Execute the table creation
    conn.execute(&create_stmt, [])?;

    Ok(conn)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub title: String,
    pub version: String,
    pub plugin_path: String
}

pub fn get_all(source: &str) -> Result<HashMap<String, PluginInfo>, Box<dyn std::error::Error>> {
    let conn = get_db(source)?;
    let mut stmt = conn.prepare(&format!(
        "SELECT plugin_id, title, version, plugin_path FROM {}",
        source
    ))?;

    let rows = stmt.query_map([], |row| {
        let plugin_id: String = row.get(0)?;
        let plugin = PluginInfo {
            title: row.get(1)?,
            version: row.get(2)?,
            plugin_path: row.get(3)?,
        };
        Ok((plugin_id, plugin))
    })?;

    let map = rows.collect::<Result<HashMap<_, _>, _>>()?;
    Ok(map)
}

pub fn get(source: &str, plugin_id: &str) -> Result<Option<PluginInfo>, Box<dyn std::error::Error>> {
    let conn = get_db(source)?;

    // Check if the table exists
    let exists_stmt = "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)";
    let table_exists: bool = conn.query_row(exists_stmt, params![source], |row| row.get(0))?;

    if !table_exists {
        return Ok(None);
    }

    // Query for the plugin by ID
    let query = format!(
        "SELECT plugin_id, title, version, plugin_path FROM {} WHERE plugin_id = ?1",
        source
    );

    let result = conn.query_row(&query, params![plugin_id], |row| {
        Ok(PluginInfo {
            title: row.get(1)?,
            version: row.get(2)?,
            plugin_path: row.get(3)?,
        })
    });

    match result {
        Ok(plugin) => Ok(Some(plugin)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}


pub fn save(
    source: &str,
    plugin_id: &str,
    title: &str,
    version: &str,
    plugin_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_db(source)?;
    
    let stmt = format!(
            "INSERT OR REPLACE INTO {} (plugin_id, title, version, plugin_path)
            VALUES (?1, ?2, ?3, ?4)",
        source
    );

    conn.execute(&stmt, params![plugin_id, title, version, plugin_path])?;
    Ok(())
}


pub fn remove(source: &str, plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_db(source)?; // Ensure table exists

    let stmt = format!(
        "DELETE FROM {} WHERE plugin_id = ?1",
        source
    );

    conn.execute(&stmt, params![plugin_id])?;
    Ok(())
}
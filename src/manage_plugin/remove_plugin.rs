use std::path::{PathBuf};
use std::fs;

use crate::{ DEFAULT_PLUGIN_DIRECTORY };




pub fn new(plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let plugin_dir = PathBuf::from(std::env::var("PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }

    let tree = sled::open(&plugin_dir.join("manifest"))?;
    tree.remove(plugin_id.as_bytes())?;

    return Ok(());
    
}
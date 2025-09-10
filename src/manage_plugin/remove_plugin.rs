use std::path::{PathBuf};
use std::fs;
use serde_json::{ from_str };
use std::str::{from_utf8};
use serde::{Deserialize, Serialize};

use crate::{ DEFAULT_PLUGIN_DIRECTORY };


#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub plugin_path: String
}

pub fn new(plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let plugin_dir = PathBuf::from(std::env::var("CHLATY_PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }

    let tree = sled::open(&plugin_dir.join("manifest"))?;
    if let Some(value)= tree.get(plugin_id.as_bytes())? {
        
        let value: PluginInfo = from_str(from_utf8(&value)?)?;

        let plugin_path = PathBuf::from(&value.plugin_path);

        if plugin_path.exists() {
            fs::remove_file(&plugin_path)?;
        }


    }else{
        return Err(format!("Plugin not ({}) found", plugin_id).into());
    }

    tree.remove(plugin_id.as_bytes())?;

    tree.flush()?;


    return Ok(());
    
}
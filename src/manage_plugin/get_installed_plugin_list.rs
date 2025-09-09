use serde_json::{ from_str };
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::str::{from_utf8};
use std::path::{PathBuf};
use sled::{IVec};

use crate::{ DEFAULT_PLUGIN_DIRECTORY };


#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub title: String,
    pub version: String,
    pub plugin_path: String
}


pub fn new() -> Result<HashMap<String, PluginInfo>, Box<dyn std::error::Error>> {
    let plugin_dir = PathBuf::from(std::env::var("PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }
    let mut data: HashMap<String, PluginInfo> = HashMap::new();
    let tree = sled::open(&plugin_dir.join("manifest"))?;
    for result in tree.iter() {
        let (key, value): (IVec, IVec) = result?;
        let value: PluginInfo = from_str(from_utf8(&value)?)?;
        data.insert(from_utf8(&key)?.to_string(), value);
    }

    return Ok(data);
    
}
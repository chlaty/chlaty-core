
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

use crate::utils::manifest::remove;
use crate::utils::get_lib_extension;
use crate::utils::plugin_loader;
use crate::DEFAULT_PLUGIN_DIRECTORY;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub plugin_path: String
}

pub fn new(source: &str, plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    plugin_loader::remove(plugin_id);
    remove(source, plugin_id)?;

    let plugin_dir = PathBuf::from(std::env::var("CHLATY_PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
        
    let source_dir = plugin_dir.join(source);

    let lib_extension = get_lib_extension::new()?;
    let file_name = format!("lib-{}{}", &plugin_id, &lib_extension);

    let file_path = source_dir.join(file_name);

    if file_path.exists() {
        fs::remove_file(file_path)?;    
    }

    return Ok(());
    
}
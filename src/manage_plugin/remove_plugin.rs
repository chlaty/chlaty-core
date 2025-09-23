
use serde::{Deserialize, Serialize};

use crate::utils::manifest::remove;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub plugin_path: String
}

pub fn new(source: &str, plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    

    remove(source, plugin_id)?;

    return Ok(());
    
}
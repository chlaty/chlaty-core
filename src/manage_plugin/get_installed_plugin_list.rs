use std::collections::HashMap;
use crate::utils::manifest::get_all;
use crate::utils::manifest::PluginInfo;

pub fn new(source: &str) -> Result<HashMap<String, PluginInfo>, Box<dyn std::error::Error>> {
    let data = get_all(source)?;
    return Ok(data);
    
}
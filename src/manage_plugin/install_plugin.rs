
use serde_json::{from_reader, json, Value, to_string};
use std::io::{BufReader};
use reqwest;
use std::{env::consts};
use std::path::{PathBuf};
use std::fs;
use sled;
use serde::{Deserialize, Serialize};

use crate::utils::{get_lib_extension, download};
use crate::manage_plugin::remove_plugin;
use crate::manage_plugin::get_plugin_release;
use crate::manage_plugin::get_plugin_release::GetPluginRelease;
use crate::{DEFAULT_PLUGIN_DIRECTORY};






#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub title: String,
    pub manifest: String
}

pub fn new<F>(
    source: &str,
    id: &str, 
    version: &str,
    plugin_manifest_info: PluginManifest, 
    callback: F
) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn(usize, usize),

{

    let get_plugin_release_result: GetPluginRelease;
    match get_plugin_release::new(plugin_manifest_info.manifest.as_str(), version) {
        Ok(result) => get_plugin_release_result = result,
        Err(e) => return Err(e),
    }
    
    let client = reqwest::blocking::Client::new();
    let res = client.get(get_plugin_release_result.url).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        let release_manifest_data: Value = from_reader(manifest_reader)?;
        
        let data = release_manifest_data.get(consts::OS).and_then(|a| a.get(consts::ARCH))
            .ok_or("Unable to find supported OS and Arch inside manifest")?;


        let file_url = data.get("file")
            .ok_or("Unable to find file url inside manifest")?
            .as_str().ok_or("Unable to convert file url to str")?;

        let lib_extension = get_lib_extension::new()?;
        let file_name = format!("lib-{}{}", &id, &lib_extension);
        let plugin_dir = PathBuf::from(std::env::var("CHLATY_PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));

        if !plugin_dir.exists() {
            fs::create_dir_all(&plugin_dir)?;
        }


        let output_file = PathBuf::from(&plugin_dir).join(&file_name);
        
        if output_file.exists() {
            fs::remove_file(&output_file)?;
        }

        download::new(
            file_url, 
            &output_file.display().to_string(), 
            callback
        )?;

        /* Save plugin info */
        let manifest_dir = plugin_dir.join("manifest");

        let source_dir = manifest_dir.join(source);

        if !source_dir.exists() {
            fs::create_dir_all(&source_dir)?;
        }

        {
            let tree = sled::open(&source_dir)?;
            tree.remove(&id.as_bytes())?;
            tree.flush()?;
        }

        let tree = sled::open(&source_dir)?;

        let store_value = json!({
            "title": plugin_manifest_info.title,
            "version": get_plugin_release_result.version,
            "plugin_path": &output_file.to_str().ok_or("Unable to convert output path to str")?
            
        });
        tree.insert(
            &id.as_bytes(),
            to_string(&store_value)?.as_bytes()
        )?;
        tree.flush()?;
        /* === */
        
        return Ok(());
    }else{
        return Err("Unable to download manifest".into());
    }
}
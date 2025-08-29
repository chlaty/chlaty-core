
use serde_json::{Value, from_reader};
use std::io::{BufReader};
use reqwest;
use std::{env::consts};
use std::path::{PathBuf};
use std::fs;

use crate::utils::{get_lib_extension, download};
use crate::{DEFAULT_PLUGIN_DIRECTORY};


pub fn new(id:&str, manifest_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("ID: {} | URL: {}", &id, &manifest_url);
    println!("OS: {} | Arch: {}", consts::OS, consts::ARCH);

    let mut manifest_data: Value = Value::Null;
    
    let client = reqwest::blocking::Client::new();
    let res = client.get(manifest_url).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        manifest_data = from_reader(manifest_reader)?;
        
    }
    
    
    let data = manifest_data.get(consts::OS).and_then(|a| a.get(consts::ARCH))
        .ok_or("Unable to find supported OS and Arch inside manifest")?;

    let file_url = data.get("file")
        .ok_or("Unable to find file url inside manifest")?
        .as_str().ok_or("Unable to convert file url to str")?;

    let lib_extension = get_lib_extension::new()?;
    let file_name = format!("{}{}", "lib-hianime", &lib_extension);
    let plugin_dir = PathBuf::from(std::env::var("PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }

    let output_file = PathBuf::from(&plugin_dir).join(&file_name);
    
    download::new(file_url, &output_file.to_str().ok_or("Unable to convert output path to str")?, 
        |current_size, total_size| {
            println!("Downloaded {} of {}", current_size, total_size);
        }
    )?;

    println!("Result: {:?}", &plugin_dir);
    
    return Ok(());
}
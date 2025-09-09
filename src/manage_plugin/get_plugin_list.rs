use serde_json::{from_reader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader};
use reqwest;
use crate::{ MANIFEST_URL, USE_LOCAL_MANIFEST, LOCAL_MANIFEST_PATH };


#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub title: String,
    pub manifest: String
}

/// Gets the manifest data from the remote url or the local file if
/// `USE_LOCAL_MANIFEST` is set to true in debug mode.
///
/// The function will return an error if the manifest file does not
/// exist or if there is an error while downloading the manifest.
///
/// The function will return `Ok(Value)` if the download is successful.
pub fn new() -> Result<HashMap<String, PluginInfo>, Box<dyn std::error::Error>> {
    let mut manifest_data: HashMap<String, PluginInfo> = HashMap::new();

    if USE_LOCAL_MANIFEST {
        println!("Using local manifest.");
        let manifest_file = fs::File::open(LOCAL_MANIFEST_PATH)?;
        let manifest_reader = BufReader::new(manifest_file);
        manifest_data = from_reader(manifest_reader)?;
        
    }else{
        let client = reqwest::blocking::Client::new();
        let res = client.get(MANIFEST_URL).send()?;

        if res.status().is_success() {
            let manifest_reader = BufReader::new(res);
            manifest_data = from_reader(manifest_reader)?;
            
        }
    }
    return Ok(manifest_data);

}
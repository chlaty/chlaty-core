use serde_json::{from_reader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader};
use reqwest;
use crate::{ MANIFEST_URL, USE_LOCAL_MANIFEST, LOCAL_MANIFEST_PATH };


#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub fn new(source: &str) -> Result<HashMap<String, PluginInfo>, Box<dyn std::error::Error>> {
    let mut data: HashMap<String, PluginInfo> = HashMap::new();

    if USE_LOCAL_MANIFEST {
        println!("Using local manifest.");
        let manifest_file = fs::File::open(LOCAL_MANIFEST_PATH)?;
        let reader = BufReader::new(manifest_file);
        let result: HashMap<String, HashMap<String, PluginInfo>> = from_reader(reader)?;
        data = result.get(source).ok_or("Unable to find source")?.clone();
    }else{
        let client = reqwest::blocking::Client::new();
        let res = client.get(MANIFEST_URL).send()?;

        if res.status().is_success() {
            let reader = BufReader::new(res);
            let result: HashMap<String, HashMap<String, PluginInfo>> = from_reader(reader)?;
            
            data = result.get(source).ok_or("Unable to find source")?.clone();
        }
    }



    return Ok(data);

}
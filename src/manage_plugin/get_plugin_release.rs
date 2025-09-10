use serde_json::{from_reader, Value};
use std::io::{BufReader};
use reqwest;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPluginRelease{
    pub url: String,
    pub version: String
}
pub fn new(manifest_url: &str, version: &str) -> Result<GetPluginRelease, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    let res = client.get(manifest_url).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        let manifest_data: Value = from_reader(manifest_reader)?;
        let version_to_use = if version == "latest" {
            manifest_data.get("latest-version")
                .ok_or("Unable to find latest version inside manifest")?
                .as_str().ok_or("Unable to convert latest version to str")?
        }else{version};
        
        
        let release_url = manifest_data.get(version_to_use)
            .ok_or("Unable to find release url inside manifest")?
            .as_str().ok_or("Unable to convert release url to str")?
            .to_string();


        return Ok(GetPluginRelease { url: release_url, version: version_to_use.to_string() });

    }else{
        return Err("Unable to download manifest".into());
    }

    

}
use serde_json::{Value,json, from_reader};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Error};
use reqwest;


/// Downloads a manifest file from a given URL and parses it as JSON.
///
/// The function will return an error if the manifest file does not exist or if there is an error while downloading the manifest.
///
/// The function will return `Ok(Value)` if the download is successful.
pub fn new(manifest_url: &str) -> Result<Value, Box<dyn std::error::Error>> {

    let mut manifest_data: Value = Value::Null;
    
    let client = reqwest::blocking::Client::new();
    let res = client.get(manifest_url).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        manifest_data = from_reader(manifest_reader)?;
        
    }
    
    
    return Ok(manifest_data);

}
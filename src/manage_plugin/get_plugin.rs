use serde_json::{Value,json, from_reader};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Error};
use reqwest;


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
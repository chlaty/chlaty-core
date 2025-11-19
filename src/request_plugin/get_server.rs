use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::ffi::{ CString, c_char, CStr};

use crate::utils::plugin_loader;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timeline {
    pub start: usize,
    pub end: usize
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceInfo {
    pub file: String,

    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackInfo {
    pub file: String,
    pub label: Option<String>,
    pub kind: String,
    pub default: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub origin: String,
    pub referer: String,
    pub playlist_base_url: String,
    pub segment_base_url: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataResult { 
    pub intro: Option<Timeline>,
    pub outro: Option<Timeline>,
    pub sources: Vec<SourceInfo>,
    pub tracks: Vec<TrackInfo>,
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerResult { 
    pub data: DataResult,
    pub config: Config
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestResult {
    pub status: bool,
    pub message: String,
    pub data: DataResult,
    pub config: Config
}


pub fn new(source: &str, plugin_id: &str, index: usize, id: &str) -> Result<ServerResult, Box<dyn std::error::Error>>{

    
    let request_result: RequestResult;


    unsafe {
        // Load the symbol
        let get_server = plugin_loader::get(source, plugin_id)?.get_server;
        let free_ptr = plugin_loader::get(source, plugin_id)?.free_ptr;


        // Prepare args
        let args = CString::new(to_string(&json!({
            "index": index,
            "id": id,
        }))?)?;
        
        let result_ptr = get_server(args.as_ptr());

        if result_ptr.is_null() {
            return Err("[get_server] result_ptr is null.")?;
        }

        request_result = from_str(&CStr::from_ptr(result_ptr).to_str()?.to_owned())?;
        println!("request_result: {:#?}", request_result);
        free_ptr(result_ptr as *mut c_char);
        
        
    }

    if !request_result.status {
        return Err(format!("[get_server]: {}", request_result.message))?;
    }
    
    return Ok(ServerResult{
        data: request_result.data,
        config: request_result.config
    });
}
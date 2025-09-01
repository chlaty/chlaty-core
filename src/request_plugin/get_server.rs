use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::ffi::{ CString, c_char};
use std::fs;
use std::path::PathBuf;
use libloading::{Library, Symbol};
use std::str::{from_utf8};
use sled;


use crate::{ DEFAULT_PLUGIN_DIRECTORY };

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub start: usize,
    pub end: usize
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    pub file: String,

    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub file: String,

    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub referer: String,
    pub playlist_base_url: String,
    pub segment_base_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DataResult { 
    pub intro: Timeline,
    pub outro: Timeline,
    pub sources: Vec<SourceInfo>,
    pub tracks: Vec<TrackInfo>,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnResult { 
    pub data: DataResult,
    pub config: Config
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub title: String,
    pub version: String,
    pub plugin_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResult {
    pub status: bool,
    pub message: String,
    pub data: DataResult,
    pub config: Config
}


pub fn new(plugin_id: &str, id: &str) -> Result<ReturnResult, Box<dyn std::error::Error>>{

    let plugin_dir = PathBuf::from(std::env::var("PLUGIN_DIRECTORY").unwrap_or(DEFAULT_PLUGIN_DIRECTORY.to_string()));
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir)?;
    }
    let tree = sled::open(&plugin_dir.join("manifest"))?;

    let value = tree.get(plugin_id)?.ok_or("Plugin not found")?;

    let plugin_info: PluginInfo = from_str(from_utf8(&value)?)?;
    let plugin_path = PathBuf::from(&plugin_info.plugin_path);
    
    let request_result: RequestResult;
    unsafe {
        let lib = Library::new(plugin_path).expect("Failed to load DLL");

        // Load the symbol
        let callable: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
            lib.get(b"get_server").expect("Failed to load symbol");

        // Prepare args
        let args = CString::new(to_string(&json!({
            "id": id,
        }))?).expect("CString::new failed while preparing args");
        
        // Call the function
        let result_ptr = callable(args.as_ptr());
        

        // Convert result to Rust method
        request_result = from_str(&CString::from_raw(result_ptr as *mut c_char).into_string()?)?;

        
        if !request_result.status {
            return Err(format!("[Request failed]: {}", request_result.message).into());
        }
    }

    Ok(ReturnResult{
        data: request_result.data,
        config: request_result.config
    })
}
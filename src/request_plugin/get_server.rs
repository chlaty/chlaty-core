use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::ffi::{ CString, c_char, CStr};
use std::path::PathBuf;
use libloading::{Library, Symbol};

use crate::utils::manifest::get;

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


pub fn new(source: &str, plugin_id: &str, id: &str) -> Result<ServerResult, Box<dyn std::error::Error>>{

    let plugin_info = get(source, plugin_id)?.ok_or("Plugin not found")?;
    let plugin_path = PathBuf::from(&plugin_info.plugin_path);
    
    let request_result: RequestResult;

    let lib = unsafe { Library::new(plugin_path)?};

    unsafe {
        // Load the symbol
        let callable: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
            lib.get(b"get_server")?;
        
        let free_ptr: Symbol<unsafe extern "C" fn(*mut c_char)> =
            lib.get(b"free_ptr")?;

        // Prepare args
        let args = CString::new(to_string(&json!({
            "id": id,
        }))?)?;
        
        let result_ptr = callable(args.as_ptr());
        request_result = from_str(CStr::from_ptr(result_ptr).to_str()?.to_owned().as_str())?;
        free_ptr(result_ptr as *mut c_char);
        
        
    }

    if !request_result.status {
        return Err(format!("[Request failed]: {}", request_result.message).into());
    }
    
    return Ok(ServerResult{
        data: request_result.data,
        config: request_result.config
    });
}
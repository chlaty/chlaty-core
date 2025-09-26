use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::collections::HashMap;
use std::ffi::{CString, c_char, CStr};
use std::path::PathBuf;
use libloading::{Library, Symbol};


use crate::utils::manifest::get;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataResult { 
    pub id: String,
    pub title: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestResult {
    pub status: bool,
    pub message: String,
    pub data: HashMap<String, Vec<DataResult>>
}


pub fn new(source: &str, plugin_id: &str, season_index: usize, episode_index: usize, episode_id: &str) -> Result<HashMap<String, Vec<DataResult>>, Box<dyn std::error::Error>>{

    let plugin_info = get(source, plugin_id)?.ok_or("Plugin not found")?;

    let plugin_path = PathBuf::from(&plugin_info.plugin_path);
    
    let request_result: RequestResult;

    let lib = unsafe { Library::new(plugin_path).expect("Failed to load shared lib")};


    unsafe {
        // Load the symbol
        let callable: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
            lib.get(b"get_episode_server").expect("Failed to load symbol");
        
        let free_ptr: Symbol<unsafe extern "C" fn(*mut c_char)> =
            lib.get(b"free_ptr").expect("Failed to load symbol");

        // Prepare args
        let args = CString::new(to_string(&json!({
            "season_index": season_index,
            "episode_index": episode_index,
            "episode_id": episode_id,
        }))?).expect("CString::new failed while preparing args");
        
        let result_ptr = callable(args.as_ptr());
        request_result = from_str(CStr::from_ptr(result_ptr).to_str()?.to_owned().as_str())?;
        free_ptr(result_ptr as *mut c_char);

        
        
    }

    if !request_result.status {
        return Err(format!("[get_episode_server] Error: {}", request_result.message).into());
    }

    return Ok(request_result.data);
}
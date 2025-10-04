use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::collections::HashMap;
use std::ffi::{CString, c_char, CStr};


use crate::utils::plugin_loader;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataResult { 
    pub index: usize,
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

    let request_result: RequestResult;

    unsafe {
        let get_episode_server = plugin_loader::get(source, plugin_id)?.get_episode_server;
        let free_ptr = plugin_loader::get(source, plugin_id)?.free_ptr;

        // Prepare args
        let args = CString::new(to_string(&json!({
            "season_index": season_index,
            "episode_index": episode_index,
            "episode_id": episode_id,
        }))?)?;
        
        let result_ptr = get_episode_server(args.as_ptr());

        if result_ptr.is_null() {
            return Err("[get_episode_server] result_ptr is null.")?;
        }

        request_result = from_str(&CStr::from_ptr(result_ptr).to_str()?.to_owned())?;
        free_ptr(result_ptr as *mut c_char);
        
    }

    if !request_result.status {
        return Err(format!("[get_episode_server] Error: {}", request_result.message))?;
    }

    return Ok(request_result.data);
}
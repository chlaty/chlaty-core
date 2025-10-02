use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, json};
use std::ffi::{CString, c_char, CStr};

use std::num::NonZeroUsize;
use std::path::PathBuf;
use libloading::{Library, Symbol};

use crate::utils::manifest::get;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataResult { 
    pub id: String,
    pub title: String,
    pub cover: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub title: String,
    pub version: String,
    pub plugin_path: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestResult {
    pub status: bool,
    pub message: String,
    pub data: Vec<DataResult>
}


pub fn new(source: &str, plugin_id: &str, search: &str, page: NonZeroUsize) -> Result<Vec<DataResult>, Box<dyn std::error::Error>>{

    let plugin_info = get(source, plugin_id)?.ok_or("Plugin not found")?;
    let plugin_path = PathBuf::from(&plugin_info.plugin_path);
    
    let request_result: RequestResult;

    unsafe {
        let lib = Library::new(plugin_path)?;

        // Load the symbol
        let callable: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
            lib.get(b"search")?;

        let free_ptr: Symbol<unsafe extern "C" fn(*mut c_char)> =
            lib.get(b"free_ptr")?;

        // Prepare args
        let args = CString::new(to_string(&json!({
            "search": search,
            "page": page
        }))?)?;
        
        
        let result_ptr = callable(args.as_ptr());

        if result_ptr.is_null() {
            return Err("[search] result_ptr is null.")?;
        }

        request_result = from_str(&CStr::from_ptr(result_ptr).to_str()?.to_owned())?;
        free_ptr(result_ptr as *mut c_char);
    }

    if !request_result.status {
        return Err(format!("[search]: {}", request_result.message))?;
    }

    return Ok(request_result.data);
}

use lazy_static::lazy_static;
use dashmap::DashMap;
use libloading::{Library, Symbol};
use std::{ ffi::c_char};
use std::sync::Arc;
use std::path::PathBuf;

use crate::utils::manifest;

#[derive(Debug)]
pub struct Plugin{
    pub lib: Arc<Library>,
    pub search: unsafe extern "C" fn(*const c_char) -> *const c_char,
    pub get_episode_list: unsafe extern "C" fn(*const c_char) -> *const c_char,
    pub get_episode_server: unsafe extern "C" fn(*const c_char) -> *const c_char,
    pub get_server: unsafe extern "C" fn(*const c_char) -> *const c_char,
    pub free_ptr: unsafe extern "C" fn(*mut c_char),

}

lazy_static! {
    /* <plugin_id, Plugin> */
    pub static ref PLUGIN_REGISTRY: DashMap<String, Arc<Plugin>> = DashMap::new();
    /* --- */

}

pub fn get(source: &str, plugin_id: &str) -> Result<Arc<Plugin>, Box<dyn std::error::Error>> {
    if let Some(plugin) = &PLUGIN_REGISTRY.get(plugin_id) {
        return Ok(plugin.value().clone());
    }else{
        unsafe {
            let plugin_info = manifest::get(source, plugin_id)?.ok_or("Plugin not found")?;

            let plugin_path = PathBuf::from(&plugin_info.plugin_path);

            let lib = Arc::new(Library::new(plugin_path)?);
            let lib_for_symbol = lib.clone();
            
            /* Load the symbol */
            let search: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
                lib_for_symbol.get(b"search")?;

            let get_episode_list: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
                lib_for_symbol.get(b"get_episode_list")?;

            let get_episode_server: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
                lib_for_symbol.get(b"get_episode_server")?;

            let get_server: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
                lib_for_symbol.get(b"get_server")?;
            
            let free_ptr: Symbol<unsafe extern "C" fn(*mut c_char)> =
                lib_for_symbol.get(b"free_ptr")?;
            /* --- */

            let new_loaded_plugin = Arc::new(Plugin{
                lib,
                search: *search,
                get_episode_list: *get_episode_list,
                get_episode_server: *get_episode_server,
                get_server: *get_server,
                free_ptr: *free_ptr
            });

            PLUGIN_REGISTRY.insert(plugin_id.to_string(), new_loaded_plugin.clone());

            return Ok(new_loaded_plugin);
        }
    }
}

pub fn unload(plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = PLUGIN_REGISTRY.remove(plugin_id);
    Ok(())
}


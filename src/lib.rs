

pub const MANIFEST_URL: &str = "https://raw.githubusercontent.com/chlaty/chlaty-core/refs/heads/main/manifest.json";
pub const USE_LOCAL_MANIFEST: bool = false;
pub const LOCAL_MANIFEST_PATH: &str = "manifest.json";

pub const DEFAULT_PLUGIN_DIRECTORY: &str = "plugins";

pub mod utils;
pub mod manage_plugin;
pub mod request_plugin;

#[cfg(test)]
mod test;

use chrono::Utc;
use tokio::spawn;
use tokio::time::{sleep, Duration};

const MAX_PLUGIN_LIFE: usize = 5 * 60_000;

pub fn init() -> () { 

    /* Spawn Worker */
    spawn(async {
        let plugin_registry = &utils::plugin_loader::PLUGIN_REGISTRY;
        loop {
            let now = Utc::now().timestamp_millis() as usize;
            plugin_registry.retain(|_id, plugin| {
                now.saturating_sub(plugin.last_use) <= MAX_PLUGIN_LIFE
            });
            sleep(Duration::from_secs(5)).await;
        }
        
    });
    /* --- */
}
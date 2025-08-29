

pub const MANIFEST_URL: &str = "https://raw.githubusercontent.com/chlaty/chlaty-core/refs/heads/main/manifest.json";
pub const USE_LOCAL_MANIFEST: bool = true;
pub const LOCAL_MANIFEST_PATH: &str = "manifest.json";

pub const DEFAULT_PLUGIN_DIRECTORY: &str = "plugins";

pub mod utils;
pub mod manage_plugin;

#[cfg(test)]
mod test;

use std::env::consts;

pub fn new() -> Result<&'static str, Box<dyn std::error::Error>>{
    return match consts::OS {
        "windows" => Ok(".dll"),
        "linux" => Ok(".so"),
        "macos" => Ok(".dylib"),
        "android" => Ok(".so"),
        _ => Err("Unknown OS".into()),
    };
}

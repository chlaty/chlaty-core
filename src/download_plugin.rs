
use std::env::consts;


pub fn new() -> () {

    println!("OS: {}", consts::OS);       // e.g., "linux", "windows", "macos"
    println!("Arch: {}", consts::ARCH);  

}
use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let debug = env::var("DEBUG").unwrap();
    let command;
    let tundra;

    if target.contains("darwin") {
        tundra = "bin/macosx/tundra/tundra2";
        if debug == "true" {
            command = "macosx-clang-debug";
        } else {
            command = "macosx-clang-release";
        }
    } else if target.contains("windows") {
        tundra = "bin/win32/tundra2";
        if debug == "true" {
            command = "win64-msvc-debug";
        } else {
            command = "win64-msvc-release";
        }
    } else {
        tundra = "tundra2";
        if debug == "true" {
            command = "linux-gcc-debug";
        } else {
            command = "linux-gcc-release";
        }
    }
    
    let _ = Command::new(tundra)
                .arg("-v")
                .arg(command)
                .output()
                .unwrap_or_else(|e| { 
        panic!("Unable to execute tundra: {}", e)
    });
}

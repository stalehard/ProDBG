use std::path::{Path, PathBuf};
use std::fs;

extern crate libc;
extern crate notify;
extern crate dynamic_reload;
extern crate prodbg_api;

pub mod plugin_handler;
pub mod session;
pub mod plugins;
pub mod standard_plugin;
pub mod view_plugins;

pub use dynamic_reload::*;

/// Formats dll name on Windows ("test_foo" -> "test_foo.dll")
#[cfg(target_os="windows")]
pub fn get_dynamiclib_name(name: &str) -> String {
    format!("{}.dll", name)
}

/// Formats dll name on Mac ("test_foo" -> "libtest_foo.dylib")
#[cfg(target_os="macos")]
pub fn get_dynamiclib_name(name: &str) -> String {
    format!("lib{}.dylib", name)
}

/// Formats dll name on *nix ("test_foo" -> "libtest_foo.so")
#[cfg(any(target_os="linux",
            target_os="freebsd",
            target_os="dragonfly",
            target_os="netbsd",
            target_os="openbsd"))]
pub fn get_dynamiclib_name(name: &str) -> String {
    format!("lib{}.so", name)
}

///
///
///
pub fn does_file_exists(path: &PathBuf) -> bool {
    match fs::metadata(&path) {
        Ok(md) => {
            if md.is_file() {
                return true;
            }
        },
        _ => {
            return false;
        }
    }

    return false;
}

///
/// Searches the paths given at the new function for the dynamic libabry
/// and returns Option<PathBuf> if found otherwise None
///
pub fn find_dynamic_lib_in_path(search_paths: &[&'static str], name: &str) -> Option<PathBuf> {
    let library_name = get_dynamiclib_name(name);
    for p in search_paths.iter() {
        let path = Path::new(p).join(&library_name);
        match fs::metadata(&path) {
            Ok(md) => {
                if md.is_file() {
                    return Some(path);
                }
            }
            _ => (),
        }
    }

    None
}





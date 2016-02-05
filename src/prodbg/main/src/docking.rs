use libloading::{Library};
use std::path::{PathBuf};
use std::env;
use core::{find_dynamic_lib_in_path, get_dynamiclib_name, does_file_exists};

pub struct Docking {
    pub lib: Library,
}


impl Docking {
    pub fn new(plugin_name: &'static str) -> Option<Docking> {
       if let Some(file) = Self::find_plugin(plugin_name) {
            match Library::new(&file) {
                Ok(l) => return Some(Docking {
                    lib: l
                }),
                Err(e) => {
                    println!("Unable to load {} error: {}", file.to_str().unwrap(), e);
                }
            }
        }
        None
    }

    fn find_plugin(plugin_name: &'static str) -> Option<PathBuf> {
        // First we try to find the file in the same directory as the executable, if we can't find
        // it there we search backwards a bit
        let exe_path = env::current_exe().unwrap();
        let plugin_path = exe_path.join(get_dynamiclib_name(plugin_name)); 
        if does_file_exists(&plugin_path) {
            return Some(plugin_path);
        }

        return find_dynamic_lib_in_path(&["../", "../..", "../../", "../../../"], plugin_name);
    }
}

extern crate libloading;

use libc::{c_char, c_void, c_int};
use std::path::{Path, PathBuf};
use std::ffi::CStr;
use std::rc::Rc;
use std::mem::transmute;
use std::fs;

use self::libloading::*;

// static STANDARD_PLUGIN_TYPES: [&'static str; 2] = ["ProDBG View", "ProDBG Backend"];

#[repr(C)]
pub struct CViewPlugin {
    pub name: *const c_char,
    pub create_instance: Option<fn(ui_api: *const c_void,
                                   service_func: extern "C" fn(service: *const c_char)
                                                               -> *mut c_void)
                                   -> *mut c_void>,
    pub destroy_instance: Option<fn(*mut c_void)>,
    pub update: Option<fn(ptr: *mut c_void,
                          ui: *mut c_void,
                          reader: *mut c_void,
                          writer: *mut c_void)
                         >,
    pub save_state: Option<fn(*mut c_void)>,
    pub load_state: Option<fn(*mut c_void)>,
}

#[repr(C)]
pub struct CBackendPlugin {
    pub name: *const c_char,
    pub create_instance: Option<fn(service_func: extern "C" fn(service: *const c_char)
                                                               -> *mut c_void)
                                   -> *mut c_void>,
    pub destroy_instance: Option<fn(*mut c_void)>,
    pub register_menu: Option<fn() -> *mut c_void>,
    pub update: Option<fn(ptr: *mut c_void,
                          a: *mut c_int,
                          ra: *mut c_void,
                          wa: *mut c_void)
                         >,
}

// We will need version handling for plugins later on but should be fine for now.
struct Plugin<T> {
    lib: Rc<Library>,
    path: PathBuf,
    plugin_funcs: *mut T,
}

pub struct PluginHandler<'a> {
    view_plugins: Vec<Plugin<CViewPlugin>>,
    backend_plugins: Vec<Plugin<CBackendPlugin>>,
    search_paths: &'a [&'static str],
}

pub struct CallbackData<'a> {
    handler: &'a PluginHandler<'a>,
    lib: Rc<Library>,
    path: PathBuf,
}

type RegisterPlugin = unsafe fn(pt: *const c_char,
                                plugin: *mut c_void,
                                size: c_int,
                                data: *mut CallbackData)
                               ;

unsafe fn add_plugin<T>(plugins: &mut Vec<Plugin<T>>,
                        plugin_type: *const c_char,
                        plugin: *mut c_void,
                        cb: &mut CallbackData,
                        type_name: &str) {
    for plugin in plugins.iter() {
        if cb.path == plugin.path {
            return;
        }
    }

    let ptype = CStr::from_ptr(plugin_type).to_string_lossy().into_owned();

    if !ptype.contains(type_name) {
        return;
    }

    let plugin = Plugin {
        path: cb.path.clone(),
        lib: cb.lib.clone(),
        plugin_funcs: transmute(plugin), 
    };

    plugins.push(plugin);
}

unsafe fn register_plugin_callback(plugin_type: *const c_char,
                                   plugin: *mut c_void,
                                   size: c_int,
                                   ph: *mut CallbackData) {
    let t = &mut (*ph); 
    add_plugin::<CViewPlugin>(&mut t.handler.view_plugins, plugin_type, plugin, t, "View");
    add_plugin::<CBackendPlugin>(&mut t.handler.backend_plugins, plugin_type, plugin, t, "Backend");
}

impl<'a> PluginHandler<'a> {
    pub fn new(search_paths: &'a [&'static str]) -> PluginHandler<'a> {
        PluginHandler {
            backend_plugins: Vec::new(),
            view_plugins: Vec::new(),
            search_paths: search_paths,
        }
    }

    fn search_plugin(&self, name: &String) -> Option<PathBuf> {
        for p in self.search_paths.iter() {
            let path = Path::new(p).join(name);
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

    unsafe fn load_plugin(&mut self, path: PathBuf) -> bool {
        match Library::new(&path) {
            Ok(library) => {
                let lib = Rc::new(library);

                let init_plugin: Result<Symbol<extern "C" fn(RegisterPlugin, *mut CallbackData)>> =
                    lib.get(b"InitPlugin");

                match init_plugin {
                    Ok(init_fun) => {
                        let mut callback_data = CallbackData {
                            handler: self,
                            lib: lib.clone(),
                            path: path,
                        };

                        init_fun(register_plugin_callback, &mut callback_data);
                        true
                    }
                    Err(e) => {
                        println!("Unable to find InitPlugin in {} error: {}",
                                 path.to_str().unwrap(),
                                 e);
                        false
                    }
                }
            }

            Err(e) => {
                println!("Unable to load {} error: {}", path.to_str().unwrap(), e);
                false
            }
        }
    }

    pub fn add_plugin(&mut self, clean_name: &str) -> bool {
        let name = Self::format_name(clean_name);

        if let Some(plugin_path) = Self::search_plugin(self, &name) {
            unsafe { Self::load_plugin(self, plugin_path) }
        } else {
            println!("Unable to find plugin {}", clean_name);
            false
        }
    }

    #[cfg(target_os="windows")]
    fn format_name(name: &str) -> String {
        format!("{}.dll", name)
    }

    #[cfg(target_os="macos")]
    fn format_name(name: &str) -> String {
        format!("lib{}.dylib", name)
    }

    #[cfg(any(target_os="linux",
              target_os="freebsd",
              target_os="dragonfly",
              target_os="netbsd",
              target_os="openbsd"))]
    fn format_name(name: &str) -> String {
        format!("lib{}.so", name)
    }

    pub fn add_non_standard(name: &str) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serach_paths_find() {
        // This actually doesn't search for a plugin file but that doesn't really matter
        let search_paths: [&str; 2] = ["src", "other_path"];
        let plugin_handler = PluginHandler::new(&search_paths);
        assert_eq!(plugin_handler.search_plugin(&"main.rs".to_string()).is_some(),
                   true);
    }

    #[test]
    fn test_serach_paths_no_find() {
        // This actually doesn't search for a plugin file but that doesn't really matter
        let search_paths: [&str; 2] = ["src", "other_path"];
        let plugin_handler = PluginHandler::new(&search_paths);
        assert_eq!(plugin_handler.search_plugin(&"main_no_find.rs".to_string()).is_none(),
                   true);
    }

    #[test]
    fn test_load_plugin_init() {
        let search_paths: [&str; 1] = ["t2-output/macosx-clang-debug-default"];
        let mut plugin_handler = PluginHandler::new(&search_paths);
        assert_eq!(plugin_handler.add_plugin(&"breakpoints_plugin".to_string()),
                   true);
    }

    #[test]
    #[cfg(target_os="windows")]
    fn test_format_name() {
        assert_eq!("test_plugin.dll", PluginHandler::format_name("test_plugin"));
    }

    #[test]
    #[cfg(target_os="macos")]
    fn test_format_name() {
        assert_eq!("libtest.dylib", PluginHandler::format_name("test"));
    }

    #[test]
    #[cfg(any(target_os="linux",
              target_os="freebsd",
              target_os="dragonfly",
              target_os="bitrig",
              target_os="netbsd",
              target_os="openbsd"))]
    fn test_format_name() {
        assert_eq!("libtest_plugin.so",
                   PluginHandler::format_name("test_plugin"));
    }
}

extern crate libloading;
extern crate tempdir;

use notify::{RecommendedWatcher, Error, Watcher, Event};
use std::sync::mpsc::{channel, Receiver};
use libc::{c_char, c_void, c_uchar};
use std::path::{Path, PathBuf};
use std::ffi::CStr;
use std::rc::Rc;
use std::mem::transmute;
use std::fs;
use std::ptr;
use self::libloading::{Library, Symbol};
use self::libloading::Result as LibRes; 
use self::tempdir::TempDir;

#[repr(C)]
pub struct CBasePlugin {
    pub name: *const c_char,
}

///
/// This holds infomation of a plugin along with loaded library
/// which path it was loaded from
/// The original path is the path where the file was before
/// shadow copy
///
pub struct Plugin {
    pub lib: Rc<Library>,
    pub path: PathBuf,
    pub original_path: Option<PathBuf>,
    pub name: String,
    pub plugin_funcs: *mut CBasePlugin,
}

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<Plugin>,
}

pub struct PluginHandler<'a> {
    pub view_plugins: Vec<Rc<Plugin>>,
    pub backend_plugins: Vec<Rc<Plugin>>,
    pub view_instances: Vec<ViewInstance>,
    pub search_paths: Vec<&'a str>,
    pub watcher: Option<RecommendedWatcher>,
    pub shadow_dir: Option<TempDir>,
    pub watch_recv: Receiver<Event>,
}


///
/// Holds temporary data when reloading instances 
///

struct ReloadData {
    pub name: String,
}

pub struct CallbackData<'a> {
    handler: &'a mut PluginHandler<'a>,
    lib: Rc<Library>,
    loaded_path: PathBuf,
    original_path: Option<PathBuf>,
}


#[repr(C)]
pub struct CViewPlugin {
    pub name: *const c_uchar,
    pub create_instance: Option<fn(ui_api: *const c_void, service: *const c_void) -> *mut c_void>,
    pub destroy_instance: Option<fn(*mut c_void)>,
    pub update: fn(ptr: *mut c_void,
                   ui: *const c_void,
                   reader: *const c_void,
                   writer: *const c_void),
    pub save_state: Option<fn(*mut c_void)>,
    pub load_state: Option<fn(*mut c_void)>,
}


type RegisterPlugin = unsafe fn(pt: *const c_char, plugin: *mut c_void, data: *mut CallbackData);

unsafe fn add_plugin(plugins: &mut Vec<Rc<Plugin>>,
                     plugin_type: *const c_char,
                     plugin: *mut c_void,
                     cb: &CallbackData,
                     type_name: &str) {
    for plugin in plugins.iter() {
        if cb.loaded_path == plugin.path {
            return;
        }
    }

    let ptype = CStr::from_ptr(plugin_type).to_string_lossy().into_owned();

    if !ptype.contains(type_name) {
        return;
    }

    let plugin_funcs: *mut CBasePlugin = transmute(plugin);

    let p = Rc::new(Plugin {
        name: CStr::from_ptr((*plugin_funcs).name).to_string_lossy().into_owned(),
        path: cb.loaded_path.clone(),
        original_path: cb.original_path.clone(),
        lib: cb.lib.clone(),
        plugin_funcs: plugin_funcs,
    });

    plugins.push(p);
}

unsafe fn register_plugin_callback(plugin_type: *const c_char,
                                   plugin: *mut c_void,
                                   ph: *mut CallbackData) {
    let t = &mut (*ph);
    add_plugin(&mut t.handler.view_plugins,
               plugin_type,
               plugin,
               &(*ph),
               "View");
    add_plugin(&mut t.handler.backend_plugins,
               plugin_type,
               plugin,
               &(*ph),
               "Backend");
}

impl<'a> PluginHandler<'a> {
    pub fn new(search_paths: Vec<&'a str>, shadow_dir: Option<&'static str>) -> PluginHandler<'a> {
        let (tx, rx) = channel();

        let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

        let watcher = match w {
            Ok(watcher) => Some(watcher),
            Err(_) => {
                println!("Unable to create file watcher, no dynamic reloading will be done");
                None
            }
        };

        // Create a temporary directory for shadow plugins

        let sd = match shadow_dir {
            Some(dir) => {
                match TempDir::new_in(dir, "shadow_plugins") {
                    Ok(td) => Some(td),
                    Err(er) => {
                        println!("Unable to create tempdir {}", er);
                        None
                    }
                }
            }
            None => None,
        };

        PluginHandler {
            backend_plugins: Vec::new(),
            view_plugins: Vec::new(),
            view_instances: Vec::new(),
            search_paths: search_paths,
            shadow_dir: sd,
            watch_recv: rx,
            watcher: watcher,
        }
    }

    ///
    /// Searches the paths given at the new function for the specified file
    /// and returns Option<PathBuf> if found otherwise None
    ///
    /// Example:
    ///
    /// ```ignore
    /// if let Some(file_path) = Self::fine_file(self, "file.txt") {
    ///     println!("found file {}", file_path.to_str().unwrap());
    /// } else {
    ///     println!("Unable to find plugin {}", clean_name);
    /// }
    /// ```
    fn fine_file(&self, name: &String) -> Option<PathBuf> {
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

    ///
    /// Loads a plugin for ProDBG. A plugin (currently) is a shared object file (dll/so/dylib)
    /// and looks for a "InitPlugin" entry point. The entry point looks like this
    /// 
    /// init_plugin(callback, private_data)
    ///
    /// As the API is in C here is how the callback function looks like
    /// 
    /// RegisterPlugin(const char* type, void* plugin, int pluginSize, void* private_data)
    ///
    /// Register plugin will be called from the plugin itself making it possible to register
    /// more than one plugin in one shared object
    ///
    /// Returns true if we managed to load the plugin and everything went ok
    ///
    unsafe fn load_plugin(&mut self, full_path: &PathBuf) -> bool {
        let path;
        let loaded_lib;
        let original_path;

        if let Some(sd) = self.shadow_dir.as_mut() {
            path = sd.path().join(full_path.file_name().unwrap());
            let _ = fs::copy(&full_path, &path);
            println!("Copy from {} {}", full_path.to_str().unwrap(), path.to_str().unwrap());
            original_path = Some(full_path.clone());
        } else {
            original_path = None;
            path = full_path.clone();
        }

        match Library::new(&path) {
            Ok(l) => loaded_lib = l,
            Err(e) => {
                println!("Unable to load {} error: {}", path.to_str().unwrap(), e);
                return false;
            }
        }

        let lib = Rc::new(loaded_lib);

        let init_plugin: LibRes<Symbol<extern "C" fn(RegisterPlugin, *mut CallbackData)>> 
            = lib.get(b"InitPlugin");

        match init_plugin {
            Ok(init_fun) => {

                if let Some(w) = self.watcher.as_mut() {
                    println!("Added watch on {}", full_path.to_str().unwrap());
                    let _ = w.watch(&full_path);
                }

                // Watch if someone changes the plugin

                let mut callback_data = CallbackData {
                    handler: transmute(self),
                    lib: lib.clone(),
                    loaded_path: path,
                    original_path: original_path,
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

    fn should_reload(reload_path: &PathBuf, plugin: &Plugin) -> bool {
        if let Some(p) = plugin.original_path.as_ref() {
            println!("{} - {}", reload_path.to_str().unwrap(), p.to_str().unwrap());
            if reload_path.to_str().unwrap().contains(p.to_str().unwrap()) {
                return true;
            }
        }

        false
    }


    fn should_reload_instance(reload_data: &mut Vec<ReloadData>, reload_path: &PathBuf, plugin: &Plugin) -> bool {
        if Self::should_reload(reload_path, plugin) {
            reload_data.push(ReloadData { name: plugin.name.clone() });
            return true;
        }

        false
    }

    pub fn reload_plugin(&mut self, reload_path: &PathBuf) {
        // TODO: Switch to SmallVec (https://github.com/arcnmx/stack-rs)
        let mut reload_data = Vec::new();

        //TODO: Implement save/reload for the removed plugins, layout, etc

        for i in (0..self.view_instances.len()).rev() {
            if Self::should_reload_instance(&mut reload_data, reload_path, &self.view_instances[i].plugin_type) {
                self.view_instances.swap_remove(i);
            }
        }

        // Unload the plugins

        for i in (0..self.view_plugins.len()).rev() {
            if Self::should_reload(reload_path, &self.view_plugins[i]) { 
                self.view_plugins.swap_remove(i);
            }
        }

        // Load plugin again

        for i in reload_data {
            unsafe { 
                Self::load_plugin(self, reload_path);
            }

            Self::create_view_instance(self, &i.name);
        }

        //println!("Number of instacnes to reload {}", reload_data.len());
    }

    pub fn create_view_instance(&mut self, plugin_type: &String) {
        for t in self.view_plugins.iter() {
            if t.name != *plugin_type {
                continue;
            }

            println!("Trying to create instance...");

            let user_data = unsafe {
                let callbacks = t.plugin_funcs as *mut CViewPlugin;
                (*callbacks).create_instance.unwrap()(ptr::null(), ptr::null())
            };

            let instance = ViewInstance {
                user_data: user_data,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                plugin_type: t.clone(),
            };

            self.view_instances.push(instance);

            return;
        }
    }

    /// Tries to add a plugin to the plugin manager.
    /// 
    /// The code will search the search paths and tries to find the file
    /// in the search_paths given to the PluginHandler::new call
    ///
    /// Returns true if everything went ok, otherwise fals
    /// 
    pub fn add_plugin(&mut self, clean_name: &str) -> bool {
        let name = Self::format_name(clean_name);

        if let Some(plugin_path) = Self::fine_file(self, &name) {
            unsafe { Self::load_plugin(self, &plugin_path) }
        } else {
            println!("Unable to find plugin {}", clean_name);
            false
        }
    }

    /// Formats dll name on Windows ("test_foo" -> "test_foo.dll")
    #[cfg(target_os="windows")]
    fn format_name(name: &str) -> String {
        format!("{}.dll", name)
    }

    /// Formats dll name on Mac ("test_foo" -> "libtest_foo.dylib")
    #[cfg(target_os="macos")]
    fn format_name(name: &str) -> String {
        format!("lib{}.dylib", name)
    }

    /// Formats dll name on *nix ("test_foo" -> "libtest_foo.so")
    #[cfg(any(target_os="linux",
              target_os="freebsd",
              target_os="dragonfly",
              target_os="netbsd",
              target_os="openbsd"))]
        fn format_name(name: &str) -> String {
            format!("lib{}.so", name)
        }

    pub fn add_non_standard(_: &str) {}
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serach_paths_find() {
        // This actually doesn't search for a plugin file but that doesn't really matter
        let search_paths = vec!["src", "other_path"];
        let plugin_handler = PluginHandler::new(search_paths);
        assert_eq!(plugin_handler.fine_file(&"main.rs".to_string()).is_some(),
        true);
    }

    #[test]
    fn test_serach_paths_no_find() {
        // This actually doesn't search for a plugin file but that doesn't really matter
        let search_paths = vec!["src", "other_path"];
        let plugin_handler = PluginHandler::new(search_paths);
        assert_eq!(plugin_handler.fine_file(&"main_no_find.rs".to_string()).is_none(),
        true);
    }

    #[test]
    fn test_load_plugin_init() {
        let search_paths = vec!["t2-output/macosx-clang-debug-default"];
        let mut plugin_handler = PluginHandler::new(search_paths);
        assert_eq!(plugin_handler.view_plugins.len(), 0);
        plugin_handler.add_plugin(&"breakpoints_plugin".to_string());
        assert_eq!(plugin_handler.view_plugins.len(), 1);
        plugin_handler.add_plugin(&"breakpoints_plugin".to_string());
        assert_eq!(plugin_handler.view_plugins.len(), 1);
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

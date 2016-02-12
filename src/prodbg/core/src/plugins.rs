extern crate libloading;
extern crate dynamic_reload;
extern crate libc;

use self::dynamic_reload::{DynamicReload, Lib, PlatformName, UpdateState};
use self::libloading::Result as LibRes; 
use self::libloading::Symbol;
use std::rc::Rc;
use std::ffi::CStr;
use self::libc::{c_char, c_void, c_uchar};
use std::mem::transmute;
use std::ptr;

pub struct Plugin {
    pub lib: Rc<Lib>,
    pub name: String,
    pub plugin_funcs: *mut CBasePlugin,
}

#[repr(C)]
pub struct CBasePlugin {
    name: *const c_char,
}

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<Plugin>,
}

pub struct Plugins {
    pub plugins: Vec<Rc<Plugin>>, 
    pub view_instances: Vec<ViewInstance>,
}

struct CallbackData<'a> {
    handler: &'a mut Plugins,
    lib: &'a Rc<Lib>,
}

// move
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

unsafe fn register_plugin_callback(_plugin_type: *const c_char,
                                   plugin: *mut c_void,
                                   ph: *mut CallbackData) {
    let t = &mut (*ph);

    let plugin_funcs: *mut CBasePlugin = transmute(plugin);

    let plugin = Plugin {
        lib: t.lib.clone(),
        name: CStr::from_ptr((*plugin_funcs).name).to_string_lossy().into_owned(),
        plugin_funcs: plugin_funcs,
    };

    t.handler.plugins.push(Rc::new(plugin));
}

pub struct ReloadHandler<'a> {
    pub plugins: &'a mut Plugins,
    pub instance_count: i32,
    pub name: String,
}

impl<'a> ReloadHandler<'a> {
    fn new(plugins: &'a mut Plugins) -> ReloadHandler {
        ReloadHandler {
            plugins: plugins,
            instance_count: 0, 
            name: "".to_string(),
        }
    }

    fn check_equal_view(&self, index: usize, lib: &Rc<Lib>) -> bool {
        self.plugins.view_instances[index].plugin_type.lib.original_path == lib.original_path
    }

    fn check_equal_plugins(&self, index: usize, lib: &Rc<Lib>) -> bool {
        self.plugins.plugins[index].lib.original_path == lib.original_path
    }

    fn unload_plugins(&mut self, lib: &Rc<Lib>) {

        for i in (0..self.plugins.view_instances.len()).rev() {
            if Self::check_equal_view(self, i, lib) {
                self.plugins.view_instances.swap_remove(i);
                self.instance_count += 1;
            }
        }

        // Unload the plugins

        for i in (0..self.plugins.plugins.len()).rev() {
            if Self::check_equal_plugins(self, i, lib) { 
                self.name = self.plugins.plugins[i].name.clone();
                self.plugins.plugins.swap_remove(i);
            }
        }
    }

    fn reload_plugins(&mut self, lib: &Rc<Lib>) {
        println!("About to reload plugins... {:?}", lib.original_path);

        unsafe {
            self.plugins.add_p(lib) 
        }

        for _ in 0..self.instance_count {
            self.plugins.create_view_instance(&self.name);
        }

        self.instance_count = 0;
    }

    fn callback(&mut self, state: UpdateState, lib: Option<&Rc<Lib>>) {
        match state {
            UpdateState::Before => {
                Self::unload_plugins(self, lib.unwrap())
            }

            UpdateState::After => {
                Self::reload_plugins(self, lib.unwrap())
            }

            UpdateState::ReloadFalied => {
                println!("Failed to reload {}", self.name);
                self.instance_count = 0;
            }
        }
    }
}

impl Plugins {
    pub fn new() -> Plugins {
        Plugins {
            plugins: Vec::new(),
            view_instances: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, lib_handler: &mut DynamicReload, name: &str) {
        match lib_handler.add_library(name, PlatformName::Yes) {
            Ok(lib) => unsafe {
                Self::add_p(self, &lib);
            },
            Err(e) => {
                println!("Unable to add {} err {:?}", name, e);
            }
        }
    }

    pub fn update(&mut self, lib_handler: &mut DynamicReload) {
        let mut handler = ReloadHandler::new(self);
        lib_handler.update(ReloadHandler::callback, &mut handler);
    }

    unsafe fn add_p(&mut self, library: &Rc<Lib>) {
        let init_plugin: LibRes<Symbol<extern "C" fn(RegisterPlugin, *mut CallbackData)>> = 
                library.lib.get(b"InitPlugin"); 

        match init_plugin {
            Ok(init_fun) => {
                // Watch if someone changes the plugin
                let mut callback_data = CallbackData {
                    handler: transmute(self),
                    lib: library,
                };

                init_fun(register_plugin_callback, &mut callback_data);
            }

            _ => (),
        }
    }

    pub fn create_view_instance(&mut self, plugin_type: &String) {
        for t in self.plugins.iter() {
            if t.name != *plugin_type {
                continue;
            }

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
}

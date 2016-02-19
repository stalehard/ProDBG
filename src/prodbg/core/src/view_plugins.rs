use prodbg_api::view::CViewCallbacks;
use libc::{c_void, c_uchar};
use std::rc::Rc;
use standard_plugin::StandardPlugin;
use dynamic_reload::Lib;
use std::ptr;

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<StandardPlugin>,
}

pub struct ViewPlugins {
    pub instances: Vec<ViewInstance>,
    plugin_types: Vec<Rc<StandardPlugin>>,
    // temporary stored for reloading
    reload_name: String,
    reload_count: i32, 
}

impl ViewPlugins {
    pub fn new() -> ViewPlugins {
        ViewPlugins {
            instances: Vec::new(),
            plugin_types: Vec::new(),
            reload_name: "".to_owned(),
            reload_count: 0,
        }
    }

    pub fn unload_plugin(&mut self, lib: &Rc<Lib>) {
        self.reload_count = 0;
        for i in (0..self.instances.len()).rev() {
            if Self::check_equal_view(self, i, lib) {
                self.instances.swap_remove(i);
                self.reload_count += 1;
            }
        }

        // Unload the plugins

        for i in (0..self.plugin_types.len()).rev() {
            if Self::check_equal_plugins(self, i, lib) { 
                self.reload_name = self.plugin_types[i].name.clone();
                self.plugin_types.swap_remove(i);
            }
        }
    }

    fn check_equal_view(&self, index: usize, lib: &Rc<Lib>) -> bool {
        self.instances[index].plugin_type.lib.original_path == lib.original_path
    }

    fn check_equal_plugins(&self, index: usize, lib: &Rc<Lib>) -> bool {
        self.plugin_types[index].lib.original_path == lib.original_path
    }
    
    pub fn reload_plugin(&mut self) {
        let name = self.reload_name.clone();
        for _ in 0..self.reload_count {
            Self::create_instance(self, &name);
        }

        Self::reset_reload(self)
    }

    pub fn is_view_plugin(plugin: &StandardPlugin) -> bool {
        // TODO: Handle versions
        plugin.type_name.contains("View")
    }

    pub fn add_plugin(&mut self, plugin: &Rc<StandardPlugin>) {
        self.plugin_types.push(plugin.clone())
    }

    pub fn reload_failed(&mut self) {
        Self::reset_reload(self);
    }

    pub extern "C" fn service_fun(_name: *const c_uchar) -> *mut c_void {
        ptr::null_mut()
    }

    pub fn create_instance(&mut self, plugin_type: &String) {
        for t in self.plugin_types.iter() {
            if t.name != *plugin_type {
                continue;
            }

            let user_data = unsafe {
                let callbacks = t.plugin_funcs as *mut CViewCallbacks;
                (*callbacks).create_instance.unwrap()(ptr::null(), Self::service_fun)
            };

            let instance = ViewInstance {
                user_data: user_data,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                plugin_type: t.clone(),
            };

            self.instances.push(instance);

            return;
        }
    }

    fn reset_reload(&mut self) {
        self.reload_count = 0;
        self.reload_name = "".to_owned();
    }

}

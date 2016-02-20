use prodbg_api::view::CViewCallbacks;
use libc::{c_void, c_uchar};
use std::rc::Rc;
use plugin::Plugin;
use plugins::PluginHandler;
use dynamic_reload::Lib;
use std::ptr;

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<Plugin>,
}

pub struct ViewPlugins {
    pub instances: Vec<ViewInstance>,
    plugin_types: Vec<Rc<Plugin>>,
    // temporary stored for reloading
    reload_name: String,
    reload_count: i32, 
}

impl PluginHandler for ViewPlugins {
    fn is_correct_plugin_type(&self, plugin: &Plugin) -> bool {
        plugin.type_name.contains("View")
    }

    fn add_plugin(&mut self, plugin: &Rc<Plugin>) {
        self.plugin_types.push(plugin.clone())
    }

    fn unload_plugin(&mut self, lib: &Rc<Lib>) {
        self.reload_count = 0;
        for i in (0..self.instances.len()).rev() {
            if &self.instances[i].plugin_type.lib == lib {
                self.instances.swap_remove(i);
                self.reload_count += 1;
            }
        }

        for i in (0..self.plugin_types.len()).rev() {
            if &self.plugin_types[i].lib == lib {
                self.reload_name = self.plugin_types[i].name.clone();
                self.plugin_types.swap_remove(i);
            }
        }
    }

    fn reload_plugin(&mut self) {
        let name = self.reload_name.clone();
        for _ in 0..self.reload_count {
            Self::create_instance(self, &name);
        }

        Self::reset_reload(self)
    }

    fn reload_failed(&mut self) {
        Self::reset_reload(self)
    }
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

use prodbg_api::view::CViewCallbacks;
use libc::{c_void, c_uchar};
use std::rc::Rc;
use plugin::Plugin;
use plugins::PluginHandler;
use dynamic_reload::Lib;
use std::ptr;

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub session_id: usize,
    pub window_id: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<Plugin>,
}

#[derive(Clone)]
struct ReloadState {
    name: String,
    session_id: usize,
    window_id: usize,
}

pub struct ViewPlugins {
    pub instances: Vec<ViewInstance>,
    plugin_types: Vec<Rc<Plugin>>,
    reload_state: Vec<ReloadState>,
}

impl PluginHandler for ViewPlugins {
    fn is_correct_plugin_type(&self, plugin: &Plugin) -> bool {
        plugin.type_name.contains("View")
    }

    fn add_plugin(&mut self, plugin: &Rc<Plugin>) {
        self.plugin_types.push(plugin.clone())
    }

    fn unload_plugin(&mut self, lib: &Rc<Lib>) {
        self.reload_state.clear();
        for i in (0..self.instances.len()).rev() {
            if &self.instances[i].plugin_type.lib == lib {
                let state = ReloadState {
                    name: self.instances[i].plugin_type.name.clone(),
                    session_id: self.instances[i].session_id,
                    window_id: self.instances[i].window_id,
                };

                self.reload_state.push(state);
                self.instances.swap_remove(i);
            }
        }

        for i in (0..self.plugin_types.len()).rev() {
            if &self.plugin_types[i].lib == lib {
                self.plugin_types.swap_remove(i);
            }
        }
    }

    fn reload_plugin(&mut self) {
        let t = self.reload_state.clone();
        for reload_plugin in &t {
            Self::create_instance(self,
                                  &reload_plugin.name,
                                  reload_plugin.session_id,
                                  reload_plugin.window_id);
        }
    }

    fn reload_failed(&mut self) {}
}


impl ViewPlugins {
    pub fn new() -> ViewPlugins {
        ViewPlugins {
            instances: Vec::new(),
            plugin_types: Vec::new(),
            reload_state: Vec::new(),
        }
    }

    pub extern "C" fn service_fun(_name: *const c_uchar) -> *mut c_void {
        ptr::null_mut()
    }

    pub fn create_instance(&mut self, plugin_type: &String, session_id: usize, window_id: usize) {
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
                session_id: session_id,
                window_id: window_id,
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
}

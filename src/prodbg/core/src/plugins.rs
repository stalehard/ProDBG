extern crate libloading;
extern crate dynamic_reload;
extern crate libc;

use self::dynamic_reload::{DynamicReload, Lib, PlatformName, UpdateState};
use self::libloading::Result as LibRes;
use self::libloading::Symbol;
use std::rc::Rc;
use self::libc::{c_char, c_void};
use std::mem::transmute;
use standard_plugin::StandardPlugin;
use view_plugins::ViewPlugins;

pub struct Plugins {
    pub plugin_types: Vec<Rc<Lib>>,
    pub view_plugins: ViewPlugins,
}

struct CallbackData<'a> {
    handler: &'a mut Plugins,
    lib: &'a Rc<Lib>,
}

pub struct ReloadHandler<'a> {
    pub plugins: &'a mut Plugins,
    pub instance_count: i32,
    pub name: String,
}

type RegisterPlugin = unsafe fn(pt: *const c_char, plugin: *mut c_void, data: *mut CallbackData);

unsafe fn register_plugin_callback(plugin_type: *const c_char,
                                   plugin: *mut c_void,
                                   ph: *mut CallbackData) {
    let t = &mut (*ph);

    t.handler.plugin_types.push(t.lib.clone());

    let standard_plugin = StandardPlugin::new(t.lib, plugin_type, plugin);

    if ViewPlugins::is_view_plugin(&standard_plugin) {
        t.handler.view_plugins.add_plugin(&Rc::new(standard_plugin));
    }
}

impl<'a> ReloadHandler<'a> {
    fn new(plugins: &'a mut Plugins) -> ReloadHandler {
        ReloadHandler {
            plugins: plugins,
            instance_count: 0,
            name: "".to_string(),
        }
    }

    fn check_equal_plugins(&self, index: usize, lib: &Rc<Lib>) -> bool {
        self.plugins.plugin_types[index].original_path == lib.original_path
    }

    fn unload_plugins(&mut self, lib: &Rc<Lib>) {
        self.plugins.view_plugins.unload_plugin(lib);

        for i in (0..self.plugins.plugin_types.len()).rev() {
            if Self::check_equal_plugins(self, i, lib) {
                self.plugins.plugin_types.swap_remove(i);
            }
        }
    }

    fn reload_plugins(&mut self, lib: &Rc<Lib>) {
        unsafe { self.plugins.add_p(lib) }
        self.plugins.view_plugins.reload_plugin()
    }

    fn reload_failed(&mut self) {
        self.plugins.view_plugins.reload_failed();
    }

    fn callback(&mut self, state: UpdateState, lib: Option<&Rc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugins(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugins(self, lib.unwrap()),
            UpdateState::ReloadFalied => Self::reload_failed(self),
        }
    }
}

impl Plugins {
    pub fn new() -> Plugins {
        Plugins {
            plugin_types: Vec::new(),
            view_plugins: ViewPlugins::new(),
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
                let mut callback_data = CallbackData {
                    handler: transmute(self),
                    lib: library,
                };

                init_fun(register_plugin_callback, &mut callback_data);
            }

            _ => (),
        }
    }
}

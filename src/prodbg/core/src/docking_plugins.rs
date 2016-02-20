use libc::{c_void, c_uchar};
use std::rc::Rc;
use standard_plugin::StandardPlugin;
use dynamic_reload::Lib;
use std::ptr;

/*

pub struct DockingPlugin {
    pub plugin: Option<Rc<StandardPlugin>>,
    pub should_reload: bool,
}

impl DockingPlugin {

    pub fn is_docking_plugin(plugin: &StandardPlugin) -> bool {
        plugin.type_name.contains("Docking")
    }

    pub fn set_plugin(&mut self, plugin: &StandardPlugin)  {
        self.plugin = Some(plugin.clone());
    }

    pub fn unload_plugin(&mut self, lib: &Rc<Lib>) {
        if let Some(plugin) = self.plugin {
            if &plugin.lib == lib {
                self.should_reload = true
            }
        }
    }

    Trait

}

*/



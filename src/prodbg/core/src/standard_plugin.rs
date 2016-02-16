///!
///! Stdandard plugin is of the type of view plugins and backend plugins
///! which follows the same structure in the shared libs
///!

use self::dynamic_reload::{DynamicReload, Lib, PlatformName, UpdateState};

#[repr(C)]
pub struct CBasePlugin {
    pub name: *const c_char,
}

struct StandardPlugin {
    pub lib: Rc<Lib>,
    pub name: String,
    pub type_name: String,
    pub plugin_funcs: *mut CBasePlugin,
}

impl StandardPlugin {
    pub fn new(plugin_type: *const c_char, plugin: *mut c_void) -> StandardPlugin {
        let plugin_funcs: *mut CBasePlugin = transmute(plugin);

        StandardPlugin {
            lib: t.lib.clone(),
            type_name: CStr::from_ptr(plugin_type).to_string_lossy().into_owned(),
            name: CStr::from_ptr((*plugin_funcs).name).to_string_lossy().into_owned(),
            plugin_funcs: plugin_funcs,
        }
    }
}

#[cfg(test)]
mod tests {
}

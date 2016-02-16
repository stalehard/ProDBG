use self::api::CViewCallbacks;

pub struct ViewInstance {
    pub user_data: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub plugin_type: Rc<StandardPlugin>,
}

struct ViewPlugins {
    pub view_instances: Vec<ViewInstance>,
    plugin_types: Vec<Rc<StandardPlugin>>,
    // temporary stored for reloading
    reload_name: String,
    reload_count: i32, 
}

impl ViewPlugins {
    pub fn unload_plugin(&mut self, lib: &Rc<Lib>) {
        self.reload_count = 0;
        for i in (0..self.view_instances.len()).rev() {
            if Self::check_equal_view(self, i, lib) {
                self.name = self.view_instances[i].plugin_type.name.clone();
                self.view_instances.swap_remove(i);
                self.instance_count += 1;
            }
        }
    }
    
    pub fn reload_plugin(&mut self, lib: &Rc<Lib>) {
        for _ in 0..self.instance_count {
            self.plugins.create_view_instance(&self.name);
        }

        Self::reset_reload()
    }

    pub fn is_view_plugin(plugin: &StandardPlugin) {
        // TODO: Handle versions
        plugin.type_name.contains("View")
    }

    pub fn add_plugin(&mut self, plugin: &StandardPlugin) {
        self.plugin_types.push(Rc::new(plugin));
    }

    pub fn reload_failed() {
        Self::reset_reload();
    }

    pub fn create_instance(&mut self, plugin_type: &String) {
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

    fn reset_reload(&mut self) {
        self.instance_count = 0;
        self.name = 0;
    }

}

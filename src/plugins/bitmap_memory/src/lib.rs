#[macro_use]
extern crate prodbg;

use prodbg::*;

struct BitmapView {
    dummy: i32,
}

impl View for BitmapView {
    fn new(_: &Ui, _: &Service) -> Self {
        BitmapView { dummy: 0 }
    }

    fn update(&mut self, ui: &Ui, _: &mut Reader, _: &mut Writer) {
        if ui.button("test") {
            println!("yah");
        }
        self.dummy += 1;
    }
}

#[no_mangle]
#[allow(unused_mut)] // likely due to compiler bug
pub fn init_plugin(plugin_handler: &mut PluginHandler) {
    println!("R: init_plugin");
    let mut plugin = define_view_plugin!(BitmapView);
    plugin_handler.register_view(VIEW_API_VERSION, &mut plugin);
}

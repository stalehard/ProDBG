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
        if ui.button("test", None) {
            println!("yah");
        }
        self.dummy += 1;
    }
}

#[no_mangle]
pub fn init_plugin(plugin_handler: &mut PluginHandler) {
    let plugin = define_view_plugin!("Bitmap View", BitmapView);
    plugin_handler.register_view(&plugin);
}

#[macro_use]
extern crate prodbg_api;

use prodbg_api::*;

struct DisassemblyView {
    location: u64,
    address_size: u8,
    reset_to_center: bool,
}

impl View for DisassemblyView {
    fn new(_: &Ui, _: &Service) -> Self {
        DisassemblyView {
            location: 0,
            address_size: 4,
            reset_to_center: true,
        }
    }

    fn update(&mut self, ui: &Ui, reader: &mut Reader, _: &mut Writer) {
        let mut request_dissasembly = false;

        for event in reader.get_event() {
            /*
            match event as EventType {
                EventType::SetExceptionLocation => {
                    let location = reader.find_u64("address").ok().unwrap();
                    
                    if self.location != location {
                        self.location = location;
                        request_dissasembly = true;
                    }

                    self.address_size = reader.find_u8("address_size").ok().unwrap();
                }

                EventType::SetExceptionLocation => {
                }

                _ => (),
            }
        */
        }
    }
}

#[no_mangle]
pub fn init_plugin(plugin_handler: &mut PluginHandler) {
    define_view_plugin!(PLUGIN, b"Disassembly2 View", DisassemblyView);
    plugin_handler.register_view(&PLUGIN);
}

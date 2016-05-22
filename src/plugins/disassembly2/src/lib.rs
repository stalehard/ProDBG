#[macro_use]
extern crate prodbg_api;

use prodbg_api::*;

struct Line {
    opcode: String,
    address: u64,
    _breakpoint: bool
}

struct DisassemblyView {
    location: u64,
    address_size: u8,
    _reset_to_center: bool,
    lines: Vec<Line>,
}

impl DisassemblyView {
    fn set_disassembly(&mut self, reader: &mut Reader) {
        for entry in reader.find_array("disassembly") {
            let addr = entry.find_u64("address").ok().unwrap();
            let line = entry.find_string("line").ok().unwrap();
            self.lines.push(Line {
                address: addr ,
                _breakpoint: false,
                opcode: line.to_owned(),
            });
        }
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        for line in &self.lines {
            ui.text_fmt(format_args!("0x{:x} {}", line.address, line.opcode));
        }
    }
}

impl View for DisassemblyView {
    fn new(_: &Ui, _: &Service) -> Self {
        DisassemblyView {
            location: 0,
            address_size: 4,
            lines: Vec::new(),
            _reset_to_center: true,
        }
    }

    fn update(&mut self, ui: &mut Ui, reader: &mut Reader, writer: &mut Writer) {
        let mut request_dissasembly = false;

        for event in reader.get_events() {
            match event {
                EVENT_SET_EXCEPTION_LOCATION => {
                    let location = reader.find_u64("address").ok().unwrap();

                    if self.location != location {
                        self.location = location;
                        request_dissasembly = true;
                    }

                    self.address_size = reader.find_u8("address_size").ok().unwrap();
                }

                EVENT_SET_DISASSEMBLY => {
                    self.set_disassembly(reader);
                }

                _ => (),
            }
        }

        if request_dissasembly {
            // some temp request right now
            writer.event_begin(EVENT_GET_DISASSEMBLY as u16);
            writer.write_u64("address_start", self.location);
            writer.write_u32("instruction_count", 30);
            writer.event_end();
        }

        self.render_ui(ui);
    }
}

#[no_mangle]
pub fn init_plugin(plugin_handler: &mut PluginHandler) {
    define_view_plugin!(PLUGIN, b"Disassembly2 View", DisassemblyView);
    plugin_handler.register_view(&PLUGIN);
}

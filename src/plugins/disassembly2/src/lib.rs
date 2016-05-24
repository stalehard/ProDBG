#[macro_use]
extern crate prodbg_api;

use prodbg_api::*;

struct Line {
    opcode: String,
    address: u64,
    _breakpoint: bool
}

struct AddressRange {
    start: u64,
    end: u64,
}

impl AddressRange {
    fn new() -> Self {
        AddressRange {
            start: u64::max_value(),
            end: 0,
        }
    }

    fn update_range(&mut self, v: u64) {
        if v < self.start { self.start = v; }   
        if v > self.end { self.end = v; }   
    }
}

struct DisassemblyView {
    location: u64,
    address_size: u8,
    //cursor: i32,
    _reset_to_center: bool,
    lines_range: AddressRange,
    visible_range: AddressRange,
    lines: Vec<Line>,
}

impl DisassemblyView {
    fn set_disassembly(&mut self, reader: &mut Reader) {
        self.lines_range = AddressRange::new();

        for entry in reader.find_array("disassembly") {
            let address = entry.find_u64("address").ok().unwrap();
            let line = entry.find_string("line").ok().unwrap();

            self.lines_range.update_range(address);

            self.lines.push(Line {
                address: address,
                _breakpoint: false,
                opcode: line.to_owned(),
            });
        }
    }

    fn get_visible_lines_count(ui: &Ui) -> usize {
        let (_, height) = ui.get_window_size();
        let text_height = ui.get_text_line_height_with_spacing();
        // - 1.0 for title text. Would be better to get the cursor pos here instead
        let visible_lines = (height / text_height) - 1.0;
        // + 0.5 to round up
        (visible_lines + 0.5) as usize 
    }

    /*
    fn check_request_disassembly(&mut self, ui: &mut Ui) -> bool {
        let Self::get_visible_lines_count(ui);
    }
    */

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
            lines_range: AddressRange::new(),
            visible_range: AddressRange::new(),
            lines: Vec::new(),
            _reset_to_center: true,
        }
    }

    fn update(&mut self, ui: &mut Ui, reader: &mut Reader, writer: &mut Writer) {
        let mut request_dissasembly = false;

        let (_, height) = ui.get_window_size();
        let text_height = ui.get_text_line_height_with_spacing();
        // - 1.0 for title text. Would be better to get the cursor pos here instead
        let visible_lines = (height / text_height) - 1.0;

        for event in reader.get_events() {
            match event {
                EVENT_SET_EXCEPTION_LOCATION => {
                    let location = reader.find_u64("address").ok().unwrap();
                    self.address_size = reader.find_u8("address_size").ok().unwrap();

                    if self.location != location {
                        self.location = location;
                        request_dissasembly = true;
                    }
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

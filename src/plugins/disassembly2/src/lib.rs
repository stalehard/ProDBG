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

    fn is_with_range(&self, v: u64) -> bool {
        v >= self.start && v < self.end
    }
}

struct DisassemblyView {
    location: u64,
    address_size: u8,
    _cursor: i32,
    reset_to_center: bool,
    lines_range: AddressRange,
    visible_range: AddressRange,
    lines: Vec<Line>,
}

impl DisassemblyView {
    fn set_disassembly(&mut self, reader: &mut Reader) {
        self.lines_range = AddressRange::new();
        self.lines.clear();

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

    fn request_disassembly(&mut self, ui: &mut Ui, location: u64, writer: &mut Writer) {
        let visible_lines = Self::get_visible_lines_count(ui) as u64;

        // if not within range we need to request new disassembly
        // We want to place the cursor in the middle of the screer so we
        // need to request about half the visible range instructions around
        // PC so we can have it the middle. To make sure we get enough instructions
        // we go back some extra in memory and request twice the amouth
        if !self.visible_range.is_with_range(location) {
            self.reset_to_center = true;
            writer.event_begin(EVENT_GET_DISASSEMBLY as u16);
            writer.write_u64("address_start", location - (4 as u64 * visible_lines / 2));
            writer.write_u32("instruction_count", (visible_lines * 5) as u32);
            writer.event_end();
        }

        self.location = location;
    }

    fn update_current_line(&mut self, ui: &mut Ui) {
        let visible_lines = (Self::get_visible_lines_count(ui) / 2) as u64;

        if self.reset_to_center {
            self.visible_range = AddressRange::new();

            let len = self.lines.len(); 

            for i in 0..len {
                if self.lines[i].address == self.location {
                    println!("range location {:x} - {} {} {} ", self.location, i, len, visible_lines);
                    self.visible_range.start = i as u64 - visible_lines;
                    self.visible_range.end = i as u64 + visible_lines;
                    println!("visible range is {:x} - {:x}", 
                            self.visible_range.start, 
                            self.visible_range.end);
                    self.reset_to_center = false;
                    return;
                }
            }
        }
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        if self.lines.len() == 0 {
            return;
        }

        let start = self.visible_range.start as usize;
        let end = self.visible_range.end as usize;
        for line in &self.lines[start..end] {
            ui.text_fmt(format_args!("0x{:x} {}", line.address, line.opcode));
        }
    }
}

impl View for DisassemblyView {
    fn new(_: &Ui, _: &Service) -> Self {
        DisassemblyView {
            location: u64::max_value(),
            address_size: 4,
            lines_range: AddressRange::new(),
            visible_range: AddressRange::new(),
            _cursor: 0,
            lines: Vec::new(),
            reset_to_center: false,
        }
    }

    fn update(&mut self, ui: &mut Ui, reader: &mut Reader, writer: &mut Writer) {
        for event in reader.get_events() {
            match event {
                EVENT_SET_EXCEPTION_LOCATION => {
                    let location = reader.find_u64("address").ok().unwrap();
                    self.address_size = reader.find_u8("address_size").ok().unwrap();

                    if self.location != location {
                        self.request_disassembly(ui, location, writer);
                    }
                }

                EVENT_SET_DISASSEMBLY => {
                    self.set_disassembly(reader);
                    self.update_current_line(ui);
                }

                _ => (),
            }
        }

        self.render_ui(ui);
    }
}

#[no_mangle]
pub fn init_plugin(plugin_handler: &mut PluginHandler) {
    define_view_plugin!(PLUGIN, b"Disassembly2 View", DisassemblyView);
    plugin_handler.register_view(&PLUGIN);
}

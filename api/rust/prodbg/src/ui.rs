use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use ui_ffi::*;

pub struct Ui {
    pub api: *mut CPdUI,
}

const STRING_SIZE: usize = 512;

struct StringHandler {
    pub local: bool,
    pub local_string: [i8; STRING_SIZE],
    pub heap_string: Option<CString>,
}

impl StringHandler {
    pub fn new(name: &str) -> StringHandler {
        unsafe {
            let name_len = name.len();
            if name_len <= STRING_SIZE - 1 {
                let mut handler = StringHandler {
                    local: true,
                    local_string: mem::uninitialized(),
                    heap_string: None,
                };

                ptr::copy(name.as_ptr(),
                          handler.local_string.as_mut_ptr() as *mut u8,
                          name_len);
                handler.local_string[name_len] = 0;
                handler
            } else {
                StringHandler {
                    local: false,
                    local_string: mem::uninitialized(),
                    heap_string: Some(CString::new(name).unwrap()),
                }
            }
        }
    }

    pub fn as_ptr(&mut self) -> *const i8 {
        if self.local {
            self.local_string.as_ptr()
        } else {
            self.heap_string.as_mut().unwrap().as_ptr()
        }
    }
}

impl Ui {
    pub fn set_title(&self, title: &str) {
        unsafe {
            let mut t = StringHandler::new(title);
            ((*self.api).set_title)((*self.api).private_data, t.as_ptr());
        }
    }

    #[inline]
    pub fn get_window_size(&self) -> PDVec2 {
        unsafe { ((*self.api).get_window_size)() }
    }

    #[inline]
    pub fn get_window_pos(&self) -> PDVec2 {
        unsafe { ((*self.api).get_window_pos)() }
    }

    pub fn begin_child(&self, id: &str, pos: Option<PDVec2>, border: bool, flags: u32) {
        unsafe {
            let t = StringHandler::new(id).as_ptr();
            match pos {
                Some(p) => ((*self.api).begin_child)(t, p, border as i32, flags as i32),
                None => {
                    ((*self.api).begin_child)(t,
                                              PDVec2 { x: 0.0, y: 0.0 },
                                              border as i32,
                                              flags as i32)
                }
            }
        }
    }

    #[inline]
    pub fn end_child(&self) {
        unsafe { ((*self.api).end_child)() }
    }

    #[inline]
    pub fn get_scroll_y(&self) -> f32 {
        unsafe { ((*self.api).get_scroll_y)() as f32 }
    }

    #[inline]
    pub fn get_scroll_max_y(&self) -> f32 {
        unsafe { ((*self.api).get_scroll_max_y)() as f32 }
    }

    #[inline]
    pub fn set_scroll_y(&self, pos: f32) {
        unsafe { ((*self.api).set_scroll_y)(pos) }
    }

    #[inline]
    pub fn set_scroll_here(&self, center_ratio: f32) {
        unsafe { ((*self.api).set_scroll_here)(center_ratio) }
    }

    #[inline]
    pub fn set_scroll_from_pos_y(&self, pos_y: f32, center_ratio: f32) {
        unsafe { ((*self.api).set_scroll_from_pos_y)(pos_y, center_ratio) }
    }

    #[inline]
    pub fn set_keyboard_focus_here(&self, offset: i32) {
        unsafe { ((*self.api).set_keyboard_focus_here)(offset) }
    }
    
    // TODO: push/pop font

    #[inline]
	pub fn push_style_color(&self, index: usize, color: u32) {
        unsafe { ((*self.api).push_style_color)(index as u32, color) }
    }

    #[inline]
	pub fn pop_style_color(&self, index: usize) {
        unsafe { ((*self.api).pop_style_color)(index as i32) }
    }

    #[inline]
	pub fn push_style_var(&self, index: usize, val: f32) {
        unsafe { ((*self.api).push_style_var)(index as u32, val) }
    }

    #[inline]
	pub fn push_style_var_vec(&self, index: usize, val: PDVec2) {
        unsafe { ((*self.api).push_style_var_vec)(index as u32, val) }
    }

    // Text

    pub fn text(&self, text: &str) {
        unsafe {
            let t = StringHandler::new(text).as_ptr();
            ((*self.api).text)(t);
        }
    }

    pub fn text_colored(&self, color: u32, text: &str) {
        unsafe {
            let t = StringHandler::new(text).as_ptr();
            ((*self.api).text_colored)(color, t);
        }
    }

    pub fn text_disabled(&self, text: &str) {
        unsafe {
            let t = StringHandler::new(text).as_ptr();
            ((*self.api).text_disabled)(t);
        }
    }

    pub fn text_wrapped(&self, text: &str) {
        unsafe {
            let t = StringHandler::new(text).as_ptr();
            ((*self.api).text_wrapped)(t);
        }
    }

    pub fn button(&self, title: &str, pos: Option<PDVec2>) -> bool {
        unsafe {
            let mut t = StringHandler::new(title);
            match pos {
                Some(p) => ((*self.api).button)(t.as_ptr(), p) != 0,
                None => ((*self.api).button)(t.as_ptr(), PDVec2 { x: 0.0, y: 0.0 }) != 0,
            }
        }
    }
}


// Only used in tests
#[allow(dead_code)]
fn get_string(handler: &mut StringHandler) -> String {
    unsafe { CStr::from_ptr(handler.as_ptr() as *const i8).to_string_lossy().into_owned() }
}

#[test]
fn test_string_handler() {
    {
        let short_string = "";
        let t = StringHandler::new(short_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), short_string);
    }
    {
        let short_string = "test_local";
        let t = StringHandler::new(short_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), short_string);
    }
    {
        let short_string = "test_local stoheusthsotheost";
        let t = StringHandler::new(short_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), short_string);
    }
    {
        // this string (with 511) buffer should just fit
        let test_511_string = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuueeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeaaaaaaaaaaaa\
                               aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
                               eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooooooooacd";
        let t = StringHandler::new(test_511_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), test_511_string);
    }
    {
        // this string (with 512) buffer should no fit
        let test_512_string = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuueeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeaaaaaaaaaaaa\
                               aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
                               eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooooooooabcd";
        let t = StringHandler::new(test_512_string);
        assert_eq!(t.local, false);
        assert_eq!(get_string(&t), test_512_string);
    }
    {
        // this string (with 513) buffer should no fit
        let test_513_string = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuueeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeaaaaaaaaaaaa\
                               aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
                               eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeuuuuuuuuuuuuuuuuuuuuuuuuuu\
                               uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuoooooooooooooooooooooooooooooo\
                               oooooooooooooooooooooooooooooooooooooooabcd";
        let t = StringHandler::new(test_513_string);
        assert_eq!(t.local, false);
        assert_eq!(get_string(&t), test_513_string);
    }
}

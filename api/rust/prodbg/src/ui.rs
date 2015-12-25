use std::ffi::CString;
use std::mem;
use std::ptr;
use ui_ffi::*;

pub struct Ui {
    pub api: *mut CPdUI,
}

const STRING_SIZE: usize = 1024;

struct StringHandler {
    pub local: bool,
    pub local_string: [u8; STRING_SIZE],
    pub heap_string: CString,
}

impl StringHandler {
    pub unsafe fn new(name: &str) -> StringHandler {
        let name_len = name.len();
        if name_len < STRING_SIZE - 1 {
            let mut handler = StringHandler {
                local: true,
                local_string: mem::uninitialized(),
                heap_string: mem::uninitialized(),
            };

            ptr::copy(name.as_ptr(), handler.local_string.as_mut_ptr(), name_len);
            handler.local_string[name_len] = 0;
            handler
        } else {
            StringHandler {
                local: false,
                local_string: mem::uninitialized(),
                heap_string: CString::new(name).unwrap(),
            }
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        if self.local {
            self.local_string.as_ptr()
        } else {
            self.heap_string.as_ptr() as *const u8
        }
    }
}

impl Ui {
    pub fn button(&self, title: &str) {
        unsafe {
            let t = StringHandler::new(title);
            let s = PDVec2 { x: 0.0, y: 0.0 };
            ((*self.api).button).unwrap()(t.as_ptr(), s);
        }
    }
}

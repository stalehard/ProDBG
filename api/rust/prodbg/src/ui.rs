use std::ffi::{CString};
use std::mem;
use std::ptr;
use ui_ffi::*;

pub struct Ui {
    pub api: *mut CPdUI,
}

const STRING_SIZE: usize = 512;

struct StringHandler {
    pub local: bool,
    pub local_string: [u8; STRING_SIZE],
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

                ptr::copy(name.as_ptr(), handler.local_string.as_mut_ptr(), name_len);
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

    /*
    pub fn as_ptr(&self) -> *const u8 {
        if self.local {
            self.local_string.as_ptr()
        } else {
            self.heap_string.unwrap().as_ptr() as *const u8
        }
    }
    */
}

impl Ui {
    pub fn button(&self, title: &str) -> bool {
        unsafe { 
            let t = StringHandler::new(title);
            let s = PDVec2 { x: 0.0, y: 0.0 };
            if t.local {
                ((*self.api).button)(t.local_string.as_ptr(), s) != 0
            } else {
                ((*self.api).button)(t.heap_string.unwrap().as_ptr() as *const u8, s) != 0
            }
        }
    }
}


// Only used in tests
#[allow(dead_code)]
/*
fn get_string(handler: &StringHandler) -> String {
    unsafe { CStr::from_ptr(handler.as_ptr() as *const i8).to_string_lossy().into_owned() }
}
*/

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

use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;

const STRING_SIZE: usize = 512;

pub struct CFixedString {
    pub local_string: [i8; STRING_SIZE],
    pub heap_string: Option<CString>,
}

impl CFixedString {
    pub fn from_str(name: &str) -> CFixedString {
        unsafe {
            let name_len = name.len();
            if name_len <= STRING_SIZE - 1 {
                let mut handler = CFixedString {
                    local_string: mem::uninitialized(),
                    heap_string: None,
                };

                ptr::copy(name.as_ptr(),
                          handler.local_string.as_mut_ptr() as *mut u8,
                          name_len);
                handler.local_string[name_len] = 0;

                handler
            } else {
                CFixedString {
                    local_string: mem::uninitialized(),
                    heap_string: Some(CString::new(name).unwrap()),
                }
            }
        }
    }

    pub fn as_ptr(&mut self) -> *const i8 {
        if self.heap_string == None {
            self.local_string.as_ptr()
        } else {
            self.heap_string.as_mut().unwrap().as_ptr()
        }
    }
}

// Only used in tests
#[allow(dead_code)]
fn get_string(handler: &mut CFixedString) -> String {
    unsafe { CStr::from_ptr(handler.as_ptr()).to_string_lossy().into_owned() }
}

#[test]
fn test_string_handler() {
    {
        let short_string = "";
        let t = CFixedString::from_str(short_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), short_string);
    }
    {
        let short_string = "test_local";
        let t = CFixedString::from_str(short_string);
        assert_eq!(t.local, true);
        assert_eq!(get_string(&t), short_string);
    }
    {
        let short_string = "test_local stoheusthsotheost";
        let t = CFixedString::from_str(short_string);
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
        let t = CFixedString::from_str(test_511_string);
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
        let t = CFixedString::from_str(test_512_string);
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
        let t = CFixedString::from_str(test_513_string);
        assert_eq!(t.local, false);
        assert_eq!(get_string(&t), test_513_string);
    }
}



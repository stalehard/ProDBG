extern crate libc;

use libc::{c_char, c_int};
use std::ptr;

pub mod plugin_handler;

// test

extern {
    fn prodbg_main(argc: c_int, argv: *const c_char);
}

pub fn main() {
    println!("Start from Rust!");
    unsafe {
        prodbg_main(0, ptr::null())
    }
}

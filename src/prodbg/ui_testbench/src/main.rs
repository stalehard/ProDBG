extern crate core;
extern crate libc;
extern crate ui;

use libc::{c_void, c_int};
/*
use core::plugin_handler::*;
use std::ptr;
use std::mem::transmute;

#[repr(C)]
struct Context<'a> {
    plugin_handler: PluginHandler<'a>,
}
*/


const WIDTH: usize = 1280;
const HEIGHT: usize = 1024;

fn main() {
    let mut window = match ui::Window::new("Test - ESC to exit", WIDTH, HEIGHT, ui::Scale::X1) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    unsafe {
        bgfx_create();
        bgfx_create_window(window.get_native_handle(), WIDTH as i32, HEIGHT as i32);  
    }

    while window.is_open() && !window.is_key_down(ui::Key::Escape) {
        
        unsafe {
            bgfx_pre_update();
            bgfx_post_update();
        }

        window.update();
    }

    unsafe {
        bgfx_destroy();
    }


    /*
    let search_paths = vec!["../../..", "t2-output/macosx-clang-debug-default", "target/debug"];

    let mut context = Box::new(Context {
        plugin_handler: PluginHandler::new(search_paths, Some("t2-output")),
    });

    context.plugin_handler.add_plugin("bitmap_memory");
    context.plugin_handler.create_view_instance(&"Bitmap View".to_string());

    unsafe {
        // this is kinda ugly but we have no good way to pass this around
        bgfx_set_context(transmute(&mut *context));
        prodbg_main(0, ptr::null())
    }
    */
}

///
/// 
///
///

extern {
    //fn prodbg_main(argc: c_int, argv: *const c_char);

    fn bgfx_pre_update();
    fn bgfx_post_update();
    fn bgfx_create();
    fn bgfx_create_window(window: *const c_void, width: c_int, height: c_int);
    fn bgfx_destroy();
    /*


    fn bgfx_get_ui_funcs() -> *const c_void;

    fn bgfx_imgui_begin(show: c_int);
    fn bgfx_imgui_end();

    fn bgfx_imgui_set_window_pos(x: c_float, y: c_float);
    fn bgfx_imgui_set_window_size(x: c_float, y: c_float);

    fn bgfx_get_screen_width() -> f32;
    fn bgfx_get_screen_height() -> f32;

    fn bgfx_set_context(context: *mut c_void); 
    fn bgfx_get_context() -> *mut c_void;
    */
}

///
/// These are calls coming from the C/C++ Code
///

#[no_mangle]
pub extern fn prodbg_create(_: *const c_void, _: c_int, _: c_int) {
/*
    unsafe { 
        bgfx_create(); 
        bgfx_create_window(window, width, height);
    }
*/
}

#[no_mangle]
pub unsafe extern fn prodbg_timed_update() {
/*
    let context = bgfx_get_context() as *mut Context;
    let t = &mut (*context);

    // check if someone has poked our file!

    match t.plugin_handler.watch_recv.try_recv() {
        Ok(file) => {
            t.plugin_handler.reload_plugin(file.path.as_ref().unwrap());
            println!("Poked file! {}", file.path.unwrap().to_str().unwrap());
        }
        _ => (),
    }

    bgfx_pre_update();

    for instance in t.plugin_handler.view_instances.iter() {
        bgfx_imgui_set_window_pos(0.0, 0.0);
        bgfx_imgui_set_window_size(bgfx_get_screen_width(), bgfx_get_screen_height());

        bgfx_imgui_begin(1);

        let plugin_funcs = instance.plugin_type.plugin_funcs as *mut CViewPlugin; 
        ((*plugin_funcs).update)(instance.user_data, bgfx_get_ui_funcs(), ptr::null(), ptr::null());

        bgfx_imgui_end();
    }

    bgfx_post_update();
*/
}

#[no_mangle]
pub extern fn prodbg_application_launched() {
}

#[no_mangle]
pub extern fn prodbg_event(event_id: c_int) {
    println!("event {}", event_id);
}

#[no_mangle]
pub extern fn prodbg_destroy() {
/*
    unsafe {
        bgfx_destroy();

        // This is kinda ugly. Hopefully this can be sorted later
        let context = bgfx_get_context() as *mut Context;
        let t = &mut (*context);

        drop(t);
    }
*/
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern fn main_run() { }

#[no_mangle]
pub extern fn main_shutdown() { }







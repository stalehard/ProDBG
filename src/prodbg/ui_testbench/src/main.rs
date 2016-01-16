extern crate core;
extern crate libc;
extern crate ui;

use libc::{c_void, c_int, c_float};

use core::plugin_handler::*;
use std::ptr;

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

    let search_paths = vec!["../../..", "t2-output/macosx-clang-debug-default", "target/debug"];

    let mut plugin_handler = PluginHandler::new(search_paths, Some("t2-output"));

    plugin_handler.add_plugin("bitmap_memory");
    plugin_handler.create_view_instance(&"Bitmap View".to_string());

    unsafe {
        bgfx_create();
        bgfx_create_window(window.get_native_handle(), WIDTH as i32, HEIGHT as i32);  
    }

    while window.is_open() && !window.is_key_down(ui::Key::Escape) {
        match plugin_handler.watch_recv.try_recv() {
            Ok(file) => {
                plugin_handler.reload_plugin(file.path.as_ref().unwrap());
            }
            _ => (),
        }

        unsafe { 
            bgfx_pre_update();

            for instance in plugin_handler.view_instances.iter() {
                bgfx_imgui_set_window_pos(0.0, 0.0);
                bgfx_imgui_set_window_size(bgfx_get_screen_width(), bgfx_get_screen_height());

                bgfx_imgui_begin(1);

                let plugin_funcs = instance.plugin_type.plugin_funcs as *mut CViewPlugin; 
                ((*plugin_funcs).update)(instance.user_data, bgfx_get_ui_funcs(), ptr::null(), ptr::null());

                bgfx_imgui_end();
            }

            bgfx_post_update();
        }

        window.update();
    }

    unsafe {
        bgfx_destroy();
    }
}

///
/// 
///
///

extern {
    fn bgfx_pre_update();
    fn bgfx_post_update();
    fn bgfx_create();
    fn bgfx_create_window(window: *const c_void, width: c_int, height: c_int);
    fn bgfx_destroy();

    fn bgfx_get_ui_funcs() -> *const c_void;

    fn bgfx_imgui_begin(show: c_int);
    fn bgfx_imgui_end();

    fn bgfx_imgui_set_window_pos(x: c_float, y: c_float);
    fn bgfx_imgui_set_window_size(x: c_float, y: c_float);

    fn bgfx_get_screen_width() -> f32;
    fn bgfx_get_screen_height() -> f32;
}


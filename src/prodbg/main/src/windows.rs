extern crate minifb;

const WIDTH: usize = 1280;
const HEIGHT: usize = 1024;

struct Window {
    /// minifb window
    win: minifb::Window,

    /// views connected to this window
    view_instances: Vec<Rc<Plugin>>,



}



///! Windows keeps track of all different windows that are present with in the application
///! There are several ways windows can be created:
///!
///! 1. User opens a new window using a shortcut or menu selection.
///! 2. User "undocks" a view from an existing window giving it it's own floating window.
///! 3. etc

struct Windows {
    /// All the windows being tracked
    windows: Vec<Window>,
}

impl Windows {
    pub fn new() -> Windows {
        Windows { windows: Vec::new() }
    }

    /// Create a default window which will only be created if there are no other
    /// windows present.

    pub fn create_default_window(&mut self) {
        if self.windows.len() > 0 {
            return;
        }

        let window = Self::create_window(WIDTH, HEIGHT).expect("Unable to create window");

        self.windows.push(window)
    }

    pub fn create_window(width: usize, height: usize) -> minifb::Result<Window> {
        try!(Window::new("ProDBG", width, height,
                         WindowOptions {
                             resize: true,
                             scale: Scale::X1,
                             ..WindowOptions::default()
                         }));
    }

    /// Save the state of the windows (usually done when exiting the application)
    pub fn save(_filename: &str) {}

    /// Load the state of all the views from a previous run
    pub fn load(_filename: &str) {}
}

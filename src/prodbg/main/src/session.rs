use core::view_plugins::ViewPlugins;
use std::rc::Rc;
use std::cell::RefCell;

///! Session is a major part of ProDBG. There can be several sessions active at the same time
///! and each session has exactly one backend. There are only communication internally in a session
///! sessions can't (at least now) not talk to eachother.
///!
///! A backend can have several views at the same time. Data can be sent between the backend and
///| views using the PDReader/PDWriter APIs (prodbg::Writer prodbg::Reader in Rust) and this is the
///| only way for views and backends to talk with each other. There are several reasons for this
///| approach:
///!
///| 1. No "hacks" on trying to share memory. Plugins can be over a socket/webview/etc.
///! 2. Views and backends makes no assumetions on the inner workings of the others.
///! 3. Backends and views can post messages which anyone can decide to (optionally) act on.
///!
struct Session {
    local: bool,

    /// Handler for the view plugins
    view_plugins: Rc<RefCell<PluginHandler>>,

    /// Handler for the view plugins
    backend_plugins: Rc<RefCell<PluginHandler>>,

    backend: Option<Rc<Plugin>>,
}

impl Session {
    fn new() -> Session {
        Session {
            local: false,
            view_plugins: Vec::new(),
            backend: None,
        }
    }

    pub fn start_local(_: &str,  _: Option<&str>) {

    }

    pub fn start_remate(_: &str, _: usize) {

    }
}

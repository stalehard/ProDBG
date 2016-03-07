//use core::view_plugins::ViewPlugins;
use core::plugins::PluginHandler;
use core::plugin::Plugin;
use std::rc::Rc;

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
pub struct Session {
    /// Current backend (can be None if no backend is active)
    pub backend: Option<Rc<Plugin>>,

    /// Instacens of view plugins beloning to this backend
    pub view_instaces: Vec<Rc<Plugin>>,
}

///! Connection options for Remote connections. Currently just one Ip adderss
///!
pub struct ConnectionSettings<'a> {
    pub address: &'a str,
}

impl Session {
    pub fn new() -> Session {
        Session {
            backend: None,
            view_instaces: Vec::new(),
        }
    }

    pub fn start_remote(_plugin_handler: &PluginHandler, _settings: &ConnectionSettings) {

    }

    pub fn start_remate(_: &str, _: usize) {

    }
}

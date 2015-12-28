use std::path::{Path, PathBuf};
use std::fs;

static STANDARD_PLUGIN_TYPES: [&'static str; 2] = ["ProDBG View", "ProDBG Backend"];

struct Plugin {
    dummy: i32,
}

struct PluginHandler<'a> {
    plugins: Vec<Plugin>,
    search_paths: &'a [&'static str],
}

impl<'a> PluginHandler<'a> {
    pub fn new(search_paths: &'a [&'static str]) -> PluginHandler<'a> {
        PluginHandler {
            plugins: Vec::new(),
            search_paths: search_paths,
        }
    }

    fn search_plugin(&self, name: &String) -> Option<PathBuf> {
        for p in self.search_paths.iter() {
            let path = Path::new(p).join(name);
            match fs::metadata(&path) {
                Ok(md) => {
                    if md.is_file() {
                        return Some(path);
                    }
                }
                _ => (),
            }
        }

        None
    }

    pub fn add_plugin(clean_name: &str) {
        let name = Self::format_name(clean_name);


    }

    #[cfg(target_os="windows")]
    pub fn format_name(name: &str) -> String {
        format!("{}.dll", name)
    }

    #[cfg(target_os="macos")]
    pub fn format_name(name: &str) -> String {
        format!("lib{}.dylib", name)
    }

    #[cfg(any(target_os="linux",
          target_os="freebsd",
          target_os="dragonfly",
          target_os="bitrig",
          target_os="netbsd",
          target_os="openbsd"))]
    pub fn format_name(name: &str) -> String {
        format!("{}.so", name)
    }

    pub fn add_non_standard(name: &str) {}
}


#[test]
#[cfg(target_os="windows")]
pub fn test_format_name() {
    assert_eq!("test_plugin.dll", PluginHandler::format_name("test_plugin"));
}

#[test]
#[cfg(target_os="macos")]
pub fn test_format_name() {
    assert_eq!("libtest.dylib", PluginHandler::format_name("test"));
}

#[test]
#[cfg(any(target_os="linux",
          target_os="freebsd",
          target_os="dragonfly",
          target_os="bitrig",
          target_os="netbsd",
          target_os="openbsd"))]
pub fn test_format_name() {
    assert_eq!("test_plugin.so", PluginHandler::format_name("test_plugin"));
}

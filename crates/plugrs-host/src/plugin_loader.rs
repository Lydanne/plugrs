use libloading::{Library, Symbol};
use plugrs_interface::Plugin;
use std::ffi::OsStr;

pub struct PluginLoader {
    lib: Library,
}

impl PluginLoader {
    pub fn new<P: AsRef<OsStr>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(path)? };
        Ok(Self { lib })
    }

    pub fn load_plugin(&self) -> Result<Box<dyn Plugin>, Box<dyn std::error::Error>> {
        let creator: Symbol<fn() -> Box<dyn Plugin>> = unsafe { self.lib.get(b"create_plugin")? };
        Ok(creator())
    }
}

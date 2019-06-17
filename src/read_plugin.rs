use std::ffi::OsStr;
use std::os::raw::c_void;

use crate::c_plugin_factory::CPluginFactory;
use crate::libloading;

pub fn read_plugin<P: AsRef<OsStr>>(path: P) -> Result<CPluginFactory, String> {
    let library = libloading::Library::new(path).expect("Couldn't load plugin!");

    let get_plugin_factory: libloading::Symbol<unsafe extern "C" fn() -> *const c_void> = unsafe {
        library
            .get(b"GetPluginFactory")
            .expect("No GetPluginFactory function found in plugin!")
    };

    let c_plugin_factory = unsafe { get_plugin_factory() };
    let plugin_factory = CPluginFactory::new(c_plugin_factory);

    Ok(plugin_factory)
}

use std::os::raw::c_void;

pub mod f_unknown;
pub mod i_plugin_factory;

pub trait Interface {
    fn new(inner: *const c_void) -> Self;
    fn get_id() -> [u32; 4];
}
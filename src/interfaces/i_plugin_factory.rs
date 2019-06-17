use crate::interfaces::f_unknown;
use crate::interfaces::Interface;
use std::os::raw::c_void;

// Functions defined for all types compatible with IPluginFactory

// IPluginFactory struct

pub struct IPluginFactory {
    inner: *const IPluginFactoryImpl,
}

impl IPluginFactory {
    pub unsafe fn query_interface<T: Interface>(&self) -> T {
        f_unknown::query_interface_impl(
            self.inner as *const c_void,
            (*(*self.inner).vtable).query_interface,
        )
    }

    pub unsafe fn add_ref(&self) -> u32 {
        f_unknown::add_ref_impl(self.inner as *const c_void, (*(*self.inner).vtable).add_ref)
    }

    pub unsafe fn release(&self) -> u32 {
        f_unknown::release_impl(self.inner as *const c_void, (*(*self.inner).vtable).release)
    }

    pub fn hello(&self) {
        println!("Hello from IPluginFactory!");
    }
}

impl Interface for IPluginFactory {
    fn new(inner: *const c_void) -> Self {
        Self {
            inner: inner as *const IPluginFactoryImpl,
        }
    }

    fn get_id() -> [u32; 4] {
        [0x1C814D7A, 0x1F4A1152, 0xEED2D9AE, 0x9FBF430B]
    }
}

// Private implementation

#[derive(Debug)]
#[repr(C)]
struct IPluginFactoryImpl {
    vtable: *const IPluginFactoryVTable,
}

#[derive(Debug)]
#[repr(C)]
struct IPluginFactoryVTable {
    // FUnknown
    query_interface: f_unknown::QueryInterfaceFnType,
    add_ref: f_unknown::AddRefFnType,
    release: f_unknown::ReleaseFnType,

    // IPluginFactory
    f3: *const c_void, // TODO
    f4: *const c_void, // TODO
    f5: *const c_void, // TODO
    f6: *const c_void, // TODO
}

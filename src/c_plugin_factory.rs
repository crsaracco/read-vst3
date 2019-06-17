use std::os::raw::c_void;
use crate::interfaces::Interface;
use crate::interfaces::f_unknown::{QueryInterfaceFnType, query_interface_impl};

pub struct CPluginFactory {
    inner: *const CPluginFactoryImpl,
}

impl CPluginFactory {
    pub fn new(inner: *const c_void) -> Self {
        Self {
            inner: inner as *const CPluginFactoryImpl,
        }
    }

    pub unsafe fn query_interface<T: Interface>(&self) -> T {
        query_interface_impl(self.inner as *const c_void, (*(*self.inner).vtable).query_interface)
    }

    pub unsafe fn count_classes(&self) -> i32 {
        ((*(*self.inner).vtable).count_classes)(self.inner)
    }

    pub fn hello(&self) {
        println!("Hello from CPluginFactory!");
    }
}



#[derive(Debug)]
#[repr(C)]
struct CPluginFactoryImpl {
    vtable: *const CPluginFactoryVTable,
}

#[derive(Debug)]
#[repr(C)]
struct CPluginFactoryVTable {
    // FUnknown
    query_interface: QueryInterfaceFnType,
    f1: *const c_void, // TODO
    f2: *const c_void, // TODO

    // IPluginFactory
    f3: *const c_void, // TODO
    count_classes: extern fn(*const CPluginFactoryImpl) -> i32,
    f5: *const c_void, // TODO
    f6: *const c_void, // TODO

    // IPluginFactory2
    f7: *const c_void, // TODO

    // IPluginFactory3
    f8: *const c_void, // TODO
    f9: *const c_void, // TODO
}
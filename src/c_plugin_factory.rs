use std::os::raw::c_void;
use crate::interfaces::Interface;
use crate::interfaces::f_unknown::{FUnknown, QueryInterface};

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
        let mut vtable_ptr: *mut c_void = std::mem::uninitialized();
        let tuid = T::get_id();

        let result = ((*(*self.inner).vtable).query_interface)(
            self.inner as *const c_void,
            tuid.as_ptr() as *const i8,
            &mut vtable_ptr as *mut *mut c_void
        );

        let obj = T::new(vtable_ptr);
        obj
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
    query_interface: QueryInterface,
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
use std::os::raw::c_void;
use crate::interfaces::Interface;
use crate::interfaces::f_unknown::QueryInterface;

pub struct IPluginFactory {
    inner: *const IPluginFactoryImpl,
}

impl IPluginFactory {
    pub unsafe fn query_interface<T: Interface>(&self) -> T {
        let mut vtable_ptr: *mut c_void = std::mem::uninitialized();
        let tuid = T::get_id();

        let result = ((*(*self.inner).vtable).query_interface)(
            self.inner as *const c_void,
            tuid.as_ptr() as *const i8,
            vtable_ptr as *mut *mut c_void
        );

        let obj = T::new(vtable_ptr);
        obj
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



#[derive(Debug)]
#[repr(C)]
struct IPluginFactoryImpl {
    vtable: *const IPluginFactoryVTable,
}

#[derive(Debug)]
#[repr(C)]
struct IPluginFactoryVTable {
    // FUnknown
    query_interface: QueryInterface,
    f1: *const c_void, // TODO
    f2: *const c_void, // TODO

    // IPluginFactory
    f3: *const c_void, // TODO
    f4: *const c_void, // TODO
    f5: *const c_void, // TODO
    f6: *const c_void, // TODO
}
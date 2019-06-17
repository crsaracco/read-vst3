use std::os::raw::c_void;
use crate::interfaces::Interface;

pub type QueryInterface = extern fn(*const c_void, *const i8, *mut *mut c_void) -> i32;

pub struct FUnknown {
    inner: *const FUnknownImpl,
}

impl FUnknown {
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
        println!("Hello from FUnknown!");
    }
}

impl Interface for FUnknown {
    fn new(inner: *const c_void) -> Self {
        Self {
            inner: inner as *const FUnknownImpl,
        }
    }

    fn get_id() -> [u32; 4] {
        [0x00000000, 0x00000000, 0x000000C0, 0x46000000]
    }
}



#[derive(Debug)]
#[repr(C)]
struct FUnknownImpl {
    vtable: *const FUnknownVTable,
}

#[derive(Debug)]
#[repr(C)]
struct FUnknownVTable {
    // FUnknown
    query_interface: QueryInterface,
    f1: *const c_void, // TODO
    f2: *const c_void, // TODO
}
use std::os::raw::c_void;
use crate::interfaces::Interface;


// Functions defined for all types compatible with FUnknown


pub type QueryInterfaceFnType = extern fn(*const c_void, *const i8, *mut *mut c_void) -> i32;
pub unsafe fn query_interface_impl<T: Interface>(this: *const c_void, func: QueryInterfaceFnType) -> T {
    let mut vtable_ptr: *mut c_void = std::mem::uninitialized();
    let tuid = T::get_id();

    let result = func(
        this,
        tuid.as_ptr() as *const i8,
        &mut vtable_ptr as *mut *mut c_void
    );

    let obj = T::new(vtable_ptr);
    obj
}


// FUnknown struct


pub struct FUnknown {
    inner: *const FUnknownImpl,
}

impl FUnknown {
    pub unsafe fn query_interface<T: Interface>(&self) -> T {
        query_interface_impl(self.inner as *const c_void, (*(*self.inner).vtable).query_interface)
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


// Private implementation


#[derive(Debug)]
#[repr(C)]
struct FUnknownImpl {
    vtable: *const FUnknownVTable,
}

#[derive(Debug)]
#[repr(C)]
struct FUnknownVTable {
    // FUnknown
    query_interface: QueryInterfaceFnType,
    f1: *const c_void, // TODO
    f2: *const c_void, // TODO
}
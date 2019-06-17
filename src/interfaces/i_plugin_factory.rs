use crate::interfaces::f_unknown;
use crate::interfaces::Interface;
use std::os::raw::c_void;

// Functions defined for all types compatible with IPluginFactory

// virtual tresult PLUGIN_API getFactoryInfo (PFactoryInfo* info) = 0;
pub type GetFactoryInfoFnType = extern "C" fn(*const c_void, *mut c_void) -> i32;
pub unsafe fn get_factory_info_impl(this: *const c_void, func: GetFactoryInfoFnType) -> i32 {
    // TODO
    unimplemented!();
}

// virtual int32 PLUGIN_API countClasses () = 0;
pub type CountClassesFnType = extern "C" fn(*const c_void) -> i32;
pub unsafe fn count_classes_impl(this: *const c_void, func: CountClassesFnType) -> i32 {
    func(this)
}

// virtual tresult PLUGIN_API getClassInfo (int32 index, PClassInfo* info) = 0;
pub type GetClassInfoFnType = extern "C" fn(*const c_void, i32, *mut c_void) -> i32;
pub unsafe fn get_class_info_impl(this: *const c_void, func: GetClassInfoFnType) -> i32 {
    // TODO
    unimplemented!();
}

//virtual tresult PLUGIN_API createInstance (FIDString cid, FIDString _iid, void** obj) = 0;
pub type CreateInstanceFnType = extern "C" fn(*const c_void, *const u8, *const u8, *mut *mut c_void) -> i32;
pub unsafe fn create_instance_impl(this: *const c_void, func: CreateInstanceFnType) -> i32 {
    // TODO
    unimplemented!();
}

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

    // TODO: Create PFactoryInfo and PClassInfo to work with get_factory_info and get_class_info
    pub unsafe fn get_factory_info(&self) -> i32 {
        // TODO
        unimplemented!();
    }

    pub unsafe fn count_classes(&self) -> i32 {
        count_classes_impl(self.inner as *const c_void, (*(*self.inner).vtable).count_classes)
    }

    pub unsafe fn get_class_info(&self) -> i32 {
        // TODO
        unimplemented!();
    }

    // TODO: Figure out why there's an "FIDString" type, and replicate it in rust to work with create_instance
    pub unsafe fn create_instance(&self) -> i32 {
        // TODO
        unimplemented!();
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
    get_factory_info: GetFactoryInfoFnType,
    count_classes: CountClassesFnType,
    get_class_info: GetClassInfoFnType,
    create_instance: CreateInstanceFnType,
}

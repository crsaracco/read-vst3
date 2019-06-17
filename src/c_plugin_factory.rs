use crate::interfaces::f_unknown;
use crate::interfaces::Interface;
use std::os::raw::c_void;
use crate::interfaces::i_plugin_factory;

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
        i_plugin_factory::count_classes_impl(self.inner as *const c_void, (*(*self.inner).vtable).count_classes)
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
    query_interface: f_unknown::QueryInterfaceFnType,
    add_ref: f_unknown::AddRefFnType,
    release: f_unknown::ReleaseFnType,

    // IPluginFactory
    get_factory_info: i_plugin_factory::GetFactoryInfoFnType,
    count_classes: i_plugin_factory::CountClassesFnType,
    get_class_info: i_plugin_factory::GetClassInfoFnType,
    create_instance: i_plugin_factory::CreateInstanceFnType,

    // IPluginFactory2
    f7: *const c_void, // TODO

    // IPluginFactory3
    f8: *const c_void, // TODO
    f9: *const c_void, // TODO
}

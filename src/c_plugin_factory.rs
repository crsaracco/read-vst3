use std::os::raw::c_void;

pub struct CPluginFactory {
    inner: *const CPluginFactoryImpl,
}

impl CPluginFactory {
    pub fn new(inner: *const c_void) -> Self {
        Self {
            inner: inner as *const CPluginFactoryImpl,
        }
    }

    pub unsafe fn count_classes(&self) -> i32 {
        ((*(*self.inner).vtable).count_classes)(self.inner)
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
    f0: *const c_void, // TODO
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
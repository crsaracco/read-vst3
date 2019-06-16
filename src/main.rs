use std::os::raw::c_void;

extern crate libloading as lib;

const LIBRARY: &str = "so_files/adelay.so";

type CountClasses = extern fn(*mut IPluginFactory) -> i32;

#[allow(non_snake_case)]
#[repr(C)]
struct IPluginFactory {
    pub vtable: *mut VTable,
}

#[allow(non_snake_case)]
#[repr(C)]
struct VTable {
    pub values: [*mut c_void; 128],
}

fn main() {
    let library = lib::Library::new(LIBRARY).unwrap();

    unsafe {
        let get_plugin_factory: lib::Symbol<unsafe extern fn() -> *mut IPluginFactory> =
            library.get(b"GetPluginFactory").unwrap();
        let c_plugin_factory = get_plugin_factory();

        // IPluginFactory's concrete type is a Steinberg::CPluginFactory.
        // Steinberg::CPluginFactory has the following vtable:
        // 0: FUnknown::queryInterface
        // 1: FUnknown::addRef
        // 2: FUnknown::release
        // 3: IPluginFactory::getFactoryInfo
        // 4: IPluginFactory::countClasses
        // 5: IPluginFactory::getClassInfo
        // 6: IPluginFactory::createInstance
        // ...

        let count_classes: CountClasses = std::mem::transmute((*(*c_plugin_factory).vtable).values[4]);
        println!("classes: {}", count_classes(c_plugin_factory));
    }
}
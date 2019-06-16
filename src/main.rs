use std::os::raw::c_void;

extern crate libloading as lib;

const LIBRARY: &str = "so_files/adelay.so";

type CountClasses = extern fn(*mut IPluginFactory) -> i32;
type QueryInterface = extern fn(*mut IPluginFactory, *const i8, *mut *mut c_void) -> i32;

#[allow(non_snake_case)]
#[repr(C)]
struct TUID {
    id: [u32; 4],
}

const FUNKNOWN_IID: TUID = TUID {
    id: [0x00000000, 0x00000000, 0x000000C0, 0x46000000],
};

const IPLUGINFACTORY3_IID: TUID = TUID {
    id: [0xABA25545, 0x574E23C1, 0x1029129B, 0x31898736],
};

const IPLUGINFACTORY_IID: TUID = TUID {
    id: [0x1C814D7A, 0x1F4A1152, 0xEED2D9AE, 0x9FBF430B],
};

const IUPDATEMANAGER_IID: TUID = TUID {
    id: [0x0C780B03, 0x8D41E6D6, 0xC20BE08C, 0xD434C809],
};

#[allow(non_snake_case)]
#[repr(C)]
struct IPluginFactory {
    pub vtable: *mut IPluginFactoryVTable,
}

#[allow(non_snake_case)]
#[repr(C)]
struct IPluginFactoryVTable {
    pub values: [*mut c_void; 7],
}

fn query(plugin: *mut IPluginFactory, query_interface: QueryInterface, iid: [u32; 4]) {
    unsafe {
        let mut vtable_ptr: *mut c_void = std::mem::uninitialized();
        println!("vtable ptr: {:016x}", vtable_ptr as usize);

        let result = query_interface(
            plugin,
            iid.as_ptr() as *const i8,
            &mut vtable_ptr as *mut *mut c_void
        );

        println!("result: {}", result);
        if result != 0 {
            println!();
            return;
        }

        let asdf1: *const IPluginFactoryVTable = vtable_ptr as *const IPluginFactoryVTable;
        for v in 0..7 {
            println!("asdf1: {:016x}", (*asdf1).values[v] as usize);
        }

        let asdf2: *const IPluginFactoryVTable = (*asdf1).values[0] as *const IPluginFactoryVTable;
        for v in 0..7 {
            println!("vtable {}: {:016x}", v, (*asdf2).values[v] as usize);
        }

        println!();
    }
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

        let query_interface: QueryInterface = std::mem::transmute((*(*c_plugin_factory).vtable).values[0]);

        query(c_plugin_factory, query_interface, IPLUGINFACTORY_IID.id);
        query(c_plugin_factory, query_interface, IPLUGINFACTORY3_IID.id);
        query(c_plugin_factory, query_interface, FUNKNOWN_IID.id);
        query(c_plugin_factory, query_interface, IUPDATEMANAGER_IID.id);
    }
}
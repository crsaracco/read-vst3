use std::os::raw::c_void;

extern crate libloading;

mod read_plugin;
use read_plugin::read_plugin;
mod c_plugin_factory;
use c_plugin_factory::CPluginFactory;
mod interfaces;
use interfaces::f_unknown::{FUnknown};
use crate::interfaces::i_plugin_factory::IPluginFactory;

const LIBRARY: &str = "so_files/adelay.so";

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

fn main() {
    let plugin_factory = read_plugin(LIBRARY).unwrap();
    println!("{:?}", unsafe{plugin_factory.count_classes()});

    unsafe {
        plugin_factory.hello();

        let obj1 = plugin_factory.query_interface::<FUnknown>();
        obj1.hello();

        let obj2 = plugin_factory.query_interface::<IPluginFactory>();
        obj2.hello();

        let obj3 = plugin_factory.query_interface::<FUnknown>();
        obj3.hello();

        let obj4 = plugin_factory.query_interface::<IPluginFactory>();
        obj4.hello();
    }
}
extern crate libloading;

mod read_plugin;
use read_plugin::read_plugin;
mod c_plugin_factory;
mod interfaces;
use interfaces::f_unknown::FUnknown;
use interfaces::i_plugin_factory::IPluginFactory;

const LIBRARY: &str = "so_files/adelay.so";

fn main() {
    let plugin_factory = read_plugin(LIBRARY).unwrap();
    println!("{:?}", unsafe{plugin_factory.count_classes()});

    unsafe {
        plugin_factory.hello();

        let obj1 = plugin_factory.query_interface::<FUnknown>();
        obj1.hello();

        let obj2 = obj1.query_interface::<IPluginFactory>();
        obj2.hello();

        let obj3 = obj2.query_interface::<FUnknown>();
        obj3.hello();

        let obj4 = obj3.query_interface::<IPluginFactory>();
        obj4.hello();
    }
}
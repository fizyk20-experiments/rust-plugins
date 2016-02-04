extern crate plugin_interface;
extern crate libloading;
use plugin_interface::PluginTrait;
use libloading::{Library, Symbol};
use std::env::current_dir;

fn main() {
    let mut path = current_dir().unwrap();
    path.push("libplugin.so");
    println!("Path: {}", path.display());

    let x: &'static str;

    {
        let lib = Library::new(path.as_path()).unwrap();

        println!("Success.");

        let object_factory: Symbol<extern fn() -> Box<PluginTrait>> = unsafe {
            lib.get(b"object_factory").unwrap()
        };

        println!("Success 2.");

        let obj = object_factory();

        x = obj.get_some_string();

        println!("Success 3.");
    }

    println!("Result: {}", x);
}

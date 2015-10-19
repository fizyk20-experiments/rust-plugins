#![feature(dynamic_lib)]
extern crate plugin_interface;
use plugin_interface::PluginTrait;
use std::env::current_dir;
use std::dynamic_lib::DynamicLibrary;
use std::mem::transmute;

fn main() {
	let mut path = current_dir().unwrap();
	path.push("libplugin.so");
	println!("Path: {}", path.display());

	let lib = match DynamicLibrary::open(Some(path.as_path())) {
		Ok(lib) => lib,
		Err(error) => panic!("Library open failed: {}", error)
	};

	println!("Success.");

	let object_factory: extern fn() -> Box<PluginTrait> = unsafe {
		match lib.symbol::<u8>("object_factory") {
			Ok(fun) => transmute(fun),
			Err(error) => panic!("Reading symbol failed: {}", error)
		}
	};

	println!("Success 2.");

	let obj = object_factory();

	println!("Success 3.");

	println!("Result: {}", obj.get_some_string());
}
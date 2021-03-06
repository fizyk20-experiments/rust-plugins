extern crate plugin_interface;
use plugin_interface::PluginTrait;

struct Whatever;

impl PluginTrait for Whatever {
	fn get_some_string(&self) -> String { "whatever".to_string() }
}

#[no_mangle]
pub extern fn object_factory() -> Box<PluginTrait> {
	Box::new(Whatever)
}
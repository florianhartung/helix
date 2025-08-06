#[allow(warnings)]
mod bindings;

use bindings::helix::plugin as bindings_package;
use bindings::{Guest, LogLevel, PluginMetadata};

struct Component;

impl Guest for Component {
    fn initialize() {
        bindings::log(LogLevel::Info, "Hello from WebAssembly!");
    }

    fn handle_key_press(code: char) {
        let area = bindings::get_focus().get_area();
        bindings::log(LogLevel::Error, &format!("area of focused view: {}", area.width));

        if code == 'p' {
            bindings::close();
        }

        bindings::set_editor_status(&format!("area of focused view: {}", area.width));
    }

    fn get_metadata() -> PluginMetadata {
        PluginMetadata {
            name: "My first plugin".to_owned(),
            description: "This is just a plugin for development and testing purposes. It is currently built-in, which means it is automatically shipped with the helix runtime. Actually this feature does not exist yet but let's pretend it does.".to_owned(),
            keywords: vec!["hello".to_owned(), "world".to_owned()],
            requested_wasi_interfaces: bindings_package::types::RequestedWasiInterfaces::empty(),
        }
    }
}

bindings::export!(Component with_types_in bindings);

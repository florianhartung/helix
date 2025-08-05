#[allow(warnings)]
mod bindings;

use bindings::*;

struct Component;

impl Guest for Component {
    fn initialize() {
        log(LogLevel::Info, "Hello from WebAssembly!");
    }

    fn handle_key_press(code: char) {
        if code == 'p' {
            match close_buffer() {
                Ok(()) => set_editor_status("all gucci"),
                Err(err) => set_editor_status(&format!("error lol: {err}")),
            }
        }
        set_editor_status(&format!("rust plugin received keycode={code}"));
        log(LogLevel::Error, &format!("Rust plugin received keycode={code}"));
    }

    fn get_metadata() -> PluginMetadata {
        PluginMetadata {
            name: "My first plugin".to_owned(),
            description: "This is just a plugin for development and testing purposes. It is currently built-in, which means it is automatically shipped with the helix runtime. Actually this feature does not exist yet but let's pretend it does.".to_owned(),
            keywords: vec!["hello".to_owned(), "world".to_owned()],
            requested_wasi_interfaces: helix::plugin::types::RequestedWasiInterfaces::empty(),
        }
    }
}

export!(Component with_types_in bindings);
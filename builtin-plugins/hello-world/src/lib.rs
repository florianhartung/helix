#[allow(warnings)]
mod bindings;

use bindings::*;

struct Component;

impl Guest for Component {
    fn initialize() {
        log(LogLevel::Info, "Hello from WebAssembly!");
        test();
    }

    fn handle_key_press(code: char) {
        set_editor_status(&format!("plugin got keycode={code}"));
    }

    fn get_metadata() -> PluginMetadata {
        PluginMetadata {
            name: "My first plugin".to_owned(),
            description: "This is just a plugin for development and testing purposes. It is currently built-in, which means it is automatically shipped with the helix runtime. Actually this feature does not exist yet but let's pretend it does.".to_owned(),
            keywords: vec!["hello".to_owned(), "world".to_owned()],
        }
    }
}

export!(Component with_types_in bindings);

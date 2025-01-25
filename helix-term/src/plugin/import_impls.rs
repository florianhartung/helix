use helix_view::Editor;
use wasmtime::component::Linker;

use crate::{compositor::Compositor, plugin::{bindings, Base, Keyevents}};

use super::{temporary_owned_reference::TemporaryOwnedBorrowMut, PluginState};

pub struct ImportImpls {
    pub(crate) cx: Option<Cx>,

    pub(crate) keyevents_interface_valid: bool,
}

pub struct Cx {
    pub current_plugin: TemporaryOwnedBorrowMut<PluginState>,
    pub editor: TemporaryOwnedBorrowMut<Editor>,
    pub compositor: TemporaryOwnedBorrowMut<Compositor>,
}


impl ImportImpls {
    pub fn new() -> Self {
        Self {
            cx: None,
            keyevents_interface_valid: false,
        }
    }

    pub fn add_to_linker<T>(
        linker: &mut Linker<T>,
        f: impl Fn(&mut T) -> &mut Self + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()> {
        Base::add_to_linker(linker, f)?;
        Keyevents::add_to_linker(linker, f)?;

        Ok(())
    }
}

impl ImportImpls {
    pub fn expect_cx<'a>(&'a mut self) -> &'a mut Cx {
        self.cx
            .as_mut()
            .expect("helix context to be present during host imports")
    }
}

impl bindings::helix::plugin::types::Host for ImportImpls {}

impl bindings::KeyeventsImports for ImportImpls {
    fn test(&mut self) {
        if !self.keyevents_interface_valid {
            // TODO handle this error differently?
            panic!("interface for optional world `keyevents` is not yet properly validated");
        }

        println!("keyevents::test called");
    }
}

impl bindings::BaseImports for ImportImpls {
    fn log(&mut self, log_level: bindings::LogLevel, msg: String) {
        let plugin_name = &self.expect_cx().current_plugin.metadata.name;
        match log_level {
            bindings::LogLevel::Info => log::info!("from {plugin_name}: {msg}"),
            bindings::LogLevel::Warn => log::warn!("from {plugin_name}: {msg}"),
            bindings::LogLevel::Error => log::error!("from {plugin_name}: {msg}"),
        }
    }

    fn set_editor_status(&mut self, msg: String) {
        self.expect_cx().editor.set_status(msg);
    }

    fn get_text_selection(&mut self) -> Option<String> {
        todo!()
    }
}

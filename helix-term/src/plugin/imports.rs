use helix_view::{Editor, ViewId};
use wasmtime::component::Linker;

use crate::{
    compositor::{self, Compositor},
    job::Jobs,
    plugin::{bindings, Base},
};

use super::{
    bindings::RunTypedCommands, temporary_owned_reference::TemporaryOwnedBorrowMut, PluginState,
};

/// Imports for a WASM component.
/// These are functions helix exposes to the plugin.
pub struct Imports {
    pub(crate) cx: Option<Cx>,

    pub(crate) keyevents_interface_valid: bool,
    pub(crate) run_typed_commands_interface_valid: bool,
}

/// Context that is stored temporarily when a plugin is invoked.
/// This struct cannot have any lifetime associated to it, because it is stored in a `wasmtime::Store<T>` only temporarily.
/// That's why `TemporaryOwnedBorrowMut` is used: It allows to own a mutable reference temporarily without a lifetime associated to it, while still providing a safe interface.
pub struct Cx {
    pub current_plugin: TemporaryOwnedBorrowMut<PluginState>,
    pub editor: TemporaryOwnedBorrowMut<Editor>,
    pub compositor: TemporaryOwnedBorrowMut<Compositor>,

    // Needed for compositor::Context
    pub scroll: Option<usize>,
    pub jobs: TemporaryOwnedBorrowMut<Jobs>,

    /// Needed for resource management. Mapping of resource ids to actual resources in the editor. Ids should be randomized and use of invalid ids immediately punished. That is because currently Wasm code can still forge resources as they are simply encoded using a u32. Maybe externrefs could be used in the future so that Wasm cannot forge or inspect resources anymore?
    /// Including GC support will resolve this, because it allows externrefs to be used as handles. See https://github.com/WebAssembly/component-model/issues/525
    resources: HashMap<u32, Resource>,
}

#[derive(Clone)]
enum Resource {
    EditorState,
    ViewId(ViewId)
}

impl Imports {
    pub fn new() -> Self {
        Self {
            cx: None,
            keyevents_interface_valid: false,
            run_typed_commands_interface_valid: false,
        }
    }

    pub fn add_to_linker<T>(
        linker: &mut Linker<T>,
        f: impl Fn(&mut T) -> &mut Self + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()> {
        Base::add_to_linker(linker, f)?;
        RunTypedCommands::add_to_linker(linker, f)?;

        Ok(())
    }
}

impl Imports {
    pub fn expect_cx<'a>(&'a mut self) -> &'a mut Cx {
        self.cx
            .as_mut()
            .expect("helix context to be present during host imports")
    }
}

impl bindings::helix::plugin::types::HostEditor for Imports {
    fn new(&mut self) -> wasmtime::component::Resource<Editor> {
        let Cx { editor, .. } = self.expect_cx();

        wasmtime::component::Resource::new_borrow(1) // lets use 1 for the editor for now
    }

    fn get_tree(
        &mut self,
        self_: wasmtime::component::Resource<Editor>,
    ) -> wasmtime::component::Resource<Tree> {
        todo!()
    }

    fn close(
        &mut self,
        self_: wasmtime::component::Resource<Editor>,
        view: wasmtime::component::Resource<ViewId>,
    ) -> () {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Editor>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl bindings::helix::plugin::types::HostTree for Imports {
    fn get_focus(&mut self,self_:wasmtime::component::Resource<bindings::helix::plugin::types::Tree>,) -> wasmtime::component::Resource<ViewId> {
        self_.rep()

    }

    fn get(&mut self,self_:wasmtime::component::Resource<Tree>,) -> wasmtime::component::Resource<View> {
        todo!()
    }

    fn drop(&mut self,rep:wasmtime::component::Resource<Tree>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl bindings::helix::plugin::types::Host for Imports {}

impl bindings::BaseImports for Imports {
    fn log(&mut self, log_level: bindings::LogLevel, msg: String) {
        let plugin_name = &self.expect_cx().current_plugin.metadata.name;
        match log_level {
            bindings::LogLevel::Info => log::info!("[PLUGIN=\"{plugin_name}\"] {msg}"),
            bindings::LogLevel::Warn => log::warn!("[PLUGIN=\"{plugin_name}\"] {msg}"),
            bindings::LogLevel::Error => log::error!("[PLUGIN=\"{plugin_name}\"] {msg}"),
        }
    }

    fn set_editor_status(&mut self, msg: String) {
        self.expect_cx().editor.set_status(msg);
    }

    fn get_text_selection(&mut self) -> Option<String> {
        todo!()
    }
}

impl bindings::RunTypedCommandsImports for Imports {
    fn close_buffer(&mut self) -> Result<(), wasmtime::component::__internal::String> {
        let cx = self.expect_cx();
        let editor = &mut *cx.editor;

        let current_doc_id = view!(editor).doc;

        let mut compositor_cx = compositor::Context {
            editor,
            jobs: &mut cx.jobs,
            scroll: None,
        };

        compositor_cx
            .block_try_flush_writes()
            .map_err(|err| err.to_string())?;
        compositor_cx
            .editor
            .close_document(current_doc_id, true)
            .map_err(|_err| "failed to close buffer".to_owned())?;

        Ok(())
    }
}

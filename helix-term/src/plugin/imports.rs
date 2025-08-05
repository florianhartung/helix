use std::{any::Any, collections::HashMap};

use bindings::helix::plugin as interface_bindings;
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
    pub resources: Vec<Resource>,
}

impl Cx {
    fn allocate_resource<T: 'static>(
        &mut self,
        resource: Resource,
    ) -> wasmtime::component::Resource<T> {
        let idx = self.resources.len();
        self.resources.push(resource);
        wasmtime::component::Resource::new_borrow(idx.try_into().unwrap())
    }

    fn get_resource<T: 'static>(
        &mut self,
        resource: wasmtime::component::Resource<T>,
    ) -> &Resource {
        self.resources
            .get(usize::try_from(resource.rep()).unwrap())
            .unwrap()
    }
}

#[derive(Clone)]
pub enum Resource {
    EditorState,
    View(ViewId),
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

impl interface_bindings::types::HostEditor for Imports {
    fn new(&mut self) -> wasmtime::component::Resource<interface_bindings::types::Editor> {
        let cx = self.expect_cx();

        cx.allocate_resource(Resource::EditorState)
    }

    fn get_focus(
        &mut self,
        editor: wasmtime::component::Resource<interface_bindings::types::Editor>,
    ) -> wasmtime::component::Resource<interface_bindings::types::View> {
        let cx = self.expect_cx();
        let Resource::EditorState = cx.get_resource(editor) else {
            panic!("bug in plugin system or malicious plugin detected");
        };

        cx.allocate_resource(Resource::View(cx.editor.tree.focus))
    }

    fn remove_view(
        &mut self,
        editor: wasmtime::component::Resource<interface_bindings::types::Editor>,
        view: wasmtime::component::Resource<interface_bindings::types::View>,
    ) {
        let cx = self.expect_cx();
        let Resource::EditorState = cx.get_resource(editor) else {
            panic!("bug in plugin system or malicious plugin detected");
        };

        let Resource::View(view_id) = cx.get_resource(view).clone() else {
            panic!("bug in plugin system or malicious plugin detected");
        };

        cx.editor.tree.remove(view_id);
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<interface_bindings::types::Editor>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }

    fn close(
        &mut self,
        self_: wasmtime::component::Resource<interface_bindings::types::Editor>,
    ) -> () {
        todo!()
    }
}

impl interface_bindings::types::HostView for Imports {
    fn get_area(
        &mut self,
        self_: wasmtime::component::Resource<interface_bindings::types::View>,
    ) -> interface_bindings::types::Rect {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<interface_bindings::types::View>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl interface_bindings::types::Host for Imports {}

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

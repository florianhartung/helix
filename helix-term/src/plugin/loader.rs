use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};

use super::{
    bindings::EditorContext, bindings::RunTypedCommands, imports::Imports, Base, Keyevents, Plugin,
    PluginInterface, PluginState,
};

pub struct PluginLoader {
    /// The linker can be shared between all components/plugins
    linker: Linker<Imports>,
    enable_caching: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("wasm file not found")]
    FileNotFound(#[source] anyhow::Error),
    #[error("failed to load wasm component because it is invalid: {0}")]
    WasmComponentInvalid(#[source] anyhow::Error),
    #[error("failed to instantiate wasm component: {0}")]
    WasmComponentInstantiationFailed(#[source] wasmtime::Error),
}

impl PluginLoader {
    pub fn new(wasm_engine: &Engine, enable_caching: bool) -> Self {
        let mut linker: Linker<Imports> = Linker::new(&wasm_engine);

        Imports::add_to_linker(&mut linker, |import_impls| import_impls).unwrap();

        Self {
            linker,
            enable_caching,
        }
    }

    pub fn load<'a>(&'a mut self, path: impl AsRef<Path>) -> Result<Plugin, Error> {
        let component =
            compile_and_cache_component(self.linker.engine(), path, self.enable_caching)
                .map_err(Error::WasmComponentInvalid)?;

        let mut interface = instantiate_plugin_interface(&mut self.linker, component)
            .map_err(Error::WasmComponentInstantiationFailed)?;

        let state = initialize_plugin_state(&mut interface).map_err(Error::WasmComponentInvalid)?;

        Ok(Plugin { interface, state })
    }
}

fn initialize_plugin_state(interface: &mut PluginInterface) -> anyhow::Result<PluginState> {
    let metadata = interface
        .base_bindings
        .call_get_metadata(&mut interface.store)
        .context("failed to get plugin metadata")?;

    Ok(PluginState { metadata })
}

fn instantiate_plugin_interface(
    linker: &mut Linker<Imports>,
    component: Component,
) -> wasmtime::Result<PluginInterface> {
    let mut store = Store::new(linker.engine(), Imports::new());
    let instance = linker.instantiate(&mut store, &component)?;

    let base_bindings = Base::new(&mut store, &instance)?;

    // Only unlock import implementation for optional world (such as `keyevents`) if its entire interface is valid
    let keyevents_bindings = Keyevents::new(&mut store, &instance).ok();
    if keyevents_bindings.is_some() {
        store.data_mut().keyevents_interface_valid = true;
    }

    let run_typed_commands_bindings = RunTypedCommands::new(&mut store, &instance).ok();
    if run_typed_commands_bindings.is_some() {
        store.data_mut().run_typed_commands_interface_valid = true;
    }

    let editor_context_bindings = EditorContext::new(&mut store, &instance).ok();
    if editor_context_bindings.is_some() {
        store.data_mut().run_typed_commands_interface_valid = true;
    }

    Ok(PluginInterface {
        store,
        base_bindings,
        keyevents_bindings,
        run_typed_commands_bindings,
        editor_context_bindings,
    })
}

const CRANELIFT_WASM_CACHE: &str = "/home/flo/.cache/helix-plugins";

fn compile_and_cache_component(
    engine: &wasmtime::Engine,
    path: impl AsRef<Path>,
    enable_caching: bool,
) -> anyhow::Result<Component> {
    let file_in_cache = PathBuf::try_from(CRANELIFT_WASM_CACHE)
        .unwrap()
        .join(path.as_ref().with_extension("").file_name().unwrap());

    let maybe_precompiled = File::open(&file_in_cache);
    if let (true, Ok(mut precompiled)) = (enable_caching, maybe_precompiled) {
        let mut bytes = Vec::new();
        precompiled.read_to_end(&mut bytes)?;

        // TODO ensure integrity though hashing checks
        unsafe { Component::deserialize(engine, bytes) }
    } else {
        let wasm_bytes = std::fs::read(&path).map_err(|err| Error::FileNotFound(err.into()))?;

        let component = Component::new(engine, wasm_bytes)?;
        let serialized = component.serialize().unwrap();

        std::fs::write(file_in_cache, serialized).unwrap();

        Ok(component)
    }
}

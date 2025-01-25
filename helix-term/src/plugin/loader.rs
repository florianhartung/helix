use std::path::Path;

use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};

use super::{import_impls::ImportImpls, Base, Keyevents, Plugin, PluginInterface, PluginState};

pub struct PluginLoader {
    /// The linker can be shared between all components/plugins
    linker: Linker<ImportImpls>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("wasm file not found")]
    FileNotFound(#[source] anyhow::Error),
    #[error("failed to load wasm component because it is invalid")]
    WasmComponentInvalid(#[source] anyhow::Error),
}

impl PluginLoader {
    pub fn new(wasm_engine: &Engine) -> Self {
        let mut linker: Linker<ImportImpls> = Linker::new(&wasm_engine);

        ImportImpls::add_to_linker(&mut linker, |import_impls| import_impls).unwrap();

        Self { linker }
    }

    pub fn load<'a>(
        &'a mut self,
        paths: impl Iterator<Item = impl AsRef<Path>> + 'a,
    ) -> impl Iterator<Item = Result<Plugin, Error>> + 'a {
        paths.into_iter().map(|path| {
            let wasm_bytes = std::fs::read(path).map_err(|err| Error::FileNotFound(err.into()))?;

            let mut interface = instantiate_plugin_interface(&mut self.linker, &wasm_bytes)
                .context("failed to instantiate plugin interface")
                .map_err(Error::WasmComponentInvalid)?;

            let state =
                initialize_plugin_state(&mut interface).map_err(Error::WasmComponentInvalid)?;

            Ok(Plugin { interface, state })
        })
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
    linker: &mut Linker<ImportImpls>,
    wasm_bytes: &[u8],
) -> wasmtime::Result<PluginInterface> {
    let component = Component::new(linker.engine(), wasm_bytes)?;
    let mut store = Store::new(linker.engine(), ImportImpls::new());
    let instance = linker.instantiate(&mut store, &component)?;

    let base_bindings = Base::new(&mut store, &instance).unwrap();

    // Only unlock import implementation for optional world (such as `keyevents`) if its entire interface is valid
    let keyevents_bindings = Keyevents::new(&mut store, &instance).ok();
    if keyevents_bindings.is_some() {
        store.data_mut().keyevents_interface_valid = true;
    }

    Ok(PluginInterface {
        store,
        base_bindings,
        keyevents_bindings,
    })
}

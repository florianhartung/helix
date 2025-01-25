use bindings::*;
use import_impls::ImportImpls;
use loader::PluginLoader;
use wasmtime::{Engine, Store};

mod bindings;
mod exports;
mod import_impls;
mod loader;
mod temporary_owned_reference;


/// # The central plugin system
///
/// There exists only one instance of this type per editor process.
/// This contains the central logic for all plugin related stuff.
pub struct PluginSystem {
    plugins: Vec<Plugin>,
}

impl PluginSystem {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut loader = PluginLoader::new(&engine);

        let paths =
            ["builtin-plugins/hello-world/target/wasm32-unknown-unknown/debug/hello_world.wasm"];

        let plugins = loader
            .load(paths.iter())
            .filter_map(|plugin| {
                plugin
                    .inspect_err(|err| {
                        log::warn!("Failed to load plugin: {err:?}");
                    })
                    .ok()
            })
            .collect();

        Self { plugins }
    }
}

/// # A loaded plugin
/// 
/// It has
/// 1. an interface to interact with its WASM component and
/// 2. a state that is used to plugin-specific information.
pub struct Plugin {
    /// The [`PluginInterface`] is used to interact with the underlying WASM component.
    interface: PluginInterface,

    /// Per-plugin information is stored in a [`PluginState`]
    state: PluginState,
}

pub struct PluginInterface {
    store: Store<ImportImpls>,
    base_bindings: Base,
    keyevents_bindings: Option<Keyevents>,
}

pub struct PluginState {
    metadata: PluginMetadata,
}
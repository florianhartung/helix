use std::{ffi::OsStr, path::Path};

use bindings::*;
use imports::Imports;
use loader::PluginLoader;
use wasmtime::{Config, Engine, Store};

mod bindings;
mod exports;
mod imports;
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
    // TODO add dedicated methods for loading, unloading and reloading plugins dynamically
    pub fn new(
        search_dirs: impl Iterator<Item = impl AsRef<Path>>,
        enable_caching: bool,
    ) -> Result<Self, (Self, anyhow::Error)> {
        let config = Config::new();
        let engine = Engine::new(&config).unwrap();
        let mut loader = PluginLoader::new(&engine, enable_caching);

        let mut search_dir_errors_occurred = false;
        let mut file_path_error_occurred = false;
        let mut wasm_file_error_occurred = false;

        let search_dir_children = search_dirs
            .flat_map(|search_dir| {
                std::fs::read_dir(search_dir.as_ref())
                    .map_err(|err| {
                        log::warn!(
                            "Failed to read children of directory {:?} with error: {err}",
                            search_dir.as_ref(),
                        );
                        search_dir_errors_occurred = true;
                    })
                    .ok()
            })
            .flatten();

        let wasm_paths = search_dir_children.flat_map(|maybe_plugin| {
            maybe_plugin
                .map_err(|err| {
                    log::warn!("Failed to load wasm path: {err}");
                    file_path_error_occurred = true;
                })
                .ok()
                .filter(|plugin| plugin.path().extension() == Some(OsStr::new("wasm")))
        });

        let plugins = wasm_paths
            .filter_map(|wasm_path| {
                loader
                    .load(&wasm_path.path())
                    .map_err(|err| {
                        log::warn!("Failed to load wasm path: {err}");
                        wasm_file_error_occurred = true;
                    })
                    .ok()
            })
            .collect::<Vec<Plugin>>();

        let plugin_system = Self { plugins };

        if search_dir_errors_occurred || file_path_error_occurred || wasm_file_error_occurred {
            Err((
                plugin_system,
                anyhow::anyhow!("some plugin(s) failed to load; see logs for more information"),
            ))
        } else {
            Ok(plugin_system)
        }
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
    store: Store<Imports>,

    base_bindings: Base,
    keyevents_bindings: Option<Keyevents>,
    run_typed_commands_bindings: Option<RunTypedCommands>,
    editor_context_bindings: Option<EditorContext>
}

pub struct PluginState {
    metadata: PluginMetadata,
}

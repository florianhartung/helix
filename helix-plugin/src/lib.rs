use plugin_invocation_context::PluginInvocationCx;
use wasmtime::{component::*, Engine, Store};

pub mod plugin_invocation_context;

mod bindings;
use bindings::*;
mod import_impls;
use import_impls::ImportImpls;

// pub struct PluginLoader {
//     builtin_plugin_dir: PathBuf,
// }

pub struct Plugin {
    // Option so it can be taken out for providing helix context
    store: Option<Store<StoreState<'static>>>,
    metadata: PluginMetadata,

    base_bindings: Base,
    keyevents_bindings: Option<Keyevents>,
}

/// # The central plugin system
///
/// There exists only one instance of this type per editor process.
/// This contains the central logic for all plugin related stuff.
pub struct PluginSystem {
    plugins: Vec<Plugin>,
}

struct StoreState<'hx_ctx> {
    import_impls: ImportImpls<'hx_ctx>,
}

impl PluginSystem {
    pub fn new() -> Self {
        let engine = Engine::default();

        // The linker can be shared between all components/plugins
        let mut linker: Linker<StoreState<'static>> = Linker::new(&engine);
        ImportImpls::add_to_linker(&mut linker, |store_state| &mut store_state.import_impls)
            .unwrap();

        let plugin_paths =
            &["builtin-plugins/hello-world/target/wasm32-unknown-unknown/debug/hello_world.wasm"];

        let plugins = plugin_paths
            .into_iter()
            .map(|path| {
                instantiate_plugin(
                    &mut linker,
                    Component::new(&engine, std::fs::read(path).unwrap()).unwrap(),
                )
                .unwrap()
            })
            .collect::<Vec<Plugin>>();

        Self { plugins }
    }

    pub fn initialize(&mut self) {
        let plugin0 = &mut self.plugins[0];
        plugin0
            .base_bindings
            .call_initialize(plugin0.store.as_mut().unwrap())
            .unwrap()
    }

    pub fn on_key_press<'a>(&mut self, cx: PluginInvocationCx<'a>, c: char) {
        // We do this so each plugin's store can take ownership of the context temporarily
        let mut cx = Some(cx);

        for plugin in &mut self.plugins {
            if let Some(keyevents) = &plugin.keyevents_bindings {
                dirty_wasmtime_stuff::provide_context_to_store(
                    &mut plugin.store,
                    &mut cx,
                    |store| {
                        keyevents.call_handle_key_press(store, c).unwrap();
                    },
                );
            }
        }
    }
}

fn instantiate_plugin(
    linker: &mut Linker<StoreState<'static>>,
    component: Component,
) -> wasmtime::Result<Plugin> {
    let mut store = Store::new(
        linker.engine(),
        StoreState {
            import_impls: ImportImpls::new(),
        },
    );

    let instance = linker.instantiate(&mut store, &component)?;

    let base_bindings = Base::new(&mut store, &instance).unwrap();

    // Only unlock import implementation for optional world (such as `keyevents`) if its entire interface is valid
    let keyevents_bindings = Keyevents::new(&mut store, &instance).ok();
    if keyevents_bindings.is_some() {
        store.data_mut().import_impls.keyevents_locked = false;
    }

    let metadata = base_bindings.call_get_metadata(&mut store).unwrap();

    Ok(Plugin {
        store: Some(store),
        metadata,
        base_bindings,
        keyevents_bindings,
    })
}

// DO NOT OPEN
mod dirty_wasmtime_stuff {
    use wasmtime::Store;

    use crate::{plugin_invocation_context::PluginInvocationCx, StoreState};

    /// Options are used for `store` and `cx` because ownership of those is needed temporarily
    /// # Panics
    /// If either `store` or `cx` is `None`.
    pub fn provide_context_to_store<'a>(
        store: &mut Option<Store<StoreState<'static>>>,
        cx: &mut Option<PluginInvocationCx<'a>>,
        f: impl FnOnce(&mut Store<StoreState<'a>>),
    ) {
        let owned_cx = cx.take().unwrap();
        let owned_store = store.take().unwrap();

        // Take empty store (without context) from plugin and inject our 'a context
        let mut owned_store = unsafe { inject_context(owned_store, owned_cx) };

        // Do stuff with store and injected borrowed context
        f(&mut owned_store);

        // Take borrowed context out of store
        let (owned_store, owned_cx) = unsafe { take_context(owned_store) };

        *store = Some(owned_store);
        *cx = Some(owned_cx);
    }

    // This should be safe because it just simulates recreating the Store instance with a different generic lifetime.
    // Here we only have to hope the Rust compiler does not change the layout of the inner generic when a different lifetime is uesd
    // The same goes for [`take_context`].
    unsafe fn inject_context<'a>(
        store: Store<StoreState<'static>>,
        cx: PluginInvocationCx<'a>,
    ) -> Store<StoreState<'a>> {
        assert!(store.data().import_impls.helix_context.is_none());
        let mut store = unsafe {
            std::mem::transmute::<Store<StoreState<'static>>, Store<StoreState<'a>>>(store)
        };

        store.data_mut().import_impls.helix_context = Some(cx);

        store
    }

    unsafe fn take_context<'a>(
        mut store: Store<StoreState<'a>>,
    ) -> (Store<StoreState<'static>>, PluginInvocationCx<'a>) {
        assert!(store.data().import_impls.helix_context.is_some());
        let cx = store.data_mut().import_impls.helix_context.take().unwrap();

        let store = unsafe {
            std::mem::transmute::<Store<StoreState<'a>>, Store<StoreState<'static>>>(store)
        };

        (store, cx)
    }

}


// struct Borrowed<T> {
//     // This outlives the current wasm execution
//     inner: *mut T,
// }
use crate::plugin::temporary_owned_reference::TemporaryOwnedBorrowMut;
use crate::{compositor::Compositor, job::Jobs};
use helix_view::Editor;

use crate::plugin::PluginSystem;

use super::{imports::Cx, Plugin, PluginInterface};

impl PluginSystem {
    // TODO fn prepare_call_plugin which injects the context and takes a closure impl FnOnce(bindings: Base) { c}in which the actual wasm function can be called

    pub fn initialize(
        &mut self,
        editor: &mut Editor,
        compositor: &mut Compositor,
        scroll: Option<usize>,
        jobs: &mut Jobs,
    ) {
        self.plugins.iter_mut().for_each(|plugin| {
            inject_helix_context_into_plugin_store(
                editor,
                compositor,
                plugin,
                scroll,
                jobs,
                |plugin| {
                    plugin
                        .base_bindings
                        .call_initialize(&mut plugin.store)
                        .unwrap()
                },
            )
        });
    }

    pub fn on_key_press(
        &mut self,
        editor: &mut Editor,
        compositor: &mut Compositor,
        scroll: Option<usize>,
        jobs: &mut Jobs,
        c: char,
    ) {
        self.plugins.iter_mut().for_each(|plugin| {
            inject_helix_context_into_plugin_store(
                editor,
                compositor,
                plugin,
                scroll,
                jobs,
                |plugin| {
                    if let Some(keyevents_bindings) = plugin.keyevents_bindings.as_mut() {
                        keyevents_bindings
                            .call_handle_key_press(&mut plugin.store, c)
                            .unwrap();
                    }
                },
            );
        });
    }
}

// Don't look at this. It basically only converts rust references to owned values temporarily, so that they can be stored in the [`wasmtime::component::Store`].
fn inject_helix_context_into_plugin_store<R>(
    editor: &mut Editor,
    compositor: &mut Compositor,
    plugin: &mut Plugin,
    scroll: Option<usize>,
    jobs: &mut Jobs,
    f: impl FnOnce(&mut PluginInterface) -> R,
) -> R {
    TemporaryOwnedBorrowMut::provide_with_all(
        (editor, compositor, &mut plugin.state, jobs),
        |(editor_owned, compositor_owned, plugin_state_owned, jobs_owned)| {
            if std::mem::replace(
                &mut plugin.interface.store.data_mut().cx,
                Some(Cx {
                    current_plugin: plugin_state_owned,
                    editor: editor_owned,
                    compositor: compositor_owned,
                    scroll: scroll.clone(),
                    jobs: jobs_owned,
                    resources: Vec::new(),
                }),
            )
            .is_some()
            {
                unreachable!(
                    "there can not be an editor context already present, when we try to insert one"
                );
            }

            let return_value = f(&mut plugin.interface);

            let cx = plugin
                .interface
                .store
                .data_mut()
                .cx
                .take()
                .expect("previously stored owned borrow context to be present");

            // give back all temporary owned borrows
            (
                (cx.editor, cx.compositor, cx.current_plugin, cx.jobs),
                return_value,
            )
        },
    )
}

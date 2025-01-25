use crate::compositor::Compositor;
use crate::plugin::temporary_owned_reference::TemporaryOwnedBorrowMut;
use helix_view::Editor;

use crate::plugin::PluginSystem;

use super::import_impls::Cx;
use super::{Plugin, PluginInterface};

impl PluginSystem {
    // TODO fn provide_context which takes a closure in which the actual wasm function can be called

    pub fn initialize(&mut self, editor: &mut Editor, compositor: &mut Compositor) {
        self.plugins.iter_mut().for_each(|plugin| {
            inject_helix_context_into_plugin_store(editor, compositor, plugin, |plugin| {
                plugin
                    .base_bindings
                    .call_initialize(&mut plugin.store)
                    .unwrap()
            })
        });
    }

    pub fn on_key_press(&mut self, editor: &mut Editor, compositor: &mut Compositor, c: char) {
        self.plugins.iter_mut().for_each(|plugin| {
            inject_helix_context_into_plugin_store(editor, compositor, plugin, |plugin| {
                if let Some(keyevents_bindings) = plugin.keyevents_bindings.as_mut() {
                    keyevents_bindings
                        .call_handle_key_press(&mut plugin.store, c)
                        .unwrap();
                }
            });
        });
    }
}

fn inject_helix_context_into_plugin_store<R>(
    editor: &mut Editor,
    compositor: &mut Compositor,
    plugin: &mut Plugin,
    f: impl FnOnce(&mut PluginInterface) -> R,
) -> R {
    TemporaryOwnedBorrowMut::provide_with_all((editor, compositor, &mut plugin.state), |cx_owned| {
        if std::mem::replace(
            &mut plugin.interface.store.data_mut().cx,
            Some(Cx {
                current_plugin: cx_owned.2,
                editor: cx_owned.0,
                compositor: cx_owned.1,
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
        ((cx.editor, cx.compositor, cx.current_plugin), return_value)
    })
}

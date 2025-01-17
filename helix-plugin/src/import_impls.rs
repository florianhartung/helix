use wasmtime::component::Linker;

use crate::{bindings, plugin_invocation_context::PluginInvocationCx, Base, Keyevents};

pub struct ImportImpls<'hx_ctx> {
    // TODO store this as `*mut PluginInvocationCx<'hx_ctx>`, so that unsafe code is safe, if transmute and pointer swap are done at the same time. use null pointer as default
    // Maybe also use pointers inside `PluginInvocationCx` so we can get rid of 'hx_ctx entirely and manage it ourself? Then we would no longer have to do the ugly transmute between lifetimes.
    pub(crate) helix_context: Option<PluginInvocationCx<'hx_ctx>>,

    pub(crate) keyevents_locked: bool,
}

impl ImportImpls<'static> {
    pub fn new() -> Self {
        Self {
            helix_context: None,
            keyevents_locked: true,
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

impl<'hx_ctx> ImportImpls<'hx_ctx> {
    pub fn expect_helix_context<'a>(&'a mut self) -> &'a mut PluginInvocationCx<'hx_ctx> {
        self.helix_context
            .as_mut()
            .expect("helix context to be present")
    }
}

impl bindings::helix::plugin::types::Host for ImportImpls<'_> {}

impl<'hx_ctx> bindings::KeyeventsImports for ImportImpls<'hx_ctx> {
    fn test(&mut self) {
        if self.keyevents_locked {
            // TODO handle this error differently?
            panic!("interface for optional world `keyevents` is not yet properly validated");
        }

        println!("keyevents::test called");
    }
}

impl<'hx_ctx> bindings::BaseImports for ImportImpls<'hx_ctx> {
    fn log(&mut self, log_level: bindings::LogLevel, msg: String) {
        match log_level {
            bindings::LogLevel::Info => println!("INFO: {msg}"),
            bindings::LogLevel::Warn => println!("WARN: {msg}"),
            bindings::LogLevel::Error => println!("ERROR: {msg}"),
        }
    }

    fn set_editor_status(&mut self, msg: String) {
        (self.expect_helix_context().set_editor_status)(msg);
    }
}

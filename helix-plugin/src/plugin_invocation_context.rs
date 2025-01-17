
/// The struct passed from `helix-term` to `helix-plugin` whenever the helix control flow is intercepted for plugin execution.
pub struct PluginInvocationCx<'a> {
    pub set_editor_status: &'a mut dyn FnMut(String),
}
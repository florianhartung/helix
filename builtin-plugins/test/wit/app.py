from hello_world import types, HelloWorld as IHelloWorld, set_editor_status, log

class HelloWorld(IHelloWorld):
    def get_metadata(self) -> types.PluginMetadata:
        return types.PluginMetadata("PY Plugin", "My description", ["python", "plugin"], types.RequestedWasiInterfaces.FOO)

    def initialize(self) -> None:
        log(types.LogLevel.INFO, "Initializing python plugin...")

    def handle_key_press(self, c: str) -> None:
        set_editor_status(f"Python got keycode {c}")
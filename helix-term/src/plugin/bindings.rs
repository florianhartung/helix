use wasmtime::component::bindgen;

bindgen!("base" in "src/plugin/wit/world.wit");

bindgen!("keyevents" in "src/plugin/wit/world.wit");

bindgen!("run-typed-commands" in "src/plugin/wit/world.wit");

bindgen!("editor-context" in "src/plugin/wit/world.wit");

impl From<helix_view::graphics::Rect> for Rect {
    fn from(value: helix_view::graphics::Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}
impl From<Rect> for helix_view::graphics::Rect {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

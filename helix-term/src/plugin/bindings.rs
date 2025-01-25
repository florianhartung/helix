use wasmtime::component::bindgen;

bindgen!("base" in "src/plugin/plugin.wit");

bindgen!("keyevents" in "src/plugin/plugin.wit");
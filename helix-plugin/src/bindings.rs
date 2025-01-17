use wasmtime::component::bindgen;

bindgen!("base" in "wit/plugin.wit");

bindgen!("keyevents" in "wit/plugin.wit");
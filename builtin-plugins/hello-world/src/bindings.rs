// Generated by `wit-bindgen` 0.36.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
pub type LogLevel = helix::plugin::types::LogLevel;
pub type PluginMetadata = helix::plugin::types::PluginMetadata;
#[allow(unused_unsafe, clippy::all)]
pub fn log(level: LogLevel, msg: &str) {
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "log"]
            fn wit_import(_: i32, _: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(level.clone() as i32, ptr0.cast_mut(), len0);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn set_editor_status(msg: &str) {
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "set-editor-status"]
            fn wit_import(_: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn test() {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "test"]
            fn wit_import();
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import() {
            unreachable!()
        }
        wit_import();
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_get_metadata_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::get_metadata();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let helix::plugin::types::PluginMetadata {
        name: name2,
        description: description2,
        keywords: keywords2,
    } = result0;
    let vec3 = (name2.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr1.add(4).cast::<usize>() = len3;
    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    let vec4 = (description2.into_bytes()).into_boxed_slice();
    let ptr4 = vec4.as_ptr().cast::<u8>();
    let len4 = vec4.len();
    ::core::mem::forget(vec4);
    *ptr1.add(12).cast::<usize>() = len4;
    *ptr1.add(8).cast::<*mut u8>() = ptr4.cast_mut();
    let vec6 = keywords2;
    let len6 = vec6.len();
    let layout6 = _rt::alloc::Layout::from_size_align_unchecked(vec6.len() * 8, 4);
    let result6 = if layout6.size() != 0 {
        let ptr = _rt::alloc::alloc(layout6).cast::<u8>();
        if ptr.is_null() {
            _rt::alloc::handle_alloc_error(layout6);
        }
        ptr
    } else {
        ::core::ptr::null_mut()
    };
    for (i, e) in vec6.into_iter().enumerate() {
        let base = result6.add(i * 8);
        {
            let vec5 = (e.into_bytes()).into_boxed_slice();
            let ptr5 = vec5.as_ptr().cast::<u8>();
            let len5 = vec5.len();
            ::core::mem::forget(vec5);
            *base.add(4).cast::<usize>() = len5;
            *base.add(0).cast::<*mut u8>() = ptr5.cast_mut();
        }
    }
    *ptr1.add(20).cast::<usize>() = len6;
    *ptr1.add(16).cast::<*mut u8>() = result6;
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_get_metadata<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
    let l2 = *arg0.add(8).cast::<*mut u8>();
    let l3 = *arg0.add(12).cast::<usize>();
    _rt::cabi_dealloc(l2, l3, 1);
    let l4 = *arg0.add(16).cast::<*mut u8>();
    let l5 = *arg0.add(20).cast::<usize>();
    let base8 = l4;
    let len8 = l5;
    for i in 0..len8 {
        let base = base8.add(i * 8);
        {
            let l6 = *base.add(0).cast::<*mut u8>();
            let l7 = *base.add(4).cast::<usize>();
            _rt::cabi_dealloc(l6, l7, 1);
        }
    }
    _rt::cabi_dealloc(base8, len8 * 8, 4);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_initialize_cabi<T: Guest>() {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    T::initialize();
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_handle_key_press_cabi<T: Guest>(arg0: i32) {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    T::handle_key_press(_rt::char_lift(arg0 as u32));
}
pub trait Guest {
    /// Every plugin has to provide some mandatory metadata, which may be displayed to the user.
    fn get_metadata() -> PluginMetadata;
    /// Basic interface functions
    /// TODO initialize only makes sense if we give it a helix context as a parameter
    fn initialize();
    fn handle_key_press(c: char);
}
#[doc(hidden)]
macro_rules! __export_world_hello_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "get-metadata"] unsafe extern "C" fn
        export_get_metadata() -> * mut u8 { $($path_to_types)*::
        _export_get_metadata_cabi::<$ty > () } #[export_name = "cabi_post_get-metadata"]
        unsafe extern "C" fn _post_return_get_metadata(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_get_metadata::<$ty > (arg0) } #[export_name =
        "initialize"] unsafe extern "C" fn export_initialize() { $($path_to_types)*::
        _export_initialize_cabi::<$ty > () } #[export_name = "handle-key-press"] unsafe
        extern "C" fn export_handle_key_press(arg0 : i32,) { $($path_to_types)*::
        _export_handle_key_press_cabi::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_hello_world_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 24]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 24]);
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod helix {
    pub mod plugin {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct PluginMetadata {
                pub name: _rt::String,
                pub description: _rt::String,
                pub keywords: _rt::Vec<_rt::String>,
            }
            impl ::core::fmt::Debug for PluginMetadata {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("PluginMetadata")
                        .field("name", &self.name)
                        .field("description", &self.description)
                        .field("keywords", &self.keywords)
                        .finish()
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum LogLevel {
                Info,
                Warn,
                Error,
            }
            impl ::core::fmt::Debug for LogLevel {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        LogLevel::Info => f.debug_tuple("LogLevel::Info").finish(),
                        LogLevel::Warn => f.debug_tuple("LogLevel::Warn").finish(),
                        LogLevel::Error => f.debug_tuple("LogLevel::Error").finish(),
                    }
                }
            }
            impl LogLevel {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> LogLevel {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => LogLevel::Info,
                        1 => LogLevel::Warn,
                        2 => LogLevel::Error,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_hello_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_hello_world_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_hello_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.36.0:component:hello-world:hello-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 497] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xef\x02\x01A\x02\x01\
A\x11\x01B\x05\x01ps\x01r\x03\x04names\x0bdescriptions\x08keywords\0\x04\0\x0fpl\
ugin-metadata\x03\0\x01\x01m\x03\x04info\x04warn\x05error\x04\0\x09log-level\x03\
\0\x03\x03\0\x12helix:plugin/types\x05\0\x02\x03\0\0\x09log-level\x03\0\x09log-l\
evel\x03\0\x01\x02\x03\0\0\x0fplugin-metadata\x03\0\x0fplugin-metadata\x03\0\x03\
\x01@\x02\x05level\x02\x03msgs\x01\0\x03\0\x03log\x01\x05\x01@\x01\x03msgs\x01\0\
\x03\0\x11set-editor-status\x01\x06\x01@\0\x01\0\x03\0\x04test\x01\x07\x01@\0\0\x04\
\x04\0\x0cget-metadata\x01\x08\x04\0\x0ainitialize\x01\x07\x01@\x01\x01ct\x01\0\x04\
\0\x10handle-key-press\x01\x09\x04\0!component:hello-world/hello-world\x04\0\x0b\
\x11\x01\0\x0bhello-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit\
-component\x070.220.0\x10wit-bindgen-rust\x060.36.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

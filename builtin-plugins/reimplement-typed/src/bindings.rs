pub type LogLevel = helix::plugin::types::LogLevel;
pub type PluginMetadata = helix::plugin::types::PluginMetadata;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
impl ::core::fmt::Debug for Rect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Rect")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct View {
    handle: _rt::Resource<View>,
}
impl View {
    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {
        Self {
            handle: _rt::Resource::from_handle(handle),
        }
    }
    #[doc(hidden)]
    pub fn take_handle(&self) -> u32 {
        _rt::Resource::take_handle(&self.handle)
    }
    #[doc(hidden)]
    pub fn handle(&self) -> u32 {
        _rt::Resource::handle(&self.handle)
    }
}
unsafe impl _rt::WasmResource for View {
    #[inline]
    unsafe fn drop(_handle: u32) {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();
        #[cfg(target_arch = "wasm32")]
        {
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[resource-drop]view"]
                fn drop(_: u32);
            }
            drop(_handle);
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
/// Logs some message in the helix log file.
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
/// Logs some message in the helix log file.
pub fn get_text_selection() -> Option<_rt::String> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-text-selection"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
        match l1 {
            0 => None,
            1 => {
                let e = {
                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                    let l3 = *ptr0.add(8).cast::<usize>();
                    let len4 = l3;
                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                    _rt::string_lift(bytes4)
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
/// --- Basic Functions ---
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
impl View {
    #[allow(unused_unsafe, clippy::all)]
    /// get-id: func() -> view-id;
    pub fn get_area(&self) -> Rect {
        unsafe {
            #[repr(align(2))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "$root")]
            extern "C" {
                #[link_name = "[method]view.get-area"]
                fn wit_import(_: i32, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: *mut u8) {
                unreachable!()
            }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*ptr0.add(0).cast::<u16>());
            let l2 = i32::from(*ptr0.add(2).cast::<u16>());
            let l3 = i32::from(*ptr0.add(4).cast::<u16>());
            let l4 = i32::from(*ptr0.add(6).cast::<u16>());
            Rect {
                x: l1 as u16,
                y: l2 as u16,
                width: l3 as u16,
                height: l4 as u16,
            }
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_focus() -> View {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-focus"]
            fn wit_import() -> i32;
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import() -> i32 {
            unreachable!()
        }
        let ret = wit_import();
        View::from_handle(ret as u32)
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn remove_view(view: &View) {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "remove-view"]
            fn wit_import(_: i32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            unreachable!()
        }
        wit_import((view).handle() as i32);
    }
}
#[allow(unused_unsafe, clippy::all)]
/// get-all-views: func() -> list<view>;
pub fn close() {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "close"]
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
        requested_wasi_interfaces: requested_wasi_interfaces2,
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
    let flags7 = requested_wasi_interfaces2;
    *ptr1.add(24).cast::<u8>() = ((flags7.bits() >> 0) as i32) as u8;
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
    /// This is the only function from which a plugin may not call imported functions.
    fn get_metadata() -> PluginMetadata;
    /// This function is called directly after the editor state is initialized.
    /// It allows plugins to make changes to the editor before it is rendered for the first time.
    fn initialize();
    fn handle_key_press(c: char);
}
#[doc(hidden)]
macro_rules! __export_world_reimplement_typed_cabi {
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
pub(crate) use __export_world_reimplement_typed_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 28]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 28]);
#[allow(dead_code)]
pub mod helix {
    #[allow(dead_code)]
    pub mod plugin {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct RequestedWasiInterfaces : u8 { const HTTP = 1 << 0; const FOO = 1
                << 1; const BAR = 1 << 2; const BAZ = 1 << 3; const BLA = 1 << 4; }
            }
            #[derive(Clone)]
            pub struct PluginMetadata {
                pub name: _rt::String,
                pub description: _rt::String,
                pub keywords: _rt::Vec<_rt::String>,
                /// TODO remove and add permissions field
                pub requested_wasi_interfaces: RequestedWasiInterfaces,
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
                        .field(
                            "requested-wasi-interfaces",
                            &self.requested_wasi_interfaces,
                        )
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
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
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
macro_rules! __export_reimplement_typed_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_reimplement_typed_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_reimplement_typed_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:reimplement-typed:reimplement-typed:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 761] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf1\x04\x01A\x02\x01\
A\x1f\x01B\x07\x01n\x05\x04http\x03foo\x03bar\x03baz\x03bla\x04\0\x19requested-w\
asi-interfaces\x03\0\0\x01ps\x01r\x04\x04names\x0bdescriptions\x08keywords\x02\x19\
requested-wasi-interfaces\x01\x04\0\x0fplugin-metadata\x03\0\x03\x01m\x03\x04inf\
o\x04warn\x05error\x04\0\x09log-level\x03\0\x05\x03\x01\x12helix:plugin/types\x05\
\0\x02\x03\0\0\x09log-level\x03\0\x09log-level\x03\0\x01\x02\x03\0\0\x0fplugin-m\
etadata\x03\0\x0fplugin-metadata\x03\0\x03\x01@\x02\x05level\x02\x03msgs\x01\0\x03\
\0\x03log\x01\x05\x01ks\x01@\0\0\x06\x03\0\x12get-text-selection\x01\x07\x01@\x01\
\x03msgs\x01\0\x03\0\x11set-editor-status\x01\x08\x01r\x04\x01x{\x01y{\x05width{\
\x06height{\x03\0\x04rect\x03\0\x09\x03\0\x04view\x03\x01\x01i\x0b\x01@\0\0\x0c\x03\
\0\x09get-focus\x01\x0d\x01h\x0b\x01@\x01\x04view\x0e\x01\0\x03\0\x0bremove-view\
\x01\x0f\x01@\0\x01\0\x03\0\x05close\x01\x10\x01@\x01\x04self\x0e\0\x0a\x03\0\x15\
[method]view.get-area\x01\x11\x01@\0\0\x04\x04\0\x0cget-metadata\x01\x12\x04\0\x0a\
initialize\x01\x10\x01@\x01\x01ct\x01\0\x04\0\x10handle-key-press\x01\x13\x04\x01\
-component:reimplement-typed/reimplement-typed\x04\0\x0b\x17\x01\0\x11reimplemen\
t-typed\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.21\
6.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

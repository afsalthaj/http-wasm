// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod golem {
        #[allow(dead_code)]
        pub mod demo {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub enum Actions {
                    Register(_rt::String),
                    Follow(_rt::String),
                    Unfollow(_rt::String),
                    PostTweet(_rt::String),
                }
                impl ::core::fmt::Debug for Actions {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Actions::Register(e) => {
                                f.debug_tuple("Actions::Register").field(e).finish()
                            }
                            Actions::Follow(e) => {
                                f.debug_tuple("Actions::Follow").field(e).finish()
                            }
                            Actions::Unfollow(e) => {
                                f.debug_tuple("Actions::Unfollow").field(e).finish()
                            }
                            Actions::PostTweet(e) => {
                                f.debug_tuple("Actions::PostTweet").field(e).finish()
                            }
                        }
                    }
                }
                #[derive(Clone)]
                pub enum CustomResult {
                    Success(_rt::String),
                    Failure(_rt::String),
                }
                impl ::core::fmt::Debug for CustomResult {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            CustomResult::Success(e) => {
                                f.debug_tuple("CustomResult::Success").field(e).finish()
                            }
                            CustomResult::Failure(e) => {
                                f.debug_tuple("CustomResult::Failure").field(e).finish()
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    T::add(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let v4 = match arg0 {
                        0 => {
                            let e4 = {
                                let len0 = arg2;
                                let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);

                                _rt::string_lift(bytes0)
                            };
                            Actions::Register(e4)
                        }
                        1 => {
                            let e4 = {
                                let len1 = arg2;
                                let bytes1 = _rt::Vec::from_raw_parts(arg1.cast(), len1, len1);

                                _rt::string_lift(bytes1)
                            };
                            Actions::Follow(e4)
                        }
                        2 => {
                            let e4 = {
                                let len2 = arg2;
                                let bytes2 = _rt::Vec::from_raw_parts(arg1.cast(), len2, len2);

                                _rt::string_lift(bytes2)
                            };
                            Actions::Unfollow(e4)
                        }
                        n => {
                            debug_assert_eq!(n, 3, "invalid enum discriminant");
                            let e4 = {
                                let len3 = arg2;
                                let bytes3 = _rt::Vec::from_raw_parts(arg1.cast(), len3, len3);

                                _rt::string_lift(bytes3)
                            };
                            Actions::PostTweet(e4)
                        }
                    };
                    let len5 = arg4;
                    let bytes5 = _rt::Vec::from_raw_parts(arg3.cast(), len5, len5);
                    let result6 = T::get(v4, _rt::string_lift(bytes5));
                    let ptr7 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result6 {
                        CustomResult::Success(e) => {
                            *ptr7.add(0).cast::<u8>() = (0i32) as u8;
                            let vec8 = (e.into_bytes()).into_boxed_slice();
                            let ptr8 = vec8.as_ptr().cast::<u8>();
                            let len8 = vec8.len();
                            ::core::mem::forget(vec8);
                            *ptr7.add(8).cast::<usize>() = len8;
                            *ptr7.add(4).cast::<*mut u8>() = ptr8.cast_mut();
                        }
                        CustomResult::Failure(e) => {
                            *ptr7.add(0).cast::<u8>() = (1i32) as u8;
                            let vec9 = (e.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr7.add(8).cast::<usize>() = len9;
                            *ptr7.add(4).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    }
                    ptr7
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_remote_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::remote(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_remote<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn add(value: u64);
                    fn get(actions: Actions, user_id: _rt::String) -> CustomResult;
                    fn remote(input: _rt::String) -> Result<_rt::String, _rt::String>;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_demo_api_cabi{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

      #[export_name = "golem:demo/api#add"]
      unsafe extern "C" fn export_add(arg0: i64,) {
        $($path_to_types)*::_export_add_cabi::<$ty>(arg0)
      }
      #[export_name = "golem:demo/api#get"]
      unsafe extern "C" fn export_get(arg0: i32,arg1: *mut u8,arg2: usize,arg3: *mut u8,arg4: usize,) -> *mut u8 {
        $($path_to_types)*::_export_get_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4)
      }
      #[export_name = "cabi_post_golem:demo/api#get"]
      unsafe extern "C" fn _post_return_get(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_get::<$ty>(arg0)
      }
      #[export_name = "golem:demo/api#remote"]
      unsafe extern "C" fn export_remote(arg0: *mut u8,arg1: usize,) -> *mut u8 {
        $($path_to_types)*::_export_remote_cabi::<$ty>(arg0, arg1)
      }
      #[export_name = "cabi_post_golem:demo/api#remote"]
      unsafe extern "C" fn _post_return_remote(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_remote::<$ty>(arg0)
      }
    };);
  }
                #[doc(hidden)]
                pub(crate) use __export_golem_demo_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::golem::demo::api::__export_golem_demo_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::demo::api);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 365] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xef\x01\x01A\x02\x01\
A\x02\x01B\x0b\x01q\x04\x08register\x01s\0\x06follow\x01s\0\x08unfollow\x01s\0\x0a\
post-tweet\x01s\0\x04\0\x07actions\x03\0\0\x01q\x02\x07success\x01s\0\x07failure\
\x01s\0\x04\0\x0dcustom-result\x03\0\x02\x01@\x01\x05valuew\x01\0\x04\0\x03add\x01\
\x04\x01@\x02\x07actions\x01\x07user-ids\0\x03\x04\0\x03get\x01\x05\x01j\x01s\x01\
s\x01@\x01\x05inputs\0\x06\x04\0\x06remote\x01\x07\x04\x01\x0egolem:demo/api\x05\
\0\x04\x01\x12golem:demo/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09pr\
oducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x06\
0.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

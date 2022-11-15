use crate::manage::resource::*;

use ::std::{cell::RefCell, ptr::NonNull, *};

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum e_rust_status {
    RUST_OK = 0,
    RUST_ERR_NULL_POINTER,
    RUST_ERR_PANICKED,
}

//use self::e_rust_status::*;

#[macro_export]
macro_rules! ffi_panic_boundary {($($tt:tt)*) => (
    // /* or */ { ::scopeguard::defer_on_unwind(process::abort()); $($tt)* }
    match ::std::panic::catch_unwind(|| {$($tt)*}) {//
        | Ok(ret) => ret,
        | Err(_) => return e_rust_status::RUST_ERR_PANICKED,
        // {
        //     // eprintln!("Rust panicked; aborting process");
        //     // ::std::process::abort()
        // },
    }
)}

#[macro_export]
macro_rules! unwrap_pointer {($pointer:expr) => (
    match $pointer {//
        | Some(non_null_pointer) => non_null_pointer,
        | None => return e_rust_status::RUST_ERR_NULL_POINTER,
    }
)}

// #[no_mangle]
// pub extern "C" fn register_cb(cb: Option<Callback>, arg: Option<Arg>) -> e_rust_status {
//     ffi_panic_boundary! {
//         let cb = unwrap_pointer!(cb);
//         CALLBACKS.with(|slf| { slf
//             .borrow_mut()
//             .push((cb, arg))
//         });
//         RUST_OK
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn call_cbs() -> e_rust_status {
//     ffi_panic_boundary! {
//         CALLBACKS.with(|slf| { slf
//             .borrow_mut()
//             .iter_mut()
//             .for_each(|&mut (cb, arg): &mut (Callback, Option<Arg>)| {
//                 cb(arg);
//             })
//         });
//         RUST_OK
//     }
// }

// #[no_mangle]
// pub extern "C" fn clear_cbs() -> e_rust_status {
//     ffi_panic_boundary! {
//         CALLBACKS.with(|slf| {
//             let mut cbs = slf.borrow_mut();
//             cbs.clear();
//             cbs.shrink_to_fit();
//         });
//         RUST_OK
//     }
// }

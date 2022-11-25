use crate::{ffi_panic_boundary, manage, unwrap_pointer};
use manage::ffi_util::e_rust_status;

use libc::{c_char, c_void};
use tokio::runtime::{self, Runtime};

pub fn set_callback_function(
    callback_get_all_data: manage::resource::cb_get_all_data,
    callback_get_data: manage::resource::cb_get_data,
) {
    manage::resource::CALLBACKS_GET_ALL_DATA.with(|slf| {
        *slf.borrow_mut() = Some(Box::new(callback_get_all_data));
    });
    manage::resource::CALLBACKS_GET_DATA.with(|slf| {
        *slf.borrow_mut() = Some(Box::new(callback_get_data));
    });

    println!("Set CallBack Function");
}

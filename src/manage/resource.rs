#[allow(non_camel_case_types)]
#[repr(C)]
pub enum SERVER_STATUS {
    R_SERVER_STOP = 0,
    R_SERVER_BOOT,
    R_SERVER_RUN,
}

use std::{cell::RefCell, ffi::c_char};

pub type cb_get_all_data = unsafe extern "C" fn(i32) -> *mut c_char;
pub type cb_get_data = unsafe extern "C" fn(i32, i32) -> *mut c_char;

thread_local! {
    static SERVER_INSTANCE : RefCell<Option<Box<rocket::Rocket<rocket::Ignite>>>> = RefCell::new(None);
    pub static CALLBACKS_GET_ALL_DATA : RefCell<Option<Box<cb_get_all_data>>> = RefCell::new(None);
    pub static CALLBACKS_GET_DATA : RefCell<Option<Box<cb_get_data>>> = RefCell::new(None);
}

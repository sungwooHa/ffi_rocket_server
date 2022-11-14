use ::libc::c_void; // libc = "0.2.51"

pub type Arg = NonNull<c_void>;
pub type Callback = unsafe extern "C" fn(mb_arg: Option<Arg>);

use ::std::{cell::RefCell, ptr::NonNull, *};

thread_local! {
    pub static CALLBACKS : RefCell< Vec< (Callback, Option<Arg>) > > = RefCell::new(Vec::new());
    pub static SERVER_INSTANCE : RefCell<Option<Box<rocket::Rocket<rocket::Ignite>>>> = RefCell::new(None);
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum SERVER_STATUS {
    R_SERVER_STOP = 0,
    R_SERVER_BOOT,
    R_SERVER_RUN,
}

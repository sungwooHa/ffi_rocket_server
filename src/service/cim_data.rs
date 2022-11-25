use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};

pub fn get_all_data(data_type: i32) -> Option<String> {
    Some(crate::manage::resource::CALLBACKS_GET_ALL_DATA.with(|slf| unsafe {
        let data: *mut c_char = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type);
        String::from_utf8_lossy(CStr::from_ptr(data).to_bytes()).to_string()
    }))
}

pub fn get_data(data_type: i32, key: i32) -> Option<String> {
    Some(crate::manage::resource::CALLBACKS_GET_DATA.with(|slf| unsafe {
        let data: *mut c_char = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type, key);
        String::from_utf8_lossy(CStr::from_ptr(data).to_bytes()).to_string()
    }))
}

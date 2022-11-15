use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json, Json};

use libc::c_char;
use std::ffi::{CStr, CString};

use crate::model::response::{self, Response};

#[get("/")]
pub fn hello() -> &'static str {
    "Hello. This is CIM REST api"
}

#[no_mangle]
#[allow(non_snake_case)]
#[get("/<data_type>")]
pub fn get_cim_data_all(data_type: i32) -> status::Custom<Json<Response>> {
    let responseData = crate::CALLBACKS_GET_ALL_DATA.with(|slf| unsafe {
        let data: *mut c_char = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type);
        CStr::from_ptr(data)
    });

    status::Custom(
        Status::from_code(404).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::to_value(responseData.to_str().unwrap()).unwrap(),
        }),
    )
}

#[allow(non_snake_case)]
#[get("/<data_type>/<key>")]
pub fn get_cim_data(data_type: i32, key: i32) -> status::Custom<Json<Response>> {
    let responseData = crate::CALLBACKS_GET_DATA.with(|slf| unsafe {
        let data: *mut c_char = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type, key);
        CStr::from_ptr(data)
    });

    status::Custom(
        Status::from_code(404).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::to_value(responseData.to_str().unwrap()).unwrap(),
        }),
    )
}

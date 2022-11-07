use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json, Json};

use std::borrow::BorrowMut;
use std::ffi::{CString, CStr};
use libc::c_char;

use crate::model::response::{Response, self};

#[get("/")]
pub fn hello() -> &'static str {
    "Hello. This is CIM REST api"
}

#[no_mangle]
#[allow(non_snake_case)]
#[get("/<data_type>")]
pub fn get_cim_data_all(data_type: i32) -> status::Custom<Json<Response>> {
    let responseData = crate::CALLBACKS.with(|slf|
        unsafe{
            match slf.borrow_mut().as_ref() {
                Some(cb_func) => {
                    CString::from_raw(cb_func.as_ref()(data_type))
                },
                None =>{
                    CString::new("can't find cb function instance").expect("CString::new failed")
                }
            }
        }
    );

    //let data : *mut c_char = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type);
        //CString::from_raw(data)
    status::Custom(
        Status::from_code(404).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data : serde_json::to_value( 
                match responseData.into_string() {
                    Ok(data) => data,
                    Err(err) => err.to_string(),
            }).unwrap(),
        }),
    )
}

#[allow(non_snake_case)]
#[get("/<data_type>/<key>")]
pub fn get_cim_data(data_type: &str, key: u32) -> status::Custom<Json<Response>> {
    status::Custom(
        Status::from_code(404).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::to_value("data test").unwrap(),
        }),
    )
}

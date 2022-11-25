use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{serde_json, Json};

use libc::c_char;
use std::ffi::{CStr, CString};

use crate::model::response::{self, Response};
use crate::service::*;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello. This is CIM REST api"
}

#[no_mangle]
#[allow(non_snake_case)]
#[get("/<data_type>")]
pub fn get_cim_data_all(data_type: i32) -> status::Custom<Json<Response>> {
    let Some(responseData) = cim_data::get_all_data(data_type)
    else {
        return status::Custom(
            Status::from_code(404).unwrap(),
            Json(Response {
                message : format!("Fail to get data"),
                data : serde_json::to_value("").unwrap(),
            })
        )
    };

    status::Custom(
        Status::from_code(200).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::from_str(responseData.as_str()).unwrap(),
        }),
    )
}

#[allow(non_snake_case)]
#[get("/<data_type>/<key>")]
pub fn get_cim_data(data_type: i32, key: i32) -> status::Custom<Json<Response>> {
    let Some(responseData) = cim_data::get_data(data_type, key)
    else {
        return status::Custom(
            Status::from_code(404).unwrap(),
            Json(Response {
                message : format!("Fail to get data"),
                data : serde_json::to_value("").unwrap(),
            })
        )
    };

    status::Custom(
        Status::from_code(200).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::from_str(responseData.as_str()).unwrap(),
        }),
    )
}

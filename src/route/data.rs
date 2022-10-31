use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json, Json};

use crate::model::response::Response;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello. This is CIM REST api"
}

#[no_mangle]
#[allow(non_snake_case)]
#[get("/<data_type>")]
pub fn get_cim_data_all(data_type: i32) -> status::Custom<Json<Response>> {
    let mut response_data: i32 = 0;
    crate::CALLBACKS.with(|slf| unsafe {
        let data = slf.borrow_mut().as_ref().unwrap().as_ref()(data_type);
        response_data = data;
    });
    status::Custom(
        Status::from_code(404).unwrap(),
        Json(Response {
            message: format!("message test, request : {}", data_type),
            data: serde_json::to_value(response_data).unwrap(),
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

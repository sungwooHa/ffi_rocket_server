#[macro_use]
extern crate rocket;

mod extern_function;
mod manage;
mod model;
mod route;
mod service;

use rocket::Build;

pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount(
            "/", //base
            routes![route::basic::hello, route::basic::shutdown,],
        )
        .mount(
            "/data", //get "$DATA_TYPE"
            routes![route::data::get_cim_data_all, route::data::get_cim_data,],
        )
        .register(
            "/",
            catchers![
                route::error::not_found,
                route::error::internal_error,
                route::error::unprocessable_entity,
            ],
        )
}

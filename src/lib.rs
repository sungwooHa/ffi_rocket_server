#[macro_use]
extern crate rocket;

mod manage;
mod model;
mod route;

use std::{
    borrow::Borrow,
    cell::{RefCell, RefMut},
    f32::consts::E,
    sync::atomic::AtomicBool,
};

use libc::{c_char, c_void};
use manage::{ffi_util::e_rust_status, resource::*};

use futures::{executor::block_on, TryFutureExt};
use rocket::{fairing::AdHoc, Build, Ignite, Rocket};

use tokio::{
    //prelude::*,
    runtime::{self, Runtime},
    sync::oneshot,
    //timer::{Delay, Interval},
};

use warp::reply;

thread_local! {
    static SERVER_INSTANCE : RefCell<Option<Box<rocket::Rocket<rocket::Ignite>>>> = RefCell::new(None);
    pub static CALLBACKS_GET_ALL_DATA : RefCell<Option<Box<cb_get_all_data>>> = RefCell::new(None);
    pub static CALLBACKS_GET_DATA : RefCell<Option<Box<cb_get_data>>> = RefCell::new(None);
}

type cb_get_all_data = unsafe extern "C" fn(i32) -> *mut c_char;
type cb_get_data = unsafe extern "C" fn(i32, i32) -> *mut c_char;

#[no_mangle]
pub struct ServerManager {
    pub server_thread: tokio::runtime::Runtime,
}

pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount(
            "/", //base
            routes![route::manage::hello, route::manage::shutdown,],
        )
        .mount(
            "/data", //get "$DATA_TYPE"
            routes![route::data::get_cim_data_all,],
        )
}

#[no_mangle]
pub extern "C" fn server_run(
    callback_get_all_data: cb_get_all_data,
    callback_get_data: cb_get_data,
) -> e_rust_status {
    ffi_panic_boundary! {

        // CALLBACKS_GET_ALL_DATA.with(|slf| {
        //     *slf.borrow_mut() = Some(Box::new(callback_get_all_data));
        // });

        CALLBACKS_GET_DATA.with(|slf| {
            *slf.borrow_mut() = Some(Box::new(callback_get_data));
        });

        let server_instance = ServerManager {
            server_thread :
                tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .on_thread_start(|| {
                    println!("thread start!");
                })
                .on_thread_stop(|| {
                    println!("thread stop!");
                })
                .build()
                .unwrap()
        };

        server_instance.server_thread.block_on(async {
            rocket()
            .attach(AdHoc::on_liftoff("launch CIM ROCKET", move |_rocket| Box::pin(async move {
                CALLBACKS_GET_ALL_DATA.with(|slf| {
                         *slf.borrow_mut() = Some(Box::new(callback_get_all_data));
                        });
                println!("Rocket has lifted off!");
            })))
            // .attach(AdHoc::on_liftoff("launch CIM ROCKET", |_rocket| Box::pin(async move {
            //     println!("Rocket has lifted off!");
            // })))
            .launch().await.expect("Failt to start server");
        });

        e_rust_status::RUST_OK
    }
}

#[no_mangle]
pub extern "C" fn server_shutdown() -> e_rust_status {
    ffi_panic_boundary! {
        let result = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
            let client = reqwest::Client::new();

            match client.get("http://127.0.0.1:8000/shutdown").send().await{
                Ok(result) => {
                    match result.text().await {
                        Ok(body) =>{
                            println!("{}", body);
                        }
                        Err(msg) => {
                            println!("{}", msg);
                        }
                    }
                }
                Err(msg) => {
                    println!("{}", msg);
                }
            }
        });
        //let result = reqwest::blocking::get("127.0.0.1:8000/shutdown").unwrap().text().unwrap();

        e_rust_status::RUST_OK
    }
}

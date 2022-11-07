#[macro_use]
extern crate rocket;

mod manage;
mod model;
mod route;

use std::{
    borrow::Borrow,
    cell::{RefCell, RefMut}, sync::atomic::AtomicBool,
};

use libc::{c_void, c_char};
use manage::{resource::*, ffi_util::e_rust_status};

use futures::executor::block_on;
use rocket::{Build, Ignite, Rocket};

use tokio::{
    //prelude::*,
    runtime::Runtime,
    sync::oneshot,
    //timer::{Delay, Interval},
};

use warp::reply;

struct ServerState {
    is_run : AtomicBool
}

pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .manage({
            SERVER_STATE.with(|slf| {
                *slf.borrow_mut() = Some(Box::new(ServerState { is_run : AtomicBool::new(true)}));
            });
        })
        .mount(
            "/", //base
            routes![
                route::manage::hello,
                route::manage::shutdown,
                ],
        )
        .mount(
            "/data", //get "$DATA_TYPE"
            routes![route::data::get_cim_data_all,],
        )
}

thread_local! {
    static SERVER_INSTANCE : RefCell<Option<Box<rocket::Rocket<rocket::Ignite>>>> = RefCell::new(None);
    //pub static CALLBACKS : RefCell< Option<Box<Callback>>> = RefCell::new(None);
    pub static CALLBACKS : RefCell<Option<Box<CB_GetAllData>>> = RefCell::new(None);
    static SERVER_STATE : RefCell<Option<Box<ServerState>>> = RefCell::new(None);
}

type Callback = unsafe extern "C" fn(i32) -> i32;
type CB_GetAllData = unsafe extern "C" fn(i32) -> *mut c_char;

#[no_mangle]
pub extern "C" fn rocket_state() -> SERVER_STATUS {
    SERVER_STATE.with(|slf|{
        match slf.borrow().as_ref(){
                    Some(server_state) => {
                        if server_state.as_ref().is_run.load(std::sync::atomic::Ordering::Relaxed) {
                            SERVER_STATUS::R_SERVER_RUN
                        } else {
                            SERVER_STATUS::R_SERVER_STOP
                        }
                    }
                    None => {
                        SERVER_STATUS::R_SERVER_STOP
                    }
                }
    })
}

#[no_mangle]
pub extern "C" fn rocket_starter(call_back_print: CB_GetAllData) -> e_rust_status {
    ffi_panic_boundary! {
        CALLBACKS.with(|slf| {
            *slf.borrow_mut() = Some(Box::new(call_back_print));
        });

        let tokio_thread = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .on_thread_start(|| {
                println!("thread start!");
            })
            .on_thread_stop(|| {
                println!("thread stop!");
            })
            .build()
            .unwrap();

        SERVER_INSTANCE.with(|slf| {
            *slf.borrow_mut() = 
                Some(
                    Box::new(
                        tokio_thread.block_on(async {
                            rocket().launch().await.expect("Failt to start server")
                        }
                    )
                ));
            });
            
        e_rust_status::RUST_OK
    }
}

//it needs gracful shutdown.
#[no_mangle]
pub extern "C" fn server_killer() -> e_rust_status {
    ffi_panic_boundary! {
        SERVER_INSTANCE.with(|slf|{
            match slf.borrow().as_ref(){
                        Some(server) => {
                            server.shutdown().notify();
                        }
                        None => {
                            println!("there is no server instance");
                        }
                    }
        });
        e_rust_status::RUST_OK
        // let instance = tokio::runtime::Builder::new_current_thread().enable_all()
        //     .on_thread_start(|| {
        //         println!("server start!");
        //     })
        //     .on_thread_stop(|| {
        //         println!("server stop!");
        //     })
        //     .build()
        //     .unwrap();

        // instance.block_on(
        //     async{
        //         let rocket = rocket().ignite().await.expect("failt to ingnite rocket");
        //                 let shutdown_handle = rocket.shutdown();
        //                 rocket::tokio::spawn(rocket.launch());
        //                 shutdown_handle.notify();
        //     }
        // );

        // SERVER_INSTANCE.with(|slf| {
        //     //let slf = slf.borrow();
        //     //let slf = slf.as_ref().unwrap();
        //     match slf.borrow().as_ref(){
        //         Some(server) => {
        //             server.as_ref().block_on(async{
        //                 let rocket = rocket().ignite().await.expect("failt to ingnite rocket");
        //                 let shutdown_handle = rocket.shutdown();
        //                 rocket::tokio::spawn(rocket.launch());
        //                 shutdown_handle.notify();
        //             })
        //         }
        //         None => {
        //             println!("there is no server instance");
        //         }
        //     }
        // });
    }
}

// pub async fn make_server_instance() {
//     let hello_world = warp::path::end().map(|| "Hello, World at root!");

//     let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

//     let killer = warp::path!("kill").map(|| {
//         server_killer();
//         warp::reply::json(&"ok")
//     });

//     let routes = warp::get().and(hello_world.or(hello).or(killer));

//     let s = warp::serve(routes).run(([127, 0, 0, 1], 3030));

//     s.await
// }

// #[no_mangle]
// pub extern "C" fn server_starter() {
//     let instance = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .on_thread_start(|| {
//             println!("server start!");
//         })
//         .on_thread_stop(|| {
//             println!("server stop!");
//         })
//         .build()
//         .unwrap();

//     instance.block_on(async { make_server_instance().await });

//     SERVER_INSTANCE.with(|slf| {
//         *slf.borrow_mut() = Some(Box::new(instance));
//     })
// }

// #[no_mangle]
// pub extern "C" fn server_killer() {
//     SERVER_INSTANCE.with(|slf| {
//         //let slf = slf.borrow();
//         //let slf = slf.as_ref().unwrap();
//         drop(slf.borrow().as_ref().unwrap())
//     })
// }

// #[no_mangle]
// pub extern "C" fn hello_world() {
//     println!("Hello World!");
// }

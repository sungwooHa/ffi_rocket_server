use rocket::fairing::AdHoc;

use crate::{
    manage::{ffi_util::*, resource::*},
    *,
};

#[no_mangle]
pub struct ServerManager {
    pub server_thread: tokio::runtime::Runtime,
}

#[no_mangle]
pub extern "C" fn server_run(
    callback_get_all_data: Option<cb_get_all_data>,
    callback_get_data: Option<cb_get_data>,
) -> e_rust_status {
    ffi_panic_boundary! {

        //function check.
        let callback_get_all_data = unwrap_pointer!(callback_get_all_data);
        let callback_get_data = unwrap_pointer!(callback_get_data);

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
            crate::rocket()
            .attach(AdHoc::on_liftoff("launch CIM ROCKET", move |_| Box::pin(async move {
                manage::handler::set_callback_function(callback_get_all_data, callback_get_data);
            })))
            .launch().await.expect("Fail to start server");
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
        e_rust_status::RUST_OK
    }
}

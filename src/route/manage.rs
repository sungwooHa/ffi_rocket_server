use rocket::Shutdown;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello. This is CIM REST api"
}

#[get("/shutdown")]
pub fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}

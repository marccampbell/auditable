extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate rustc_serialize;
extern crate persistent;
extern crate redis;
extern crate uuid;
extern crate time;

mod auditable;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let server = auditable::server::new_server();
    server.start();
}

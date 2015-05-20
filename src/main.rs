extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate rustc_serialize;
extern crate persistent;

mod auditable;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut server = auditable::server::new_server();
    server.start();
}

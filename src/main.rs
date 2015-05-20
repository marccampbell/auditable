extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate rustc_serialize;

mod auditable { pub mod server; }

use auditable::{server};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut server = server::Server::new();
    server.start();
}

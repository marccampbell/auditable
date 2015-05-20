extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate rustc_serialize;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use router::Router;
use bodyparser::*;
use auditable::auditevent;
use auditable::auditevent::AuditEvent;

pub struct Server;

fn ping(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "pong")))
}

pub fn new_server() -> Server {
    Server
}


impl Server {

    pub fn start(&self) {
        let mut router = Router::new();

        router.get("/ping", ping);

        router.post("/v1/event", auditevent::auditevent_create);

        Iron::new(router).http("0.0.0.0:3000").unwrap();
    }
}

extern crate persistent;

use iron::prelude::*;
use iron::status;
use router::Router;
use bodyparser::*;

pub struct Server;

#[derive(Debug,Clone,RustcDecodable)]
struct AuditEvent {
    action: String,
    actor: Option<String>,
    created_at: i64,
}


pub fn auditevent_create(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<Raw>();
    match body {
        Ok(Some(body)) => println!("Received audit event: {}", body),
        Ok(None) => { ; },
        Err(err) => println!("Error: {:?}", err)
    }

    let audit_event = req.get::<Struct<AuditEvent>>();
    match audit_event {
        Ok(Some(audit_event)) => println!("Parsed body:\n{:?}", audit_event),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with(status::Ok))
}

fn ping(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "pong")))
}

impl Server {

    pub fn new() -> Server {
        Server
    }

    pub fn start(&self) {
        let mut router = Router::new();

        router.get("/ping", ping);

        router.post("/v1/event", auditevent_create);

        Iron::new(router).http("0.0.0.0:3000").unwrap();
    }
}

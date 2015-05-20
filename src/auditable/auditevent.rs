use iron::prelude::*;
use iron::status;
use router::Router;
use bodyparser::*;

#[derive(Debug,Clone,RustcDecodable)]
pub struct AuditEvent {
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
        Ok(Some(audit_event)) =>
          audit_event.create(),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with(status::Ok))
}

impl AuditEvent {

    pub fn create(&self) {
        println!("creating");
    }
}

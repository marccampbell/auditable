extern crate redis;
extern crate time;

use iron::prelude::*;
use iron::status;
use bodyparser::*;
use redis::Commands;
use std::env;
use uuid::Uuid;

pub fn auditevent_create(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<Raw>();
    match body {
        Ok(Some(body)) => println!("Received audit event: {}", body),
        Ok(None) => { ; },
        Err(err) => println!("Error: {:?}", err)
    }

    let audit_event = req.get::<Struct<AuditEvent>>();
    match audit_event {
        Ok(Some(audit_event)) => audit_event.create(),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with(status::Ok))
}

#[derive(Debug,Clone,RustcDecodable)]
pub struct AuditEvent {
    action: String,
    actor: String,
    created_at: String,
    description: Option<String>,
}

impl AuditEvent {
    pub fn create(&self) {
        let event_id = Uuid::new_v4().to_string();
        let now = time::now_utc().to_timespec();

        let redis_uri = env::var_os("REDIS_URI").unwrap();
        let client = redis::Client::open(redis_uri.to_str().unwrap());
        let connect_result = client.unwrap().get_connection().unwrap();

        redis::cmd("HMSET").arg(event_id.clone()).arg(&[
            ("action", self.action.clone()),
            ("actor", self.actor.clone()),
            ("created.at", now.sec.to_string())
        ]).execute(&connect_result);

        redis::cmd("ZADD").arg("audit.events").arg(now.sec).arg(event_id.clone()).execute(&connect_result);
    }
}

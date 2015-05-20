extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

#[test]
fn test_ping() {
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://localhost:3000")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
    assert!(true);
}

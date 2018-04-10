#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate tokio_core;
extern crate http;
extern crate reqwest;
extern crate yup_oauth2;

extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate router;

use std::env;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::Read;

pub mod firebase;

use firebase::auth::*;

//struct ComicServer {
//    firebase_db: Fireb
//}

fn all_feeds(_req: &mut Request) -> IronResult<Response> {
    println!("Requested: {:?}", _req);
    let mut buffer= String::new();
    _req.body.read_to_string(&mut buffer).expect("Unable to read body");
    println!("Request body: {}", buffer);
    Ok(Response::with((status::Ok, "all feed")))
}

fn main() {
    let host_port = match env::var("PORT").unwrap().parse::<u16>() {
        Ok(port) => port,
        Err(_) => 3000
    };
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), host_port);

    let mut router = Router::new();

    router.get("/", all_feeds, "index");
    router.get("/all", all_feeds, "all");
    router.post("/all", all_feeds, "all");

    println!("Server running at {}", host);

    match Iron::new(router).http(host) {
        Ok(v) => println!("Ok: {:?}", v),
        Err(e) => println!("Error: {}", e)
    }

}

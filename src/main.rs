extern crate iron;
extern crate router;

use std::env;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;

use iron::prelude::*;
use iron::status;
use router::Router;

fn all_feeds(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "all feed")))
}

fn main() {
    let host_port = match env::var("PORT").unwrap().parse::<u16>() {
        Ok(port) => port,
        Err(_) => 3000
    };
    let host = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), host_port);

    let mut router = Router::new();

    router.get("/", all_feeds, "index");
    router.get("/all", all_feeds, "all");

    println!("Server running at {}", host);

    match Iron::new(router).http(host) {
        Ok(v) => println!("Ok: {:?}", v),
        Err(e) => println!("Error: {}", e)
    }

}

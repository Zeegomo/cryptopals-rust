extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Hello, world!");
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/hello/:name/:age", middleware! { |request|
        format!("Hello, {:?} year old named {:?}!", request.param("age"), request.param("name"))
    });

    server.listen("127.0.0.1:1714");
}


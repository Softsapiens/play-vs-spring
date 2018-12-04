use actix_web::{server, App, Path, Result, http};

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String
fn hello(info: Path<(String, u32)>) -> Result<String> {
    Ok(format!("Hello, {} year old named {}!", info.1, info.0))
}

fn main() {
    server::new(|| App::new().resource(
        "/hello/{name}/{age}",
        |r| r.method(http::Method::GET).with(hello))
                )
        .bind("127.0.0.1:1714")
        .unwrap()
        .run();
}

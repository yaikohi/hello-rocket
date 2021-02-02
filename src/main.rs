#![feature(decl_macro)]

use rocket::{self, get, routes};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![hello, hello_name])
    .launch();
}
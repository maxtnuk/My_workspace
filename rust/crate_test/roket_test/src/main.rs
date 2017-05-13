#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::response::NamedFile;
use std::io;

extern crate rocket;
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
fn main() {
    rocket().launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

#[get("/")]
fn index() -> HelloTemplate {
    HelloTemplate {
        name: String::from("test"),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index<'a>() -> &'a str {
    "Hello, world!"
}

#[get("/echo/<echo>")]
fn echo(echo: String) -> String {
    format!("{}", echo)
}

fn main() { 
    rocket::ignite().mount("/", routes![index, echo]).launch();
}
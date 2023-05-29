#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};
use std::io::Error;

use rocket::{response::NamedFile};

#[macro_use] extern crate rocket;

#[get("/")]
fn index<'a>() -> &'a str {
    "Hello, world!"
}

#[get("/echo/<echo>")]
fn echo(echo: String) -> String {
    format!("{}", echo)
}

#[get("/images/<image..>")]
fn image_server<'a>(image: PathBuf) -> Result<NamedFile, Error> {
    NamedFile::open(Path::new("public/images").join(image))
}

fn main() { 
    rocket::ignite().mount("/", routes![index, echo, image_server]).launch();
}
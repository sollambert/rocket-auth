#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};
use std::io::Error;

use rocket::{response::NamedFile};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

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
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .attach(SpaceHelmet::default())
    .mount("/", routes![index, echo, image_server])
    .mount("/images", StaticFiles::from("public/images/"))
}
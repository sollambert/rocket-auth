#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]
use rocket::routes;
use rocket_contrib::{serve::StaticFiles, helmet::SpaceHelmet};

//List all modules to import
pub mod tests;
pub mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
    .attach(SpaceHelmet::default())
    .mount("/", routes![
        routes::index,
        routes::echo,
        routes::image_server]
    )
    .mount("/images", StaticFiles::from("public/images/"))
}
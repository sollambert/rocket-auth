#![feature(proc_macro_hygiene, decl_macro)]
// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_codegen;
// #[macro_use] extern crate dotenv;
// #[macro_use] extern crate serde;
// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate tokio_postgres;

use rocket_auth_demo::rocket_builder;
use dotenv;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().unwrap();
    rocket_builder().launch().await?;
    Ok(())
}
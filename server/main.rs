#![feature(proc_macro_hygiene, decl_macro)]

use rocket_auth_demo::rocket_builder;
use dotenv;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().unwrap();
    rocket_builder().launch().await?;
    Ok(())
}
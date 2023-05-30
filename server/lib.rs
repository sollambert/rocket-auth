#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]
use std::env;
use std::net::Ipv4Addr;

use rocket::config::SecretKey;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{fs::FileServer, Build};
use rocket::{Rocket, Request, Response, Config};
use routes::user;

//List all modules to import
pub mod tests;
pub mod routes;
pub mod pool;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn get_env<'a>(env_var: String) -> String {
    match env::vars().into_iter().find(
    |(key, _)| key == &env_var).ok_or(()) {
        Ok(result) => {
            result.1
        },
        Err(_) => {
            println!("ENV Var {} not found", env_var);
            String::new()
        }
    }
}

pub fn rocket_builder() -> Rocket<Build> {
    let secret_key = SecretKey::derive_from(
        get_env("SESSION_SECRET".to_string()).as_bytes());
    let config = Config {
        port: 8000,
        address: Ipv4Addr::new(127,0,0,1).into(),
        temp_dir: "/tmp".into(),
        secret_key,
        ..Config::default()
    };

    rocket::build()
    .mount("/", rocket::routes![
        routes::index,
        routes::echo
    ])
    .mount("/users", rocket::routes![
        user::register_user,
        user::login_user
    ])
    .mount("/images", FileServer::from("public/images/"))
    .attach(CORS)
    .configure(config)
}
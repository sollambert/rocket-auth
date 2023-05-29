#![feature(proc_macro_hygiene, decl_macro)]

use rocket_auth_demo::rocket_builder;

fn main() {
    rocket_builder().launch();
}
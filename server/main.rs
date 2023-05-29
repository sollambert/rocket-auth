#![feature(proc_macro_hygiene, decl_macro)]
use lib::rocket_builder;

mod lib;

#[macro_use] extern crate rocket;

fn main() {
    rocket_builder().launch();
}
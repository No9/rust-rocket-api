#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate env_logger;

use std::env;
use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
    "{ \"Hello\", \"world!\" }"
}

fn main() {
    env_logger::init();

    let key = "PORT";

    let port: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::from("8080"),
    };
    let int_port: u16 = port.parse().unwrap();
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(int_port)
        .finalize();

    rocket::custom(config.unwrap())
        .mount("/", routes![index])
        .launch();
}

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

#[macro_use] extern crate error_chain;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate chrono;
extern crate ring_pwhash;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
extern crate rocket;
extern crate simple_jwt;

mod error;
mod commands;
mod db;
mod server;

fn main() {
    server::build_rocket().launch();
}

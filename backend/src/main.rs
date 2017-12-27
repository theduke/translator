#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate dotenv;
#[macro_use] extern crate error_chain;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate chrono;
extern crate ring_pwhash;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
extern crate rocket;
extern crate simple_jwt;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;
extern crate uuid;

mod error;
mod commands;
mod db;
mod config;
mod repo;
mod app;
mod api;
mod server;

fn main(){
    app::run();
}

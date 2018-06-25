#[macro_use]
extern crate diesel;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate juniper;
extern crate bcrypt;
extern crate chrono;
extern crate uuid;
extern crate dotenv;
extern crate env_logger;

mod prelude;
mod app;
mod db;
mod server;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let app = app::App::from_env().unwrap();
    server::run_server(app)
}

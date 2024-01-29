#[macro_use]
extern crate log;
#[macro_use] extern crate rocket;

use crate::route::new_app;


mod core;
mod route;
mod comm;
mod controller;
mod entity;
mod errors;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("./log4rs.yml", Default::default()).unwrap();
    new_app()
}
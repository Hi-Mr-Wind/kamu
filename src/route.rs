use rocket::{Build, Rocket};
use crate::controller::sys_controller;
use crate::errors::error_catch;
use crate::errors::error_catch::{not_found};

static mut ROUTES:Vec<String> = Vec::new();


pub fn new_app()->Rocket<Build>{
    rocket::build()
        .register("/",catchers![not_found])
        // .mount("/", routes![sys_controller::index])
        .mount("/", routes![sys_controller::fail,sys_controller::index,sys_controller::test_post_json,sys_controller::test_key])
}
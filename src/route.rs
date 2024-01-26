use rocket::{Build, Rocket};
use crate::controller::sys_controller;

static mut ROUTES:Vec<String> = Vec::new();


pub fn new_app()->Rocket<Build>{
    rocket::build()
        // .mount("/", routes![sys_controller::index])
        .mount("/", routes![sys_controller::fail,sys_controller::index,sys_controller::test_post_json])
}
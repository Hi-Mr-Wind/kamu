use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub phone:String,
    pub age:i32
}

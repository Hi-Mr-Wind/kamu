
#[derive(Debug，Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub nick_name: String,
    pub phone:String,
    pub age:u8,
    pub gender:u8,
    pub username:String,
    pub password:String,
    pub head_portrait:String,
    pub state:u8,
    pub create_time:u128,
}

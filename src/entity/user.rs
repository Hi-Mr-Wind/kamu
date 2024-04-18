use std::fmt::{Debug, Write};

use sea_orm::{ColumnTrait, EntityName, EnumIter, Iden, IdenStatic};
use serde::Serialize;

#[derive(Debug,Serialize, Default)]
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

#[derive(Copy, Clone, Default, Debug)]
pub struct UserTable;

impl IdenStatic for UserTable {
    fn as_str(&self) -> &str {
        todo!()
    }
}

impl Iden for UserTable {
    fn unquoted(&self, s: &mut dyn Write) {
        todo!()
    }
}

impl EntityName for UserTable{
    fn table_name(&self) -> &str {
        "user"
    }
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Column {
    Uid,
    NickName,
}
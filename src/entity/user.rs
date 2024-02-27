use std::fmt::{Debug, Formatter, Write};

use sea_orm::{ColumnDef, ColumnTrait, ColumnType, ColumnTypeTrait, DeriveColumn, DeriveEntity, EntityName, EntityTrait, EnumIter, Iden, IdenStatic};

#[derive(Copy,Clone,Debug，Serialize, Default, DeriveEntity)]
#[sea_orm(table_name = "user")]
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

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct UserTable;

impl EntityName for UserTable{
    fn table_name(&self) -> &str {
        "user"
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Uid,
    NickName,
}
impl ColumnTrait for Column {
    type EntityName = UserTable;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(None).def(),
            Self::Name => ColumnType::String(None).def(),
        }
    }
}
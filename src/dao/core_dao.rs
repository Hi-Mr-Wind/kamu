use sea_orm::{ColumnTrait, Condition, EntityTrait};
use crate::entity::user::{Column, User, UserTable};
use crate::persistent_layer::db::DB;
use cake::Entity as Cake;

pub async  fn select_user_by_username(username: String) ->User{
    let chocolate: Vec<User> = UserTable::find()
        .filter(Column::Uid.contains("1"))
        .all(DB)
        .await?;
}
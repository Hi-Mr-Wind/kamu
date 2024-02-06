use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct AppConfig {
    pub database : DatabaseConfig

}
/// 数据库连接参数
#[derive(Deserialize,Debug)]
pub struct DatabaseConfig {
    ///数据库
    pub data_base : String,
    ///连接地址
    pub host : String,
    ///账号
    pub username : String,
    ///密码
    pub password : String,
    ///数据库名称
    pub db_name : String,
}
///连接池参数
#[derive(Deserialize,Debug)]
pub struct ConnectionPool{
    ///最大连接数
    pub max_connections : u32,
    ///最小连接数
    pub min_connections : u32,
}
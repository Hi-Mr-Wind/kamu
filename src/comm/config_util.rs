use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};
use serde::Deserialize;

thread_local! {
    pub static COINFIG: Arc<Mutex<AppConfig>> = Arc::new(Mutex::new(get_config()));
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    ///数据库配置
    pub database: DatabaseConfig,
    ///连接池配置
    pub connection_poll: ConnectionPool,
    ///文件存储位置
    pub file_path: String,

}

/// 数据库连接参数
#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    ///数据库
    pub data_base: String,
    ///连接地址
    pub host: String,
    ///账号
    pub username: String,
    ///密码
    pub password: String,
    ///数据库名称
    pub db_name: String,
}

///连接池参数
#[derive(Deserialize, Debug)]
pub struct ConnectionPool {
    ///最大连接数
    pub max_connections: u32,
    ///最小连接数
    pub min_connections: u32,
}

/// 读取配置文件并获取配置信息
pub fn get_config() -> AppConfig {
    let mut result = fs::File::open("config.yml").expect("读取配置文件失败！");
    let mut buffer = String::new();
    result.read_to_string(&mut buffer).expect("读取配置文件信息失败！");
    serde_yaml::from_str(&buffer).expect("配置文件格式异常！")
}

pub fn get_db_url() -> String{
    let binding = COINFIG.with(|f|f.clone());
    let config = binding.lock().expect("读取配置信息失败");
    format!("{}://{}:{}@{}/{}",config.database.data_base,config.database.username,config.database.password,config.database.host,config.database.db_name)
}
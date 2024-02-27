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
    ///sql日志开关
    pub sql_logger_open: bool,
    ///sql日志级别
    pub sql_logger_level: String,
}

///连接池参数
#[derive(Deserialize, Debug)]
pub struct ConnectionPool {
    ///最大连接数
    pub max_connections: u32,
    ///最小连接数
    pub min_connections: u32,
    ///连接超时时间
    pub connect_timeout: u64,
    ///等待获取连接所花费的最长时间
    pub acquire_timeout: u64,
    ///最大空闲时间
    pub idle_timeout: u64,
    ///单个连接的最长生存期
    pub max_lifetime: u64,


}

/// 读取配置文件并获取配置信息
pub fn get_config() -> AppConfig {
    let mut result = fs::File::open("config.yml").expect("读取配置文件失败！");
    let mut buffer = String::new();
    result.read_to_string(&mut buffer).expect("读取配置文件信息失败！");
    serde_yaml::from_str(&buffer).expect("配置文件格式异常！")
}



use std::sync::Arc;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::comm::config_util::{COINFIG};

thread_local! {
    pub static DB:Arc<DatabaseConnection> = Arc::new(create_connection());
}


/// 创建数据库连接池
pub fn create_connection() -> DatabaseConnection {
    let url = get_db_url().as_str();
    let mut opt = ConnectOptions::new(url);
    let binding = COINFIG.with(|f| f.clone());
    let config = binding.lock().expect("读取配置信息失败");
    let log_info = match config.database.sql_logger_level {
        String::from("info") => log::LevelFilter::Info,
        String::from("debug") => log::LevelFilter::Debug,
        String::from("error") => log::LevelFilter::Error,
        String::from("warn") => log::LevelFilter::Warn,
        _ => log::LevelFilter::Info
    };

    opt.max_connections(config.connection_poll.max_connections)
        .min_connections(config.connection_poll.min_connections)
        .connect_timeout(Duration::from_secs(config.connection_poll.connect_timeout))
        .acquire_timeout(Duration::from_secs(config.connection_poll.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.connection_poll.idle_timeout))
        .max_lifetime(Duration::from_secs(config.connection_poll.max_lifetime))
        .sqlx_logging(config.database.sql_logger_open)
        .sqlx_logging_level(log_info)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    Database::connect(opt)?
}

pub fn get_db_url() -> String {
    let binding = COINFIG.with(|f| f.clone());
    let config = binding.lock().expect("读取配置信息失败");
    format!("{}://{}:{}@{}/{}", config.database.data_base, config.database.username, config.database.password, config.database.host, config.database.db_name)
}
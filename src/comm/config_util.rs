use std::fs;
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Mutex;

pub type Store<'s> = Arc<Mutex<AppConfig<'s>>>;

pub static mut CONFIG: Store = Arc::new(Mutex::new(AppConfig::default()));

#[derive(Serialize,Default)]
pub struct AppConfig<'s> {
    pub file_path: &'s str,
}
pub async unsafe fn read_config(){
    let result = fs::read_to_string("AppConfig.toml").unwrap();
    let value:AppConfig = toml::from_str(&result).unwrap_or_default();
    let x = CONFIG.lock().await;
    x.file_path = value.file_path;
    drop(result);
}
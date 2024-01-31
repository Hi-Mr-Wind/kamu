#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use log::Level::Error;
use uuid::{NoContext, Timestamp, Uuid, uuid};

use crate::core::read_file::file_handling;

mod core;
mod route;
mod comm;
mod controller;
mod entity;
mod errors;
mod service;

// #[launch]
// fn rocket() -> _ {
//     log4rs::init_file("./log4rs.yml", Default::default()).unwrap();
//     new_app()
// }


#[tokio::test]
async fn main_test() {
    let star_time = get_current_timestamp_ms();
    let ts = Timestamp::from_unix(NoContext, SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64, SystemTime::now()
                                      .duration_since(UNIX_EPOCH)
                                      .unwrap()
                                      .as_millis() as u32);
    let uuid = Uuid::new_v7(ts).to_string();
    let src_path = PathBuf::from(r"C:\Users\Admin\Downloads\Cangjie-0.39.8-windows_x64.exe");
    let dest_dir = PathBuf::from(format!("./enc_file/{}",uuid));
    let key = 0x42; // 使用一个简单的异或密钥
    let dest_data = PathBuf::from(format!("./enc_file/{}.tag.gz",uuid).as_str());

    // 上传文件并加密
    let result = file_handling::convert_file(&src_path, &dest_dir, key, Arc::new(uuid)).await;
    match result {
        Ok(file_handling) => {
            // file_handling::compress_folder_to_tar_gz(&dest_dir, &dest_data).await.expect("生成文件错误");
            println!("{:?}",file_handling)
        }
        Err(e) => {
           error!("{:?}",e)
        }
    }
    let end_time = get_current_timestamp_ms();
    let end_time = end_time - star_time;
    println!("文件处理耗时：{}", end_time);
    // file_handling::restore_file(&dest_dir,)
}
fn get_current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
fn main() {}
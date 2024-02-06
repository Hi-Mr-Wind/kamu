#[macro_use]
extern crate log;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::comm::app_cache::CACHE_INSTANCE;
use crate::route::routes::new_app;

mod core;
mod comm;
mod controller;
mod entity;
mod errors;
mod service;
mod route;
mod app_middleware;


#[tokio::main]
async fn main() {
    log4rs::init_file("./log4rs.yml", Default::default()).unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, new_app()).await.unwrap();
}


#[tokio::test]
async fn main_test() {
    let caceh = CACHE_INSTANCE.clone();
    caceh.insert("key".to_string(), "value".to_string(), Option::from(Duration::from_secs(1)));
    let option = caceh.get("key");
    match option {
        None => {println!("未找到引用值")}
        Some(axum) => {println!("{}",axum)}
    }
    tokio::spawn(async move{
     let c = CACHE_INSTANCE.clone();
        let option = c.get("key");
        match option {
            None => {println!("协成1未找到引用值")}
            Some(axum) => {println!("协成：{}",axum)}
        }
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    let option = caceh.get("key");
    match option {
        None => {println!("引用值已被过期删除")}
        Some(axum) => {println!("{}",axum)}
    }
    //
    // let handle = tokio::spawn(async move {
    //     let caceh = CACHE_INSTANCE.clone();
    //     caceh.insert("key1".to_string(), "value1".to_string(), Option::from(Duration::from_secs(1)));
    //     let option = caceh.get("key1");
    //     match option {
    //         None => {println!("线程1未找到引用值")}
    //         Some(axum) => {println!("{}",axum)}
    //     }
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    //     let option = caceh.get("key1");
    //     match option {
    //         None => {println!("线程1引用值已被过期删除")}
    //         Some(axum) => {println!("{}",axum)}
    //     }
    // });
    // handle.await;
    // let star_time = get_current_timestamp_ms();
    // let ts = Timestamp::from_unix(NoContext, SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_millis() as u64, SystemTime::now()
    //                                   .duration_since(UNIX_EPOCH)
    //                                   .unwrap()
    //                                   .as_millis() as u32);
    // let uuid = Uuid::new_v7(ts).to_string();
    // let src_path = PathBuf::from(r"C:\Users\29120\Downloads\WebStorm-2023.2.1.exe");
    // let dest_dir = PathBuf::from(format!("./enc_file/{}", uuid));
    // let key = 0x42; // 使用一个简单的异或密钥
    // let dest_data = PathBuf::from(format!("./enc_file/{}", uuid).as_str());
    //
    // // 上传文件并加密
    // let result = file_handling::convert_file(&src_path, &dest_dir, key, Arc::new(uuid)).await;
    // match result {
    //     Ok(file_handling) => {
    //         // file_handling::compress_folder_to_zip(&dest_dir, &dest_data).await;
    //         println!("{:?}", file_handling)
    //     }
    //     Err(e) => {
    //         error!("{:?}",e)
    //     }
    // }
    // let end_time = get_current_timestamp_ms();
    // let end_time = end_time - star_time;
    // println!("文件处理耗时：{}秒", end_time / 1000u128);
    // file_handling::restore_file(&dest_dir,)
}

fn get_current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// #[tokio::main]
// async fn main() {
//     let star_time = get_current_timestamp_ms();
//     let uuid = String::from("10690807-e5ff-7bbb-b529-ee9ede39eda0");
//     let dest_dir = PathBuf::from(format!("./enc_file/{}", &uuid));
//     // let dest_dir = PathBuf::from(r"D:\rust-kamu\enc_file\1");
//     let dest_data = PathBuf::from(format!("./enc_file/{}", &uuid).as_str());
//     // file_handling::compress_folder_to_zip(&dest_dir, &dest_data).await;
//     let key = 0x42; // 使用一个简单的异或密钥
//     let uid = PathBuf::from(uuid);
//     restore_file(&dest_dir, &dest_data, key, &uid).await.expect("TODO: panic message");
//     let end_time = get_current_timestamp_ms();
//     let end_time = end_time - star_time;
//     println!("文件处理耗时：{}秒", end_time / 1000u128);
// }
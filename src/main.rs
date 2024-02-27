#[macro_use]
extern crate log;

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::comm::app_cache::CACHE_INSTANCE;
use crate::route::routes::new_app;
use sysinfo::{Components, Disks, Networks, System};
mod core;
mod comm;
mod controller;
mod entity;
mod errors;
mod service;
mod route;
mod app_middleware;
mod persistent_layer;
mod dao;


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
        None => { println!("未找到引用值") }
        Some(axum) => { println!("{}", axum) }
    }
    tokio::spawn(async move {
        let c = CACHE_INSTANCE.clone();
        let option = c.get("key");
        match option {
            None => { println!("协成1未找到引用值") }
            Some(axum) => { println!("协成：{}", axum) }
        }
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    let option = caceh.get("key");
    match option {
        None => { println!("引用值已被过期删除") }
        Some(axum) => { println!("{}", axum) }
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
#[tokio::test]
async fn  test1(){
    println!("CPU核心数：{}",num_cpus::get());
}

#[tokio::test]
async fn test_sys_info(){
// 请注意，我们使用“new_all”来确保所有列表
// 组件、网络接口、磁盘和用户已经存在
    let mut sys = System::new_all();

// 首先，我们更新“系统”结构的所有信息。
    sys.refresh_all();
    println!("=> system:");
// RAM 和交换信息：
    println!("总内存:           {} MB", sys.total_memory()/1024u64/1024u64);
    println!("已使用内存 :       {} MB", sys.used_memory()/1024u64/1024u64);
    println!("总交换内存  :      {} MB", sys.total_swap()/1024u64/1024u64);
    println!("已使用交换内存   :  {} MB", sys.used_swap()/1024u64/1024u64);

// 显示系统信息：
    println!("系统名称:             {:?}", System::name().unwrap());
    println!("系统版本:             {:?}", System::kernel_version().unwrap());
    println!("系统OS版本:           {:?}", System::os_version().unwrap());
    println!("系统主机名:           {:?}", System::host_name().unwrap());

// CPU数量：
    println!("CPU核心数: {}", sys.cpus().len());

// // 显示进程 ID、名称和磁盘使用情况：
//     for (pid, process) in sys.processes() {
//         println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
//     }

// 我们显示所有磁盘的信息：
    println!("=> 磁盘信息:");
    let disks = Disks::new_with_refreshed_list();
    let mut numerical_order = 1;
    for disk in &disks {
        println!("-----磁盘{}-----",numerical_order);
        println!("磁盘名称：{:?}\n磁盘类型：{}\n文件系统：{:?}\n磁盘总大小：{}GB\n可用空间：{}GB",
                 disk.name(),
                 disk.kind(),
                 disk.file_system(),
                 disk.total_space()/1024u64/1024u64/1024u64,
                 disk.available_space()/1024u64/1024u64/1024u64);
        numerical_order += 1
    }

// 网络接口名称、接收的数据和传输的数据：
    let networks = Networks::new_with_refreshed_list();
    println!("=> 网络信息:");
    for (interface_name, data) in &networks {
        println!("{interface_name}: {}/{} B", data.received(), data.transmitted());
    }

// 组件温度：
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }

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
fn find_number(target: u32) -> Option<u32> {
    for i in 0..1000000 {
        if i == target {
            return Some(i);
        }
    }
    None
}

#[tokio::test]
async fn test_password() {
    let target = 999999; // 替换为你想要找到的六位数字
    let now = Instant::now();

    match find_number(target) {
        Some(number) => {
            let elapsed = now.elapsed();
            println!("找到号码: {:06}", number);
            println!("所用时间: {:.2?}", elapsed);
        }
        None => {
            let elapsed = now.elapsed();
            println!("在范围内找不到数字.");
            println!("时间（以秒为单位）: {:.2?}", elapsed);
        }
    }
}
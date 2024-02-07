use std::{fs, io};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use sha256::try_digest;

use zip_archive::Archiver;

use crate::core::read_file::create_file_hash;
use crate::entity::file_data::FileData;

//块尺寸256KB
const CHUNK_SIZE: usize = 256 * 1024;

/// # 转换文件
/// 将文件转换并切割加密，生成最终存储的目标数据
/// `src_path`原始文件路径
/// `dest_dir`目标文件夹路径
/// `key` 加密key
pub async fn convert_file(src_path: &PathBuf, dest_dir: &PathBuf, key: u8, uid: Arc<String>) -> io::Result<FileData> {
    // 创建存储文件夹
    fs::create_dir_all(dest_dir)?;

    // 打开原始文件
    let file = File::open(src_path)?;
    let metadata = file.metadata()?;
    let file_name = src_path.file_name().unwrap().to_string_lossy();
    let file_size = metadata.len();

    // 读取并切割文件
    let mut reader = io::BufReader::new(file);
    let mut chunk_number = 1;
    loop {
        let mut buffer = vec![0; CHUNK_SIZE];
        // 读取文件内容到缓冲区
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // 到达文件末尾
        }
        let uid_cloud = uid.clone();
        // 创建新文件名
        let chunk_path = dest_dir.join(format!("{}-{}", &uid_cloud, chunk_number));
        //开启协程写入文件
        // let handle = tokio::spawn(async move {
        let mut chunk_file = File::create(chunk_path).ok().unwrap();

        // 异或加密每个切片
        for chunk in buffer.iter_mut() {
            *chunk ^= key;
        }
        // 将缓冲区内容写入新文件
        chunk_file.write_all(&buffer[..bytes_read]).unwrap();

        // 清理缓冲区
        drop(buffer);
        // });
        chunk_number += 1;
    }
    // 写入原文件信息
    let file_hash = create_file_hash(src_path);
    let cow = file_name.clone().to_string();
    let size = file_size.clone();
    let data_stc = FileData::new(cow, size, file_hash, chunk_number);
    let data = data_stc.to_json();

    let file_info_path = dest_dir.join(format!("{}.data", uid));
    let mut file_info_file = fs::File::create(file_info_path)?;
    write!(file_info_file, "{}", data)?;
    Ok(data_stc)
}

/// # 还原并存储文件
/// `src_path`原始文件夹路径
/// `dest_dir`目标文件夹路径
/// `key` 解密key
pub async fn restore_file(src_dir: &PathBuf, dest_path: &PathBuf, key: u8, uid: &PathBuf) -> io::Result<()> {
    let file_data_name = uid.join(".data");
    // let buf = PathBuf::from(".data");
    //数据文件路径
    let s_str = uid.to_str().unwrap().to_owned();
    let string = format!("{}{}", &s_str, ".data");
    let data_f = src_dir.join(string);
    // 打开并读取项目记录文件
    let mut file_data = File::open(data_f)?;
    let mut json = String::new();
    file_data.read_to_string(&mut json)?;
    //文件数据
    let file_data: FileData = serde_json::from_str(&*json)?;

    //还原文件路径
    let path_buf = dest_path.join(&file_data.file_name);
    // 创建一个原始文件
    let mut file_restore = File::create(path_buf)?;
    // 缓冲区
    let mut buffer = vec![0; CHUNK_SIZE];
    //寻找并读取文件内容
    let mut i = 1;
    loop {
        if i >= file_data.file_sequence {
            break;
        }
        let s_src = src_dir.to_str().unwrap().to_owned();
        let path = PathBuf::from(format!("{}/{}{}{}",s_src,&s_str,"-",i));
        i+=1;
        info!("{:?}",&path);
        let mut result = fs::File::open(&path)?;
        loop {
            let n = result.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            // 异或加密每个切片
            for chunk in buffer.iter_mut() {
                *chunk ^= key;
            }
            file_restore.write_all(&buffer[..n])?;
        }
    }

    Ok(())
}

/// # 压缩生成最终数据文件
/// source_dir 源数据文件夹
/// target_file 新文件路径
pub async fn compress_folder_to_zip(source_dir: &PathBuf, target_file: &PathBuf) {
    let mut archiver = Archiver::new();
    archiver.push(source_dir);
    archiver.set_destination(target_file);
    archiver.set_thread_count(3);
    match archiver.archive() {
        Ok(_) => (),
        Err(e) => {
            error!("文件转换异常！{}",e)
        }
    }
}


/// 计算文件哈希值
pub fn create_file_hash(file_path: &PathBuf) -> String {
    // let input = Path::new(file_path);
    let result = try_digest(file_path);
    match result {
        Ok(s) => { s }
        Err(s) => {
            error!("{}",s);
            return "".to_string();
        }
    }
}
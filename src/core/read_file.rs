use std::hash::Hash;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use sha256::try_digest;

// 计算文件哈希值
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

pub mod file_handling {
    use std::{fs, io};
    use std::fs::File;
    use std::io::{BufRead, Write};
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::Arc;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tar::Builder;

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
            let handle = tokio::spawn(async move {
                let mut chunk_file = File::create(chunk_path).ok().unwrap();

                // 异或加密每个切片
                for chunk in buffer.iter_mut() {
                    *chunk ^= key;
                }
                // 将缓冲区内容写入新文件
                chunk_file.write_all(&buffer[..bytes_read]).unwrap();

                // 清理缓冲区
                drop(buffer)
            });
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
        let buf = PathBuf::from(".data");
        //数据文件路径
        let data = src_dir.join(buf);
        // 打开并读取项目记录文件
        let mut file_data = File::open(&data)?;
        let mut json = String::new();
        file_data.read_to_string(&mut json)?;
        //文件数据
        let file_data: FileData = serde_json::from_str(json.as_str())?;

        //还原文件路径
        let path_buf = dest_path.join(&file_data.file_name);
        // 创建一个原始文件
        let mut file_restore = File::create(path_buf)?;

        // 缓冲区应有大小
        let buf_size = 256usize * file_data.file_sequence;
        // 缓冲区
        let mut buffer = vec![0; buf_size];
        //寻找并读取文件内容
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            // 如果是文件
            if metadata.is_file() {
                let option = path.file_name()?;
                // 如果是数据文件则跳过
                if file_data_name.eq(&option) {
                    continue
                } else {

                }
            }
        }
        file_restore.write_all(&buffer)?;
        Ok(())
    }

    /// # 压缩生成最终数据文件
    /// source_dir 源数据文件夹
    /// target_file 新文件路径和名称
    pub async fn compress_folder_to_tar_gz(source_dir: &PathBuf, target_file: &PathBuf) -> io::Result<()> {
        // 创建tar.gz文件
        let file = File::create(target_file)?;
        let enc = GzEncoder::new(file, Compression::default());
        let mut tar = Builder::new(enc);

        // 遍历源目录
        for entry in fs::read_dir(source_dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;

            // 检查是否为目录或文件
            if metadata.is_dir() {
                // 如果是目录，则递归处理
                // compress_folder_to_tar_gz(&path, target_file).await?;
                warn!("发现违规目录！");
            } else if metadata.is_file() {
                // 如果是文件，则添加到tar文件中
                let mut file = File::open(&path)?;
                tar.append_file(path, &mut file)?;
            }
        }
        tar.finish()?;

        Ok(())
    }
}
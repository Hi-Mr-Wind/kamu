use std::fs::File;
use std::hash::Hash;
use std::path::Path;

use sha256::try_digest;

pub fn read_file(file_path: &str) -> Option<File> {
    let file = File::open(file_path);
    match file {
        Ok(f) => {Some(f)}
        Err(e) => {None}
    }
}

// 计算文件哈希值
pub fn create_file_hash (file_path: &str) ->String{
    let input = Path::new(file_path);
    let result = try_digest(input);
    match result {
        Ok(s)=>{s}
        Err(s)=>{error!("{}",s);
            return "".to_string()
        }
    }
}

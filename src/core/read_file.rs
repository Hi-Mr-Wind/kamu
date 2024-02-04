use std::hash::Hash;
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
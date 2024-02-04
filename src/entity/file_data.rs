use log::Level;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct FileData {
    //文件名
    pub file_name: String,
    //文件大小
    pub file_size: u64,
    //文件哈希
    pub file_hash: String,
    //文件序列数
    pub file_sequence : i32,
}

impl FileData {
    pub fn new(file_name: String, file_size: u64, file_hash: String,file_sequence: i32) -> FileData {
        FileData { file_name, file_size, file_hash,file_sequence }
    }

    pub fn to_json(&self) -> String{
        let result = serde_json::to_string(self);
        match result {
            Ok(s)=>{
                s
            },
            Err(e)=>{
                log!(Level::Error,"{:?}",e);
                "".to_string()
            }
        }
    }

}

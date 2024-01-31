use log::Level;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FileData {
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
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

use log::Level;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FileData {
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
}

impl FileData {
    pub fn new(file_name: String, file_size: u64, file_hash: String) -> FileData {
        FileData { file_name, file_size, file_hash }
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

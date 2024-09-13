use serde::Serialize;

pub mod user;
pub mod file_data;

/// 系统信息
#[derive(Debug, Serialize, Clone)]
pub struct OsInfo {
    ///总内存
    pub total_memory: u64,
    ///已使用内存
    pub used_memory: u64,
    ///系统名称
    pub os_name: String,
    ///CPU核心数
    pub cpu_len: usize,
    /// 磁盘总大小
    pub total_space: u64,
    ///磁盘已使用大小
    pub available_space: u64,
}

impl OsInfo {
    pub fn new() -> OsInfo {
        OsInfo {
            total_memory: 0,
            used_memory: 0,
            os_name: "".to_string(),
            cpu_len: 0,
            total_space: 0,
            available_space: 0,
        }
    }
}
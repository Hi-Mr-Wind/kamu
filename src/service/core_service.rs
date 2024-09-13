use sysinfo::{Disks, System};
use crate::entity::OsInfo;
use crate::entity::user::User;

pub struct CoreService();
impl CoreService {
    pub fn login(user: User) {}

    pub fn get_os_info() -> OsInfo {
        let mut os_info = OsInfo::new();
        let mut sys = System::new_all();
        sys.refresh_all();
        os_info.total_space = sys.total_memory()/1024u64/1024u64;
        os_info.used_memory = sys.used_memory()/1024u64/1024u64;
        os_info.os_name = System::name().expect("获取操作系统信息失败！");
        os_info.cpu_len = sys.cpus().len();
        let disks = Disks::new_with_refreshed_list();
        let mut numerical_order = 1;

        let mut total_space = 0;
        let mut available_space = 0;
        for disk in &disks {
            total_space += disk.total_space() / 1024u64 / 1024u64 / 1024u64;
            available_space  += disk.available_space() / 1024u64 / 1024u64 / 1024u64;
            numerical_order += 1
        }
        os_info.total_space = total_space;
        os_info.available_space = available_space;
        os_info
    }
}
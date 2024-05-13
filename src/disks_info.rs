use serde::{Serialize};
use sysinfo::{Disks};

#[derive(Serialize)]
pub struct DiskInfo {
    pub device_name: String,
    pub file_system: String,
    pub total_space: u64,
    pub free_space: u64,
    pub used_space: u64,
    pub mount_point: String,
}

pub fn get_disk_info() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    let mut disk_info_vec = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let device_name = disk.name().to_string_lossy().into_owned();
        let file_system = disk.file_system().to_string_lossy().into_owned();
        let total_space = disk.total_space();
        let free_space = disk.available_space();
        let used_space = total_space - free_space;
        let mount_point = disk.mount_point().to_string_lossy().into_owned();

        disk_info_vec.push(DiskInfo {
            device_name,
            file_system,
            total_space,
            free_space,
            used_space,
            mount_point,
        });
    }

    Ok(disk_info_vec)
}

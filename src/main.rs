use serde::{Serialize};
//use serde_json::to_string;
//use sysinfo::{Disks, System};

use std::time::{SystemTime, UNIX_EPOCH};

mod memory_info ;
use crate::memory_info::get_memory_info;
use crate::memory_info::MemInfo;

mod disks_info ;
use crate::disks_info::get_disk_info;
use crate::disks_info::DiskInfo;

mod net_info ;
use crate::net_info::get_net_info;
use crate::net_info::NetInfo;

#[derive(Serialize)]
    struct Monitor {
        timestamp: u64,
        disk_info: Vec<DiskInfo>,
        memory_info: MemInfo,
        net_info: Vec<NetInfo>,
    }


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = now.as_millis() as u64;

    let sysmonitor = Monitor {
        timestamp,
        memory_info: get_memory_info()?,
        disk_info: get_disk_info()?,
        net_info: get_net_info()?,
    };

    // Convert MemInfo to JSON
    println!("{}", serde_json::to_string_pretty(&sysmonitor).unwrap());
//    let json_data = to_string(&sysmonitor)?;
//    println!("{}", json_data);

    Ok(())
}

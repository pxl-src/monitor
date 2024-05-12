#![allow(unused_imports)]
use serde::{Serialize};
use serde_json::to_string;
use sysinfo::{Disks, System};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::memory_info::get_memory_info;
use crate::memory_info::MemInfo;
use crate::disks_info::get_disk_info;
use crate::disks_info::DiskInfo;

mod disks_info ;
mod memory_info ;

#[derive(Serialize)]
    struct Monitor {
        timestamp: u64,
        disk_info: Vec<DiskInfo>,
        memory_info: MemInfo,
    }


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = now.as_millis() as u64;

    let sysmonitor = Monitor {
        timestamp,
        memory_info: get_memory_info()?,
        disk_info: get_disk_info()?,
    };

    // Convert MemInfo to JSON
    let json_data = to_string(&sysmonitor)?;

    println!("{}", json_data);

    Ok(())
}

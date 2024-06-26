use serde::{Serialize};
//use serde_json::to_string;
//use sysinfo::{Disks, System};

use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;

mod memory_info ;
use crate::memory_info::get_memory_info;
use crate::memory_info::MemInfo;

mod disks_info ;
use crate::disks_info::get_disk_info;
use crate::disks_info::DiskInfo;

mod net_info ;
use crate::net_info::get_net_info;
use crate::net_info::NetInfo;

mod process_info ;
use crate::process_info::get_proc_info;
use crate::process_info::ProcInfo;

#[derive(Parser)]
#[clap(author, version, about = "Monitor system resources")]
struct Args {
    #[clap(short = 'd', long = "disk", help = "Include disk information")]
    include_disk: bool,
    #[clap(short = 'm', long = "memory", help = "Include memory information")]
    include_memory: bool,
    #[clap(short = 'n', long = "net", help = "Include network information")]
    include_net: bool,
    #[clap(short = 'p', long = "proc", help = "Include process information")]
    include_proc: bool,
}


#[derive(Serialize, Default)]
    struct Monitor {
        timestamp: u64,
        disk_info: Option<Vec<DiskInfo>>,
        memory_info: Option<MemInfo>,
        net_info: Option<Vec<NetInfo>>,
        proc_info: Option<Vec<ProcInfo>>,
    }


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = now.as_millis() as u64;

    let mut disk_info: Option<Vec<DiskInfo>> = None;
    let mut memory_info: Option<MemInfo> = None;
    let mut net_info: Option<Vec<NetInfo>> = None;
    let mut proc_info: Option<Vec<ProcInfo>> = None;

        let args = Args::parse(); // Parses command-line arguments


    if args.include_disk {
        // Get disk information using sysinfo or other libraries
        disk_info = Some(get_disk_info()?) ;
    }

    if args.include_memory {
        memory_info = Some(get_memory_info()?) ;
    }

    if args.include_net {
        net_info = Some(get_net_info()?) ;
    }

    if args.include_proc {
        proc_info = Some(get_proc_info()?);
    } 

    let sysmonitor = Monitor {
        timestamp,
        memory_info,
        disk_info ,
        net_info ,
        proc_info ,
    };

    // Convert MemInfo to JSON
    println!("{}", serde_json::to_string_pretty(&sysmonitor).unwrap());
//    let json_data = to_string(&sysmonitor)?;
//    println!("{}", json_data);

    Ok(())
}

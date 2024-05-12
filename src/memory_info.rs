use serde::{Serialize};
use sysinfo::{System};

#[derive(Serialize)]
pub struct MemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

pub fn get_memory_info() -> Result<MemInfo, Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mem_info = MemInfo {
        total_memory: sys.total_memory() ,
        used_memory: sys.used_memory() ,
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };
    Ok(mem_info)
}

use serde::{Serialize};
use sysinfo::{System};

#[derive(Serialize)]
pub struct ProcInfo {
    pub pid : u32,
    pub name : String,
    pub cpu_usage: f32, // Percentage of CPU used
    pub memory_usage: u64, 
}

pub fn get_proc_info() -> Result<Vec<ProcInfo>, Box<dyn std::error::Error>> {
    let mut proc_info_vec = Vec::new();
    let mut sys = System::new_all();

    sys.refresh_all(); // Refresh system information

    // Loop through all processes
    for (pid, process) in sys.processes() {


        // Try to retrieve memory usage from procfs
        let memory_usage = process.memory() ;

        let process_info = ProcInfo {
            pid: pid.as_u32(), // Explicitly cast the dereferenced Pid to u32
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage() , // Convert to percentage
            memory_usage,
        };

        proc_info_vec.push(process_info);
    }
    Ok(proc_info_vec)
}
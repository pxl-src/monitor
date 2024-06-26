use hyper::{Body, Response};
use serde::Serialize;
use sysinfo::{ProcessExt, System, SystemExt};
use std::io;

#[derive(Serialize)]
struct ProcessInfo {
    pid: i32,
    name: String,
    cpu_usage: f32,
    memory: u64,
    virtual_memory: u64,
}

pub async fn get_process_info() -> io::Result<Response<Body>> {
    let mut sys = System::new_all();
    sys.refresh_all(); // Refresh the system to get updated information

    let process_infos: Vec<ProcessInfo> = sys.processes().iter().map(|(_, process)| {
        ProcessInfo {
            pid: i32::from(process.pid()),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            virtual_memory: process.virtual_memory(),
        }
    }).collect();

    let json = serde_json::to_string_pretty(&process_infos).unwrap();
    Ok(Response::new(Body::from(json)))
}

use hyper::{Body, Response};
use serde::Serialize;
use sysinfo::{ProcessExt, System, SystemExt};
use std::io::{self, BufRead};
use std::fs::File;
use users::get_user_by_uid;

#[derive(Serialize)]
struct ProcessInfo {
    pid: i32,
    name: String,
    cpu_usage: f32,
    memory: u64,
    virtual_memory: u64,
    owner: String,
}

fn get_process_owner(pid: i32) -> String {
    let status_path = format!("/proc/{}/status", pid);
    if let Ok(file) = File::open(status_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if line.starts_with("Uid:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        if let Ok(uid) = parts[1].parse::<u32>() {
                            if let Some(user) = get_user_by_uid(uid) {
                                return user.name().to_string_lossy().to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}

pub async fn get_process_info() -> io::Result<Response<Body>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let process_infos: Vec<ProcessInfo> = sys.processes().iter().map(|(_, process)| {
        ProcessInfo {
            pid: i32::from(process.pid()),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            virtual_memory: process.virtual_memory(),
            owner: get_process_owner(i32::from(process.pid())),
        }
    }).collect();

    let json = serde_json::to_string_pretty(&process_infos).unwrap();
    Ok(Response::new(Body::from(json)))
}

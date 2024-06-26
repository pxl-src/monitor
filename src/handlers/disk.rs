use hyper::{Body, Response};
use serde::Serialize;
//use std::fs::File;
//use std::io::{self, Read};
use std::io;

#[derive(Serialize)]
struct DiskInfo {
    filesystem: String,
    total: u64,
    used: u64,
    available: u64,
    used_percentage: f32,
    mounted_on: String,
}

pub async fn get_disk_info() -> io::Result<Response<Body>> {
    let output = std::process::Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut disk_infos = Vec::new();

    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 6 {
            let total = parts[1].replace("G", "").parse().unwrap_or(0.0) as u64;
            let used = parts[2].replace("G", "").parse().unwrap_or(0.0) as u64;
            let available = parts[3].replace("G", "").parse().unwrap_or(0.0) as u64;
            let used_percentage = parts[4].replace("%", "").parse().unwrap_or(0.0);

            disk_infos.push(DiskInfo {
                filesystem: parts[0].to_string(),
                total,
                used,
                available,
                used_percentage,
                mounted_on: parts[5].to_string(),
            });
        }
    }

    //let json = serde_json::to_string(&disk_infos).unwrap();
    let json = serde_json::to_string_pretty(&disk_infos).unwrap();
    Ok(Response::new(Body::from(json)))
}

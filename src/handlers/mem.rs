use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct MemInfo {
    total: u64,
    available: u64,
}

pub async fn get_mem_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/meminfo")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut total = 0;
    let mut available = 0;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
    }

    let mem_info = MemInfo { total, available };
    //let json = serde_json::to_string(&mem_info).unwrap();
    let json = serde_json::to_string_pretty(&mem_info).unwrap();

    Ok(Response::new(Body::from(json)))
}

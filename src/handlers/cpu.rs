use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct CpuInfo {
    cores: usize,
}

pub async fn get_cpu_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/cpuinfo")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let cores = content.lines().filter(|line| line.starts_with("processor")).count();
    let cpu_info = CpuInfo { cores };
    //let json = serde_json::to_string(&cpu_info).unwrap();
    let json = serde_json::to_string_pretty(&cpu_info).unwrap();

    Ok(Response::new(Body::from(json)))
}

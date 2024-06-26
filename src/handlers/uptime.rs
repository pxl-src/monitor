use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct UptimeInfo {
    uptime: f64,
    idle: f64,
}

pub async fn get_uptime_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/uptime")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let parts: Vec<&str> = content.split_whitespace().collect();
    let uptime_info = UptimeInfo {
        uptime: parts[0].parse().unwrap(),
        idle: parts[1].parse().unwrap(),
    };
    //let json = serde_json::to_string(&uptime_info).unwrap();
    let json = serde_json::to_string_pretty(&uptime_info).unwrap();

    Ok(Response::new(Body::from(json)))
}

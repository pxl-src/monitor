use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct LoadInfo {
    one: f64,
    five: f64,
    fifteen: f64,
}

pub async fn get_load_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/loadavg")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let parts: Vec<&str> = content.split_whitespace().collect();
    let load_info = LoadInfo {
        one: parts[0].parse().unwrap(),
        five: parts[1].parse().unwrap(),
        fifteen: parts[2].parse().unwrap(),
    };
    //let json = serde_json::to_string(&load_info).unwrap();
    let json = serde_json::to_string_pretty(&load_info).unwrap();

    Ok(Response::new(Body::from(json)))
}

use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct SwapInfo {
    total: u64,
    free: u64,
}

pub async fn get_swap_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/meminfo")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut total = 0;
    let mut free = 0;

    for line in content.lines() {
        if line.starts_with("SwapTotal:") {
            total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("SwapFree:") {
            free = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
    }

    let swap_info = SwapInfo { total, free };
    //let json = serde_json::to_string(&swap_info).unwrap();
    let json = serde_json::to_string_pretty(&swap_info).unwrap();

    Ok(Response::new(Body::from(json)))
}

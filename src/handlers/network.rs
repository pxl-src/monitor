use hyper::{Body, Response};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize)]
struct NetworkInfo {
    interface: String,
    rx_bytes: u64,
    tx_bytes: u64,
}

pub async fn get_network_info() -> io::Result<Response<Body>> {
    let mut file = File::open("/proc/net/dev")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut network_infos = Vec::new();

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 17 {
            network_infos.push(NetworkInfo {
                interface: parts[0].trim_end_matches(':').to_string(),
                rx_bytes: parts[1].parse().unwrap_or(0),
                tx_bytes: parts[9].parse().unwrap_or(0),
            });
        }
    }

    //let json = serde_json::to_string(&network_infos).unwrap();
    let json = serde_json::to_string_pretty(&network_infos).unwrap();
    Ok(Response::new(Body::from(json)))
}

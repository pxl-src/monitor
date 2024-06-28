use hyper::{Body, Response};
use serde::Serialize;
use std::io;

use crate::handlers::{
    cpu::get_cpu_info, mem::get_mem_info, swap::get_swap_info,
    load::get_load_info, uptime::get_uptime_info, disk::get_disk_info,
    network::get_network_info, process::get_process_info,
};

#[derive(Serialize)]
struct AllInfo {
    cpu: serde_json::Value,
    mem: serde_json::Value,
    swap: serde_json::Value,
    load: serde_json::Value,
    uptime: serde_json::Value,
    disk: serde_json::Value,
    network: serde_json::Value,
    process: serde_json::Value,
}

pub async fn get_all_info() -> io::Result<Response<Body>> {
    let cpu_info = get_cpu_info().await?;
    let mem_info = get_mem_info().await?;
    let swap_info = get_swap_info().await?;
    let load_info = get_load_info().await?;
    let uptime_info = get_uptime_info().await?;
    let disk_info = get_disk_info().await?;
    let network_info = get_network_info().await?;
    let process_info = get_process_info().await?;

    let all_info = AllInfo {
        cpu: serde_json::from_slice(&hyper::body::to_bytes(cpu_info.into_body()).await.unwrap()).unwrap(),
        mem: serde_json::from_slice(&hyper::body::to_bytes(mem_info.into_body()).await.unwrap()).unwrap(),
        swap: serde_json::from_slice(&hyper::body::to_bytes(swap_info.into_body()).await.unwrap()).unwrap(),
        load: serde_json::from_slice(&hyper::body::to_bytes(load_info.into_body()).await.unwrap()).unwrap(),
        uptime: serde_json::from_slice(&hyper::body::to_bytes(uptime_info.into_body()).await.unwrap()).unwrap(),
        disk: serde_json::from_slice(&hyper::body::to_bytes(disk_info.into_body()).await.unwrap()).unwrap(),
        network: serde_json::from_slice(&hyper::body::to_bytes(network_info.into_body()).await.unwrap()).unwrap(),
        process: serde_json::from_slice(&hyper::body::to_bytes(process_info.into_body()).await.unwrap()).unwrap(),
    };

    let json = serde_json::to_string_pretty(&all_info).unwrap();
    Ok(Response::new(Body::from(json)))
}

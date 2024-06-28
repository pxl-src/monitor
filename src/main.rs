use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use clap::Parser;
use daemonize::Daemonize;
use log::{info, error};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::fs::File;
use env_logger;

mod handlers;
use handlers::{cpu::get_cpu_info, mem::get_mem_info, swap::get_swap_info, load::get_load_info, uptime::get_uptime_info, disk::get_disk_info, network::get_network_info, process::get_process_info, all::get_all_info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    #[arg(short, long, default_value_t = 80)]
    port: u16,

    #[arg(long)]
    daemon: bool,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = match req.uri().path() {
        "/cpu" => get_cpu_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/mem" => get_mem_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/swap" => get_swap_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/load" => get_load_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/uptime" => get_uptime_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/disk" => get_disk_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/network" => get_network_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/process" => get_process_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        "/all" => get_all_info().await.unwrap_or_else(|_| Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        _ => Response::builder().status(404).body(Body::from("Not Found")).unwrap(),
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.daemon {
        let stdout = File::create("/tmp/server_monitor.out").unwrap();
        let stderr = File::create("/tmp/server_monitor.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/server_monitor.pid")
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => info!("Server monitoring daemonized, pid file created at /tmp/server_monitor.pid"),
            Err(e) => {
                error!("Error, {}", e);
                return;
            },
        }
    }

    let addr = format!("{}:{}", args.address, args.port).parse::<SocketAddr>().unwrap();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}

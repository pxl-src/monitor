use serde::{Serialize};
use sysinfo::{Networks};

#[derive(Serialize)]
pub struct NetInfo {
    pub interface : String,
    pub total_receive: u64,
    pub total_transmitted: u64,
}

pub fn get_net_info() -> Result<Vec<NetInfo>, Box<dyn std::error::Error>> {
    let mut net_info_vec = Vec::new();
    let networks = Networks::new_with_refreshed_list();

    for (interface_name, data) in &networks {
        let interface = interface_name.to_string();
        let total_receive =  data.total_received() ;
        let total_transmitted = data.total_transmitted() ;

        net_info_vec.push(NetInfo {
            interface,
            total_receive,
            total_transmitted,
        });
    }

    Ok(net_info_vec)
}
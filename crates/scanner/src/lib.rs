use netscan::blocking::PortScanner;
use netscan::setting::{Destination, ScanType};
use network_interface::NetworkInterfaceConfig;
use network_interface::{Addr, NetworkInterface};
use std::net::IpAddr;
use std::time::Duration;

use anyhow::anyhow;
use netscan::result::PortStatus;

pub fn scan(
    device_name: String,
    target: String,
    port_range: Vec<u16>,
) -> Result<(), anyhow::Error> {
    let addr = get_network_interface_address(&device_name)
        .ok_or(anyhow!("Invalid device name {device_name}"))?;
    let mut port_scanner = match PortScanner::new(addr.ip()) {
        Ok(scanner) => scanner,
        Err(e) => panic!("Error creating scanner: {}", e),
    };

    let target_ip: IpAddr = target.as_str().parse::<IpAddr>()?;
    let (start_port, end_port): (u16, u16) = (port_range[0], port_range[1]);
    let destination: Destination =
        Destination::new_with_port_range(target_ip, start_port, end_port);
    port_scanner.add_destination(destination);

    // Set options
    port_scanner.set_scan_type(ScanType::TcpSynScan);
    port_scanner.set_timeout(Duration::from_millis(10000));
    port_scanner.set_wait_time(Duration::from_millis(100));
    //port_scanner.set_send_rate(Duration::from_millis(1));

    // Run scan
    let result = port_scanner.scan();

    println!("Status: {:?}", result.scan_status);
    println!("Results:");
    for (ip, ports) in result.result_map {
        let open_ports = ports
            .iter()
            .filter(|&port| matches!(port.status, PortStatus::Open))
            .collect::<Vec<_>>();
        for port in open_ports {
            println!("{}:{} - {:?}", ip, port.port, port.status);
        }
    }
    println!("Scan Time: {:?}", result.scan_time);

    Ok(())
}

fn get_network_interface_address(name: &String) -> Option<Addr> {
    let network_interfaces = NetworkInterface::show().unwrap();
    network_interfaces
        .iter()
        .filter(|&network_interface| {
            let is_ipv4 = match network_interface.addr {
                None => false,
                Some(ip) => match ip {
                    Addr::V4(_) => true,
                    Addr::V6(_) => false,
                },
            };
            is_ipv4
        })
        .find(|&network_interface| network_interface.name == *name)
        .map(|network_interface| network_interface.addr)
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}

mod scantype;

use netscan::blocking::PortScanner;
use netscan::setting::{Destination};
use network_interface::NetworkInterfaceConfig;
use network_interface::{Addr, NetworkInterface};
use std::str::FromStr;
use std::time::Duration;

use crate::scantype::ScanTypeInput;
use anyhow::anyhow;
use cidr_utils::cidr::IpCidr;
use netscan::result::PortStatus;

pub fn scan(
    device_name: String,
    target_cidr_str: String,
    port_range: Vec<u16>,
    scan_type: String,
) -> Result<(), anyhow::Error> {
    let interface_address = get_network_interface_address(&device_name)
        .ok_or(anyhow!("Invalid device name {device_name}"))?;

    let mut port_scanner = match PortScanner::new(interface_address.ip()) {
        Ok(scanner) => scanner,
        Err(e) => panic!("Error creating scanner: {}", e),
    };

    let cidr = IpCidr::from_str(target_cidr_str)?;

    let (start_port, end_port): (u16, u16) = (port_range[0], port_range[1]);

    for ip_addr in cidr.iter() {
        let destination: Destination =
            Destination::new_with_port_range(ip_addr, start_port, end_port);
        port_scanner.add_destination(destination);
    }

    // Set options
    port_scanner.set_scan_type(ScanTypeInput::from_str(scan_type.as_str())?.convert());
    port_scanner.set_timeout(Duration::from_millis(10000));
    port_scanner.set_wait_time(Duration::from_millis(200));
    port_scanner.set_send_rate(Duration::from_millis(10));

    // Run scan
    let result = port_scanner.scan();

    for (ip, ports) in result.result_map {
        let open_ports = ports
            .iter()
            .filter(|&port| !matches!(port.status, PortStatus::Closed))
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
    fn it_works() {}
}

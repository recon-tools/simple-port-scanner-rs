use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use cidr_utils::cidr::IpCidr;
use netscan::blocking::PortScanner;
use netscan::result::PortStatus;
use netscan::setting::Destination;

use router::get_network_interface_address_by_name;

use crate::router::Router;
use crate::scan_type::ScanTypeInput;

mod router;
mod scan_type;

pub fn scan(
    device_name: String,
    target_cidr_str: String,
    port_range: Vec<u16>,
    scan_type: String,
) -> Result<(), anyhow::Error> {
    let cidr = IpCidr::from_str(target_cidr_str)?;

    let interface_address = if device_name.is_empty() {
        let router = Router::new();
        router.get_network_interface_address(cidr.first_as_ip_addr())?
    } else {
        get_network_interface_address_by_name(&device_name).ok_or(anyhow!(""))?
    };

    let mut port_scanner = match PortScanner::new(interface_address) {
        Ok(scanner) => scanner,
        Err(e) => panic!("Error creating scanner: {}", e),
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use anyhow::anyhow;
use network_interface::NetworkInterfaceConfig;
use network_interface::{Addr, NetworkInterface};

pub(crate) fn get_default_network_interface_address() -> Result<IpAddr, anyhow::Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.connect("128.199.158.128:12345")?; // scanme.sh
    let local_address = socket.local_addr()?;
    let result = match local_address {
        SocketAddr::V4(socket_address) => Ok(Ipv4Addr::from(*socket_address.ip())),
        SocketAddr::V6(_) => Err(anyhow!("IPv6 is not yet supported!")),
    };
    Ok(IpAddr::V4(result?))
}

pub(crate) fn get_network_interface_address_by_name(name: &String) -> Option<IpAddr> {
    let network_interfaces = NetworkInterface::show().unwrap();
    let option_address = network_interfaces
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
        .flatten();

    option_address.map(|a| a.ip())
}

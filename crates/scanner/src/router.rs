use itertools::Itertools;
use std::process::Command;
use std::str;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use anyhow::anyhow;
use cidr_utils::cidr::IpCidr;
use network_interface::NetworkInterfaceConfig;
use network_interface::{Addr, NetworkInterface};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Route {
    default_route: bool,
    network_interface: String,
    destination: String,
    gateway: String,
    flags: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Router {
    ipv4_routes: Vec<Route>,
    ipv6_routes: Vec<Route>,
}

#[cfg(target_os = "macos")]
impl Router {
    pub(crate) fn new() -> Router {
        let output = Command::new("netstat")
            .arg("-nr")
            .output()
            .expect("failed to execute process");

        let text = str::from_utf8(&*output.stdout).unwrap();
        let lines = text
            .split('\n')
            .into_iter()
            .map(|line| String::from(line))
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let mut ipv4_routes: Vec<Route> = vec![];
        let mut ipv6_routes: Vec<Route> = vec![];

        let mut current_routes: &mut Vec<Route> = &mut ipv4_routes;
        for line in lines {
            if line.contains(&String::from("Routing tables")) || line.contains("Destination") {
                continue;
            }
            if line.contains("Internet:") {
                current_routes = &mut ipv4_routes;
                continue;
            }
            if line.contains("Internet6:") {
                current_routes = &mut ipv6_routes;
                continue;
            }
            let parts = line
                .split(" ")
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();
            if let Some((destination, gateway, flags, network_interface)) =
                parts[0..4].into_iter().tuples().next()
            {
                current_routes.push(Route {
                    default_route: destination == &"default",
                    network_interface: String::from(*network_interface),
                    destination: String::from(*destination),
                    gateway: String::from(*gateway),
                    flags: String::from(*flags),
                });
            }
        }

        return Router {
            ipv4_routes,
            ipv6_routes,
        };
    }

    pub(crate) fn get_network_interface_address(
        &self,
        address: IpAddr,
    ) -> Result<IpAddr, anyhow::Error> {
        let mut default_route: Option<&Route> = None;
        for route in &self.ipv4_routes {
            println!("{:?}", route);
            if route.default_route {
                default_route = Some(route);
                continue;
            }

            let cidr = IpCidr::from_str(&route.destination);

            if cidr.is_ok() && cidr?.contains(address) {
                return get_network_interface_address_by_name(&route.network_interface)
                    .ok_or(anyhow!("Interface not found!"));
            }
        }

        if default_route.is_some() {
            return get_network_interface_address_by_name(
                &default_route.unwrap().network_interface,
            )
            .ok_or(anyhow!("Interface not found!"));
        }

        get_default_network_interface_address()
    }
}

pub(crate) fn get_default_network_interface_address() -> Result<IpAddr, anyhow::Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.connect("128.199.158.128:12345")?; // scanme.sh
    let local_address = socket.local_addr()?;
    let result = match local_address {
        SocketAddr::V4(socket_address) => Ok(Ipv4Addr::from(*socket_address.ip())),
        SocketAddr::V6(_) => Err(anyhow!("IPv6 is not yet supported!")),
    };
    println!("{:?}", result);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let router = Router::new();
        println!("{:?}", router);
    }
}

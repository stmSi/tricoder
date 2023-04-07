use crate::{
    common_ports::MOST_COMMON_PORTS_10,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn scan_ports(subdomain: &mut Subdomain) -> &Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS_10
        .par_iter() // Multithreaded
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}

pub fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let addr_str = format!("{}:{}", hostname, port);
    let socket_addresses_result = addr_str.to_socket_addrs();

    let socket_addresses: Vec<SocketAddr> = match socket_addresses_result {
        Ok(sock_addr) => sock_addr.collect(),
        Err(err) => {
            eprintln!("Error while converting socket \"{}\":{}", addr_str, err);
            return Port {
                port,
                is_open: false,
            };
        }
    };

    if socket_addresses.len() == 0 {
        return Port {
            port,
            is_open: false,
        };
    }

    let is_open = match TcpStream::connect_timeout(&socket_addresses[0], timeout) {
        Ok(_) => true,
        Err(_) => false,
    };

    Port { port, is_open }
}

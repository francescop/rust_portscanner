use std::process;
use std::env;
use std::net::{IpAddr, ToSocketAddrs, SocketAddr, TcpStream};
use std::time::Duration;
use std::net::Ipv4Addr;

const MIN_PORT_NUMBER : u16 = 0;
const MAX_PORT_NUMBER : u16 = 65535;
const TIMEOUT_SEC : u64 = 1;
const TIMEOUT_MS : u32 = 800;

enum ResolveHostResult {
    Success,
    Error,
    GetIpError
}

enum ResolvePortResult {
    Success,
    Error
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let host = &args[1];
    let mut scan_port : u16 = 0;

    if args.len() < 2 {
        println!("Must supply a host.");
        process::exit(1);
    }

    if args.len() > 2 {
        match resolve_port(&args[2], &mut scan_port) {
            ResolvePortResult::Success => {},
            ResolvePortResult::Error => {
                println!("Invalid port.");
                process::exit(1);
            }
        }
    }

    let mut open_tcp_ports : Vec<u16> = Vec::with_capacity(65535);
    let mut ip_addr : IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    match resolve_host(host, &mut ip_addr) {
        ResolveHostResult::Success => {},
        ResolveHostResult::Error => {
            println!("Could not resolve hostname.");
            process::exit(1);
        },
        ResolveHostResult::GetIpError => {
            println!("Could not get ip.");
            process::exit(1);
        }
    }

    let timeout = Duration::new(TIMEOUT_SEC, TIMEOUT_MS);

    if scan_port > 0 {
        do_scan(&mut open_tcp_ports, timeout, ip_addr, scan_port);
    } else {
        for port in MIN_PORT_NUMBER..=MAX_PORT_NUMBER {
            do_scan(&mut open_tcp_ports, timeout, ip_addr, port);
        }
    }

    println!("ip: {:?}", ip_addr);
    println!("open tcp ports: {:#?}", open_tcp_ports);
}

fn resolve_host(host: &str, ip_addr: &mut IpAddr) -> ResolveHostResult {
    let mut resolved_ips = match (host, 0).to_socket_addrs() {
        Ok(ips) => ips,
        Err(_) => return ResolveHostResult::Error
    };

    *ip_addr = match resolved_ips.next() {
        Some(socket_addr) => socket_addr.ip(),
        None => return ResolveHostResult::GetIpError
    };

    ResolveHostResult::Success
}

fn resolve_port(port_str: &str, scan_port: &mut u16) -> ResolvePortResult {
    *scan_port = match port_str.parse::<u16>() {
        Ok(port) => {
            port
        },
        Err(_) => return ResolvePortResult::Error
    };

    if *scan_port < MIN_PORT_NUMBER || *scan_port > MAX_PORT_NUMBER {
        return ResolvePortResult::Error
    }

    ResolvePortResult::Success
}

fn do_scan(open_tcp_ports: &mut Vec<u16>, timeout: Duration, ip_addr: IpAddr, port: u16){
    let socket = SocketAddr::new(ip_addr, port);
    if let Ok(_) = TcpStream::connect_timeout(&socket, timeout) {
        open_tcp_ports.push(socket.port())
    } 
}

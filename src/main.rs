use std::process;
use std::env;
use std::net::{IpAddr, ToSocketAddrs, SocketAddr, TcpStream};
use std::time::Duration;
use std::fmt;
use std::error::Error;

const MIN_PORT_NUMBER : u16 = 0;
const MAX_PORT_NUMBER : u16 = 65535;
const TIMEOUT_SEC : u64 = 1;
const TIMEOUT_MS : u32 = 800;

#[derive(Debug)]
enum ResolveHostResult {
    Error,
    GetIpError
}

impl fmt::Display for ResolveHostResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolveHostResult::Error => f.write_str("Could not resolve hostname."),
            ResolveHostResult::GetIpError => f.write_str("Could not get ip."),
        }
    }
}

impl Error for ResolveHostResult {
    fn description(&self) -> &str {
        match *self {
            ResolveHostResult::Error => "Could not resolve hostname.",
            ResolveHostResult::GetIpError => "Could not get ip.",
        }
    }
}

#[derive(Debug)]
enum ResolvePortResult {
    Error,
    OutOfRange
}

impl fmt::Display for ResolvePortResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolvePortResult::Error => f.write_str("Invalid port."),
            ResolvePortResult::OutOfRange => f.write_str("Port out of range."),
        }
    }
}

impl Error for ResolvePortResult {
    fn description(&self) -> &str {
        match *self {
            ResolvePortResult::Error => "Invalid port.",
            ResolvePortResult::OutOfRange => "Port out of range.",
        }
    }
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

        scan_port = match resolve_port(&args[2]) {
                Ok(port) => port,
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
        };
    }

    let mut open_tcp_ports : Vec<u16> = Vec::with_capacity(65535);
    let ip_addr = match resolve_host(host) {
        Ok(ip) => ip,
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    };


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

fn resolve_host(host: &str) -> Result<IpAddr, ResolveHostResult> {
    let mut resolved_ips = match (host, 0).to_socket_addrs() {
        Ok(ips) => ips,
        Err(_) => return Err(ResolveHostResult::Error)
    };

    match resolved_ips.next() {
        Some(socket_addr) => Ok(socket_addr.ip()),
        None => Err(ResolveHostResult::GetIpError)
    }
}

fn resolve_port(port_str: &str) -> Result<u16, ResolvePortResult> {
    match port_str.parse::<u16>() {
        Ok(port) => { 
            if port < MIN_PORT_NUMBER || port > MAX_PORT_NUMBER {
                return Err(ResolvePortResult::OutOfRange)
            }
            return Ok(port)
        }
        Err(_) => return Err(ResolvePortResult::Error)
    };
}

fn do_scan(open_tcp_ports: &mut Vec<u16>, timeout: Duration, ip_addr: IpAddr, port: u16){
    let socket = SocketAddr::new(ip_addr, port);
    if let Ok(_) = TcpStream::connect_timeout(&socket, timeout) {
        open_tcp_ports.push(socket.port())
    }
}

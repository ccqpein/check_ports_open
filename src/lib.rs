use std::fs::File;
use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

// try type alias
type Ports<'a> = Vec<&'a str>;
type Hosts<'a> = Vec<&'a str>;

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("cannot find file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn parse_file(contents: &str) -> Vec<&str> {
    let result = contents
        .split(|c| c == ' ' || c == '\n')
        .filter(|c| *c != "")
        .collect::<Vec<&str>>();
    result
}

pub fn create_socket_addr(hosts: &Hosts, ports: &Ports) -> Vec<SocketAddr> {
    let mut socket_addrs = Vec::new();

    for host in hosts {
        for port in ports {
            if let Ok(ip_addr) = IpAddr::from_str(*host) {
                socket_addrs.push(SocketAddr::new(ip_addr, port.parse::<u16>().unwrap()));
            } else {
                panic!("looks something wrong in host");
            }
        }
    }
    socket_addrs
}

pub fn check_connect_to_host(addr: &SocketAddr) -> bool {
    if let Ok(_) = TcpStream::connect_timeout(addr, Duration::new(3, 0)) {
        println!("Connected to the server! {}", addr);
        true
    } else {
        println!("wrong with {}", addr);
        false
    }
}

pub fn check_connect_to_host_concurrent(hosts: &Hosts, ports: &Ports) {
    let sockets = create_socket_addr(hosts, ports);
    let mut children = vec![];

    for s in sockets {
        children.push(thread::spawn(move || {
            check_connect_to_host(&s);
            thread::sleep(Duration::from_secs(1));
        }))
    }

    for c in children {
        c.join().unwrap();
    }
}

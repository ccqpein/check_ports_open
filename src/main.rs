use std::fs::File;
use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

// try type alias
type Ports = Vec<u16>;
type Hosts<'a> = Vec<&'a str>;

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("cannot find file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn parse_file(filename: &str) {
    let mut contents = read_file(filename);
    // json struct.new() to create json file
}

fn create_socket_addr(hosts: &Hosts, ports: &Ports) -> Vec<SocketAddr> {
    let mut socket_addrs = Vec::new();

    for host in hosts {
        for port in ports {
            if let Ok(ip_addr) = IpAddr::from_str(*host) {
                socket_addrs.push(SocketAddr::new(ip_addr, *port));
            }
        }
    }
    socket_addrs
}

//:= TODO: learn how to use [test]
fn check_connect_to_host(addr: &SocketAddr) -> bool {
    //println!("host is {}, port is {}\n", addr.ip(), addr.port());
    if let Ok(_) = TcpStream::connect(addr) {
        println!("Connected to the server!");
        true
    } else {
        println!("wrong");
        false
    }
}

fn main() {
    //:= TODO: read hosts && ports file
    let hosts = vec!["127.0.0.1", "127.0.0.1"];
    let ports = vec![8080, 8081];

    let sockets = create_socket_addr(&hosts, &ports);

    for socket in sockets {
        check_connect_to_host(&socket);
    }

    //println!("{}", read_file("LICENSE"))
}

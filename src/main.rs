use std::net::{SocketAddr, TcpStream};

fn read_file() {}

fn parse_file() {}

fn check_connect_to_host(addr: &SocketAddr) -> bool {
    if let Ok(_) = TcpStream::connect(addr) {
        println!("Connected to the server!");
        true
    } else {
        println!("wrong");
        false
    }
}

fn main() {
    let test_socket = SocketAddr::from(([127, 0, 0, 1], 8080));

    check_connect_to_host(&test_socket);
}

use std::fs::File;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

// try type alias
type Ports<'a> = Vec<&'a str>;
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

#[macro_use]
extern crate json_api;

struct Test {
    id: i64,
    body: String,
}

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

    println!("{}", read_file("LICENSE"))
}

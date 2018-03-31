use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let a = b"shoudao\n";
    let rec = stream.write(a).unwrap();
    let _ = stream.read(&mut [0; 128]);
    println!("{:?}", rec);
    println!("hello?");
}

fn connect_to_host(addr: &SocketAddr) {
    if let Ok(stream) = TcpStream::connect(addr) {
        println!("Connected to the server!");
        println!("{:?}", stream);
    } else {
        println!("wrong")
    }
}

fn main() {
    let test_socket = SocketAddr::from(([127, 0, 0, 1], 8080));

    connect_to_host(&test_Socket);
    /*
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();
    let a = b"fasong\n";
    let _ = stream.write(a);
    let _ = stream.read(&mut [0; 128]);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(_) => println!("error"),
        }
    }*/
}

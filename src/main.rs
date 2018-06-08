extern crate check_ports_open;

use check_ports_open::*;
use std::thread;
use std::time::Duration;

fn main() {
    let hosts_file = check_ports_open::read_file("hosts");
    let ports_file = check_ports_open::read_file("ports");

    let hosts = check_ports_open::parse_file(&hosts_file);
    let ports = check_ports_open::parse_file(&ports_file);

    //test one by one
    let sockets = check_ports_open::create_socket_addr(&hosts, &ports);
    for socket in sockets {
        check_ports_open::check_connect_to_host(&socket);
        thread::sleep(Duration::from_secs(1));
    }

    println!("done with seq");

    //test concurrency
    check_ports_open::check_connect_to_host_concurrent(&hosts, &ports);
}

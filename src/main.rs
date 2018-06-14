extern crate check_ports_open;

use check_ports_open::*;

fn main() {
    let hosts_file = check_ports_open::read_file("hosts");
    let ports_file = check_ports_open::read_file("ports");

    let hosts = check_ports_open::parse_file(&hosts_file);
    let ports = check_ports_open::parse_file(&ports_file);

    check_ports_open::check_connect_to_host_concurrent(&hosts, &ports);
}

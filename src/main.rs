fn main() {
    use local_ip_address::list_afinet_netifas;

    let network_interfaces = list_afinet_netifas();

    if let Ok(network_interfaces) = network_interfaces {
        for (name, ip) in network_interfaces.iter() {
            println!("{}:\t{:?}", name, ip);
        }
    } else {
        println!("Error getting network interfaces: {:?}", network_interfaces);
    }
}

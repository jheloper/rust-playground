pub fn example_enums() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4 Addr Kind is {:?}", four);
    println!("IPv6 Addr Kind is {:?}", six);
}
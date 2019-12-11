pub fn example_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4 Addr Kind is {:?}", four);
    println!("IPv6 Addr Kind is {:?}", six);

    let home1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback1 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("home ip address is {:?}", home1);
    println!("loopback ip address is {:?}", loopback1);

    let home2 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback2 = IpAddrEnum::V6(String::from("::1"));

    println!("home ip address is {:?}", home2);
    println!("loopback ip address is {:?}", loopback2);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}
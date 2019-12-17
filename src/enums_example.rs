pub fn example_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4 Addr Kind is {:?}", four);
    println!("IPv6 Addr Kind is {:?}", six);

    let home1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback1 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home ip address is {:?}", home1);
    println!("loopback ip address is {:?}", loopback1);

    let home2 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback2 = IpAddrEnum::V6(String::from("::1"));

    println!("home ip address is {:?}", home2);
    println!("loopback ip address is {:?}", loopback2);

    let message1 = Message::Quit;
    let message2 = Message::Move { x: 7, y: 12 };
    let message3 = Message::Write(String::from("hello"));
    let message4 = Message::ChangeColor(15, 10, 30);

    message1.call();
    message2.call();
    message3.call();
    message4.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some number is {:?}", some_number);
    println!("some string is {:?}", some_string);
    println!("absent number is {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // 아래 코드는 i8과 Option<i8>을 더하려고 하기 때문에 컴파일 오류 발생.
    // let sum = x + y;
    
    // Option<T>의 unwrap 메서드를 사용하여 i8 값을 추출.
    let sum = x + y.unwrap();
    println!("x + y is {}", sum);
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

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call method in enum: {:?}", self);
    }
}

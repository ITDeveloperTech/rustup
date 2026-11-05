enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    _kind: IpAddrKind,
    _address: String,
}

enum IpAddrAssoc {
    V4(String),
    V6(String),
}

enum IpAddrWithNum {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {

}
struct Ipv6Addr {

}
enum IpAddrWStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    // Enum Create and Use
    let four_ip = IpAddrKind::V4;
    let six_ip = IpAddrKind::V6;
    route(four_ip);
    route(six_ip);

    // Struct with Enum
    let _home = IpAddr {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };

    // Enum Assoc with String
    let _home = IpAddrAssoc::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrAssoc::V6(String::from("::1"));

    // Enum Assoc with numbers and String
    let _home = IpAddrWithNum::V4(127, 0, 0, 1);
    let _loopback = IpAddrAssoc::V6(String::from("::1"));

    // Use Call fn in impl Message enum
    let m = Message::Write(String::from("Hello, world!"));
    m.call();

    //Option Type: Some<T>, None
    let _some_number = Some(5);
    let _some_char = Some('c');
    let _absent_number: Option<i32> = None;

}

fn route(_ip_kind: IpAddrKind) {}
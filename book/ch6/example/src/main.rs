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
enum IpAddr2 {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quit => write!(f, "Quit"),
            Self::Move { x, y } => f.debug_struct("Move").field("x", x).field("y", y).finish(),
            Self::Write(arg0) => f.debug_tuple("Write").field(arg0).finish(),
            Self::ChangeColor(arg0, arg1, arg2) => f
                .debug_tuple("ChangeColor")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

impl Message {
    fn call(&self) {
        println!("call from Message");
        println!("{:?}", self);
    }
}

fn main() {
    println!("Hello, world!");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", home.kind);
    println!("{:?}", loopback.address);
    println!("{:?}", home2);
    println!("{:?}", loopback2);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Option::Some(5);
    let some_char = Some('e');

}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // named fields
    Write(String),  // a single unnamed field (i.e. tuple)
    ChangeColor(i32, i32, i32),  // multiple unnamed fields (also a tuple)
}

impl Message {
    fn call(&self) {
        // ...
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     address: String::from("::1"),
    //     kind: IpAddrKind::V6,
    // };
    for address in [&home, &loopback] {
        println!("{:#?}", address);
    }
    
}

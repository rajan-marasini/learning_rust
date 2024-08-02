// enum IpAddrKind {
//     V4,
//     V6
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// }
//
// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }

// enum ipaddr {
//     v4(string),
//     v6(string),
// }
//
// fn main() {
//     let home = ipaddr::v4(string::from("127.0.0.1"));
//
//     let loopback = ipaddr::v6(string::from("::1"))
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         println!("hello form call function");
//     }
// }
#[derive(Debug)] //so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    println!("hell world");
}

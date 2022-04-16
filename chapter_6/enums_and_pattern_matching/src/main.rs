enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String    
}

fn main() {
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);

    // Included in scope by default
    // Never need to include in code to use
    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    println!("{}", sum);

    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);

    if let Some(3) = some_value {
        println!("three")
    }
}

fn route(ip_kind: IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    // "Arms" of a match expression
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // If anything else
        _ => None
    }
}
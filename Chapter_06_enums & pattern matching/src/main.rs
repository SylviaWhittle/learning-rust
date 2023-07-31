// Some variables we want to only be certain values
// These values can store values, denoted by the brackets
// Unlike with structs, enum values can have different types in their items
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Create a function that can take any IpAddrKind
fn route(addr: IpAddrKind) {
    println!("Routing address: {:?}", addr);
}

// An enum with lots of different data types
#[derive(Debug)]
enum Message {
    // This has no data associated
    Quit,
    // This has named fields, like a struct does
    Move { x: i32, y: i32 },
    // This includes a single string
    Write(String),
    // Includes 3 i32 values
    ChangeColor(i32, i32, i32),
}

// We can implement methods on enums
impl Message {
    fn call(&self) {
        println!("Message: {:?}", &self);
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
}

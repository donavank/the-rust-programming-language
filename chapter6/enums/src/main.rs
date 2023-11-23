
// IP address is a good example for an enum because there are a limited number of versions
// but we want to treat both versions as an IP address. V4 and V6 are known as variants of the enumeration
// We will also want to store the address as a string. Initially, structs seem like a good idea for this, but
// enums in rust can have data associated with them as shown below.
//
// Each variant can be described with different types. For example, we may show a V4 as 4 u8's instead of a String while
// keeping the V6 String representation
enum IpAddrKind {
    //V4(u8, u8, u8, u8),
    V4(String),
    V6(String),
}

// The standard library has an IpAddr type already defined like so:
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
// Which demonstates the ability to define an enum with any type.

// With this, we can define a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {}

// Here is another example to represent a message. You can see how this would be useful for
// messages in networking. Note that Move has named fields like a struct. 
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// This enum is the same as:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
// But the structs aren't grouped under an enumeration

// Methods can be associated with enums just like they are with structs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
}

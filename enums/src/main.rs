// Enumeration listing the possible kinds of IP addresses
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr { 
    // Enumerations can also take structs as a value
    V4(u8, u8, u8, u8), // Having one variable with multiple traits would not be possible with a struct
    V6(String),
}

// Example to show capabilities of enum
enum Message {
    Quit, // Has no data
    Move {x: i32, y: i32}, // Has named fields
    Write(String), 
    ChangeColor(i32, i32, i32),
}
/*
impl Message {
    fn call(&self) {
        Method Body
    }
}
*/

fn main() {
    
    // Instances of IpAddrKind IP adresses
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    //m.call();  This would be valid if call had a method body

}

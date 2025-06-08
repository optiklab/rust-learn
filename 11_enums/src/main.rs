#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
////////////////////////////////Struct+Enum/////////////////////////////////////
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn struct_and_enum() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    route_ip_addr(home);
    route_ip_addr(loopback);
}
fn route_ip_addr(ip_addr: IpAddr) {
    println!("IP Kind: {:?} IP Address: {:?}", ip_addr.kind, ip_addr.address);
}
//////////////////////////Enum with attached data///////////////////////////////
#[derive(Debug)]
enum IpAddrOptimized {
    V4(String),
    V6(String),
}
fn enum_optimized() {
    let home_optimized = IpAddrOptimized::V4(String::from("127.0.0.1"));
    let loopback_optimized = IpAddrOptimized::V6(String::from("::1"));
    println!("IP Address with Kind: {:?}", home_optimized);
    println!("IP Address with Kind: {:?}", loopback_optimized);
}
//////////////////////////Enum with various attached data///////////////////////
#[derive(Debug)]
enum IpAddrVariants {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn enum_variants() {
    let home_variants = IpAddrVariants::V4(127, 0, 0, 1);
    let loopback_variants = IpAddrVariants::V6(String::from("::1"));
    println!("IP Address v4 as u8 parts: {:?}", home_variants);
    println!("IP Address v6 as string: {:?}", loopback_variants);
}
//////////////////////////Enum with attached struct/////////////////////////////
#[derive(Debug)]
struct Ipv4Addr {
    address1: u8,
    address2: u8,
    address3: u8,
    address4: u8
}
#[derive(Debug)]
struct Ipv6Addr {
    address: String,
}
#[derive(Debug)]
enum IpAddrStructed {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}
//////////////////////////Enum with attached enum///////////////////////////////
#[derive(Debug)]
enum Message {
    Quit,                      // Analog of   struct QuitMessage; // unit struct
    Move { x: i32, y: i32 },   // Analog of
                               // struct MoveMessage {
                               //    x: i32,
                               //    y: i32
                               // }
    Write(String),             // Analog of    struct MessageWrite(String); // tuple struct
    ChangeColor(i32, i32, i32),// Analog of    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    GetAddressKind(IpAddrKind) // Enum value of enum
}
/////////////////////////////Enum with methods//////////////////////////////////
impl Message {
    fn call(&self) {
        println!("Message call: {:?}", self);
    }
}
fn route_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
////////////////////////////////////////////////////////////////////////////////
fn main() {
    let four = IpAddrKind::V4;

    route(four);
    route(IpAddrKind::V6);

    struct_and_enum();
    enum_optimized();
    enum_variants();
    route_message();
}

fn route(ip_kind: IpAddrKind) {
    println!("IP Kind: {:?}", ip_kind);
}
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
//////////////////////////////////Optimized/////////////////////////////////////
#[derive(Debug)]
enum IpAddrOptimized {
        V4(String),
        V6(String),
    }
fn enum_optimized() {
    let home_optimized = IpAddrOptimized::V4(String::from("127.0.0.1"));
    let loopback_optimized = IpAddrOptimized::V6(String::from("::1"));
    route_ip_addr_optimized(home_optimized);
    route_ip_addr_optimized(loopback_optimized);
}
fn route_ip_addr_optimized(ip_addr: IpAddrOptimized) {
    println!("IP Address with Kind: {:?}", ip_addr);
}
////////////////////////////////////////////////////////////////////////////////
fn main() {
    let four = IpAddrKind::V4;

    route(four);
    route(IpAddrKind::V6);

    struct_and_enum();
    enum_optimized();
}

fn route(ip_kind: IpAddrKind) {
    println!("IP Kind: {:?}", ip_kind);
}
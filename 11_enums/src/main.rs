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

////////////////////////////////////////////////////////////////////////////////
//////////////////////////Enum with attached enum///////////////////////////////
////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////
/////////////////////////////Enum with methods//////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
//////////////////////Enum with various types of data///////////////////////////
////////////////////////////////////////////////////////////////////////////////
struct Point {
    x: u64,
    y: u64,
}

enum MessageConstruct {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: MessageConstruct) {

        match message {
            MessageConstruct::Resize { width, height } => {
                self.width = width;
                self.height = height;
            },
            MessageConstruct::Move(point) => self.position = point,
            MessageConstruct::Echo(echo) => self.message = echo,
            MessageConstruct::ChangeColor(r, g, b) => self.color = (r, g, b),
            MessageConstruct::Quit => self.quit = true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(MessageConstruct::Resize {
            width: 10,
            height: 30,
        });
        state.process(MessageConstruct::Move(Point { x: 10, y: 15 }));
        state.process(MessageConstruct::Echo(String::from("Hello world!")));
        state.process(MessageConstruct::ChangeColor(255, 0, 255));
        state.process(MessageConstruct::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}

enum IpAddrKind {   // standard enum
    V4,
    V6,
}

enum IpAddr {       // enum with associated values of type string (can be different for different enum values)
    V4(String),     
    V6(String),
}

/*struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}*/

enum Message {      // enum with named values
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} 

impl Message {
    fn call(&self) {
        // method call on enum value
    }
}



fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    /*let localhost = IpAddr { 
        kind: ipv4,
        addr: String::from("127.0.0.1"),
    }*/

    let localhost = IpAddr:: V4("127.0.0.1");

    let msg = Message::Quit;
    msg.call();     // calls method with enum value+

    
}

fn route(ip_kind: IpAddrKind) { }
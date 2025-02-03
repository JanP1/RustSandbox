use enums::{enums::Message, enums::IpAddrKind};

fn main() {

    let _four = IpAddrKind::V4(String::from("127.0.0.1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

}

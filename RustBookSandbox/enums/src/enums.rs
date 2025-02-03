pub enum IpAddrKind {
    V4(String), 
    V6(String),
}



pub enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    pub fn call(&self) {
        println!("Method on enum Message")
    }
    
}

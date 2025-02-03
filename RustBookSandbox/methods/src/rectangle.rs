#![allow(dead_code)]


#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }


    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width &&
            self.height > other.height
    }
}

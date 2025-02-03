#![allow(dead_code)]

pub struct Order {
    pub item: String, 
}


impl Order {
   pub fn new(item: String) -> Self {
       Self{item}
   }
}


